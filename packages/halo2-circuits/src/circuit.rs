use halo2_base::{
    gates::{circuit::builder::BaseCircuitBuilder, GateInstructions, RangeInstructions},
    halo2_proofs::{arithmetic::Field, halo2curves::bn256::Fr},
    poseidon::hasher::{spec::OptimizedPoseidonSpec, PoseidonHasher},
    AssignedValue, Context,
};
use halo2_ecc::bigint::OverflowInteger;
use halo2_rsa::{
    AssignedBigUint, AssignedRSAPubE, AssignedRSAPublicKey, BigUintConfig, BigUintInstructions, Fresh, RSAConfig,
    RSAInstructions, RSAPubE, RSAPublicKey, RSASignature,
};
use num_bigint::BigUint;
use num_traits::{identities::One, ToPrimitive};

#[derive(Debug, Clone)]
pub struct PublicInput {
    // 2048 bits
    pub nation_pubkey: BigUint,
}

#[derive(Debug, Clone)]
pub struct PrivateInput {
    pub tbs_cert: BigUint,
    // 2048 bits
    pub nation_sig: BigUint,
    pub password: Fr,
}

// LOOKUP_BITS must be divisible by 64. because halo2-rsa uses limbs that are 64 bits wide each.
const LOOKUP_BITS: usize = 16;
const RSA_KEY_SIZE: usize = 2048;
const PUBKEY_BEGINS: usize = 2216;
const E: usize = 65537;
// The reason we chose 64 here is because
// a) 2048, the width of RSA keys, must be able to divide limb_bits https://github.com/zkemail/halo2-rsa/blob/main/src/big_uint/chip.rs#L49C49-L49C49
// b) limb_bits * 2 must be smaller than 253 the order of the field https://github.com/axiom-crypto/halo2-lib/blob/dd21d6c2ddb1c6cb5ef78f20d68a6c9682353698/halo2-ecc/src/bigint/mul_no_carry.rs#L8
// the maximum value that satisfies the both is 64
const LIMB_BITS: usize = 32;

// halo2_rsa's mul_mod constraints are too heavy for just taking a bit slice.
// here I write optimized constraints that constraints the same thing.
fn slice_biguint(
    ctx: &mut Context<Fr>,
    biguint_chip: &BigUintConfig<Fr>,
    src: AssignedBigUint<Fr, Fresh>,
    since: usize,
    until: usize,
) -> AssignedBigUint<Fr, Fresh> {
    assert!(0 < src.num_limbs());
    assert!(since < until);

    // I want an auditor to look at this function carefully
    //          since:        |
    //          until:                         |
    //            src: |----|----|----|----|----|----|
    //           dest:        |----|----|----|-|
    // beginning_part:      |-|  |-|  |-|  |-|
    //    ending_part:        |--| |--| |--| |-|
    //     final_part:                         ||

    let beginning_part_width = since % biguint_chip.limb_bits();
    let ending_part_base = ctx.load_constant(Fr::from(2).pow_vartime([beginning_part_width as u64]));
    let ending_part_width = biguint_chip.limb_bits() - beginning_part_width;
    let beginning_part_base = ctx.load_constant(Fr::from(2).pow_vartime([ending_part_width as u64]));
    let final_part_offset = until % biguint_chip.limb_bits();
    let final_part_base = ctx.load_constant(Fr::from(2).pow_vartime([final_part_offset as u64]));
    let mut parts: Vec<AssignedValue<Fr>> = Vec::new();

    // assign witnesses
    let mut read_bits = since / biguint_chip.limb_bits() * biguint_chip.limb_bits();
    while read_bits < until {
        let src_limb = (src.value() >> read_bits) % (BigUint::one() << biguint_chip.limb_bits());

        let beginning_part_max_width = until - read_bits;
        let beginning_part_width = beginning_part_width.min(beginning_part_max_width);
        let beginning_part = src_limb.clone() % (BigUint::one() << beginning_part_width);
        parts.push(ctx.load_witness(Fr::from(beginning_part.to_u64().unwrap())));

        let ending_part_max_width = until - read_bits - beginning_part_width;
        let ending_part_width = ending_part_width.min(ending_part_max_width);
        let ending_part = (src_limb.clone() >> beginning_part_width) % (BigUint::one() << ending_part_width);
        parts.push(ctx.load_witness(Fr::from(ending_part.to_u64().unwrap())));

        if until < read_bits + biguint_chip.limb_bits() {
            let final_part = src_limb.clone() >> final_part_offset;
            parts.push(ctx.load_witness(Fr::from(final_part.to_u64().unwrap())));
        }

        read_bits += biguint_chip.limb_bits();
    }

    // range check the first beginning part
    // it does not overlap with dest
    // thus the range check for dest does not imply a range check for the first beginning part
    let width: usize = (until - since) as usize;
    biguint_chip.range().range_check(ctx, parts[0], beginning_part_width.min(width));

    let chunks = parts.chunks_exact(2);
    let final_part = if let &[final_part] = chunks.remainder() { final_part } else { ctx.load_zero() };

    // constrain against src
    for (i, pair) in chunks.enumerate() {
        let is_last = i + 1 == parts.len() / 2;
        let final_part = if is_last { final_part } else { ctx.load_zero() };

        if let &[beginning_part, ending_part] = pair {
            let src_limb = biguint_chip.gate().mul_add(ctx, ending_part_base, ending_part, beginning_part);
            let src_limb = biguint_chip.gate().mul_add(ctx, final_part_base, final_part, src_limb);

            ctx.constrain_equal(&src.limbs()[since / biguint_chip.limb_bits() + i], &src_limb);
        } else {
            unreachable!();
        }
    }

    // constrain against dest
    let dest_limbs: Vec<AssignedValue<Fr>> = parts[1..parts.len() - 1]
        .chunks_exact(2)
        .map(|pair| {
            if let &[ending_part, beginning_part] = pair {
                let dest_limb = biguint_chip.gate().mul_add(ctx, beginning_part_base, beginning_part, ending_part);
                biguint_chip.range().range_check(ctx, dest_limb, biguint_chip.limb_bits());
                dest_limb
            } else {
                unreachable!();
            }
        })
        .collect();
    let dest_in_rust = (src.value() >> since) % (BigUint::one() << width);
    let dest_in_circuit =
        AssignedBigUint::new(OverflowInteger::new(dest_limbs, biguint_chip.limb_bits()), dest_in_rust);

    dest_in_circuit
}

pub fn proof_of_japanese_residence(public: PublicInput, private: PrivateInput) -> BaseCircuitBuilder<Fr> {
    // TODO: Choose this k
    let k = 17;
    let exp_bits = 5; // UNUSED

    let mut builder = BaseCircuitBuilder::new(false);
    builder.set_k(k);
    builder.set_lookup_bits(LOOKUP_BITS);
    builder.set_instance_columns(1);

    let range_chip = builder.range_chip();
    let ctx = builder.main(0);
    let biguint_chip: BigUintConfig<Fr> = BigUintConfig::construct(range_chip, LIMB_BITS);
    let rsa_chip = RSAConfig::construct(biguint_chip, RSA_KEY_SIZE, exp_bits);
    let mut poseidon = PoseidonHasher::new(OptimizedPoseidonSpec::<Fr, 3, 2>::new::<8, 57, 0>());
    poseidon.initialize_consts(ctx, rsa_chip.gate());

    // load public inputs
    let nation_pubkey =
        rsa_chip.assign_public_key(ctx, RSAPublicKey::new(public.nation_pubkey, RSAPubE::Fix(E.into()))).unwrap();

    // load private inputs
    let tbs_cert = rsa_chip.biguint_config().assign_integer(ctx, &private.tbs_cert, 1 << 15).unwrap();
    let nation_sig = rsa_chip.assign_signature(ctx, RSASignature::new(private.nation_sig)).unwrap();
    let password = ctx.load_witness(private.password);

    // extract citizen's public key from the tbs certificate
    let n = slice_biguint(ctx, &rsa_chip.biguint_config(), tbs_cert, PUBKEY_BEGINS, PUBKEY_BEGINS + RSA_KEY_SIZE);
    let citizen_pubkey = AssignedRSAPublicKey::new(n.clone(), AssignedRSAPubE::Fix(E.into()));

    let identity_commitment_preimage: Vec<AssignedValue<Fr>> =
        n.limbs().into_iter().copied().chain(std::iter::once(password)).collect();
    let identity_commitment = poseidon.hash_fix_len_array(ctx, rsa_chip.gate(), &identity_commitment_preimage);

    builder.assigned_instances[0].push(identity_commitment);
    let circuit_params = builder.calculate_params(None);
    builder.use_params(circuit_params)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::read_citizen_cert;
    use halo2_base::utils::testing::base_test;

    // TODO: Write tests for failure cases
    #[test]
    fn extract_citizen_pubkey() {
        let (_, tbs_cert, expected_citizen_pubkey) = read_citizen_cert("certs/myna_cert.pem");
        base_test().k(LOOKUP_BITS as u32).bench_builder((), (), |pool, range_chip, _| {
            let biguint_chip: BigUintConfig<Fr> = BigUintConfig::construct(range_chip.clone(), LIMB_BITS);
            let tbs_cert = biguint_chip.assign_integer(pool.main(), &tbs_cert, 1 << 15).unwrap();
            let expected = biguint_chip.assign_constant(pool.main(), expected_citizen_pubkey.clone()).unwrap();
            let result =
                slice_biguint(pool.main(), &biguint_chip, tbs_cert, PUBKEY_BEGINS, PUBKEY_BEGINS + RSA_KEY_SIZE);
            let is_ok = biguint_chip.is_equal_fresh(pool.main(), &result, &expected).unwrap();
            let one = pool.main().load_constant(Fr::one());
            pool.main().constrain_equal(&is_ok, &one);
        });
    }
}
