use halo2_base::{
    gates::{circuit::builder::BaseCircuitBuilder, GateInstructions, RangeChip, RangeInstructions},
    halo2_proofs::{arithmetic::Field, halo2curves::bn256::Fr},
    poseidon::hasher::{spec::OptimizedPoseidonSpec, PoseidonHasher},
    AssignedValue, Context, QuantumCell,
};
use halo2_ecc::bigint::OverflowInteger;
use halo2_rsa::{
    AssignedBigUint, BigUintConfig, BigUintInstructions, Fresh, RSAConfig, RSAInstructions, RSAPubE, RSAPublicKey,
    RSASignature,
};
use halo2_sha256_unoptimized::Sha256Chip;
use num_bigint::BigUint;

#[derive(Debug, Clone)]
pub struct PublicInput {
    // 2048 bits
    pub nation_pubkey: BigUint,
}

#[derive(Debug, Clone)]
pub struct PrivateInput {
    pub tbs_cert: Vec<u8>,
    // 2048 bits
    pub nation_sig: BigUint,
    pub password: Fr,
}

// halo2-sha256-unoptimized takes inputs byte by byte so I guess 8 is optimimal
const LOOKUP_BITS: usize = 8;
const RSA_KEY_SIZE: usize = 2048;
const PUBKEY_BEGINS: usize = 2216;
const E: usize = 65537;
const LIMB_BITS: usize = 64;
const SHA256_BLOCK_BITS: usize = 512;
const MAX_TBS_CERT_BITS: usize = 1 << 15;
const SHA256_INPUT_BLOCKS: usize = MAX_TBS_CERT_BITS / SHA256_BLOCK_BITS; // the remainder must be 0

pub fn bytes_to_biguint(
    ctx: &mut Context<Fr>,
    biguint_chip: BigUintConfig<Fr>,
    src: &[AssignedValue<Fr>],
) -> AssignedBigUint<Fr, Fresh> {
    let num_bases = (biguint_chip.limb_bits() / 8) as u64;
    let bases: Vec<QuantumCell<Fr>> =
        (0..num_bases).map(|i| QuantumCell::Constant(Fr::from(2).pow_vartime([i * 8]))).collect();
    let dest_limbs = src
        .chunks(biguint_chip.limb_bits() / 8)
        .map(|bytes_in_limb| {
            let dest_limb = biguint_chip.gate().inner_product(ctx, bytes_in_limb.to_vec(), bases.clone());
            biguint_chip.range().range_check(ctx, dest_limb, biguint_chip.limb_bits());
            dest_limb
        })
        .collect();

    let bytes_in_rust: Vec<u8> = src.iter().map(|byte| byte.value().to_bytes()[0].clone()).collect();
    let dest_in_rust = BigUint::from_bytes_le(&bytes_in_rust);
    AssignedBigUint::new(OverflowInteger::new(dest_limbs, biguint_chip.limb_bits()), dest_in_rust)
}

pub fn bytes_to_64s(
    ctx: &mut Context<Fr>,
    range_chip: RangeChip<Fr>,
    src: &[AssignedValue<Fr>],
) -> Vec<AssignedValue<Fr>> {
    let biguint_chip = BigUintConfig::construct(range_chip, 64);
    bytes_to_biguint(ctx, biguint_chip, src).limbs().to_vec()
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
    let biguint_chip: BigUintConfig<Fr> = BigUintConfig::construct(range_chip.clone(), LIMB_BITS);
    let rsa_chip = RSAConfig::construct(biguint_chip, RSA_KEY_SIZE, exp_bits);
    let mut sha256_chip =
        Sha256Chip::construct(vec![SHA256_INPUT_BLOCKS * SHA256_BLOCK_BITS / 8], range_chip.clone(), true);
    // let mut poseidon = PoseidonHasher::new(OptimizedPoseidonSpec::<Fr, 3, 2>::new::<8, 57, 0>());
    // poseidon.initialize_consts(ctx, rsa_chip.gate());

    // load public inputs
    // let nation_pubkey =
    //     rsa_chip.assign_public_key(ctx, RSAPublicKey::new(public.nation_pubkey, RSAPubE::Fix(E.into()))).unwrap();

    // load private inputs
    // let nation_sig = rsa_chip.assign_signature(ctx, RSASignature::new(private.nation_sig)).unwrap();
    // let password = ctx.load_witness(private.password);

    // extract citizen's public key from the tbs certificate
    // let n = bytes_to_biguint(
    //     ctx,
    //     rsa_chip.biguint_config().clone(),
    //     &sha256ed.input_bytes[PUBKEY_BEGINS / 8..PUBKEY_BEGINS / 8 + RSA_KEY_SIZE / 8],
    // );
    // let citizen_pubkey = AssignedRSAPublicKey::new(n.clone(), AssignedRSAPubE::Fix(E.into()));

    let sha256ed = sha256_chip.digest(ctx, &private.tbs_cert, None).unwrap();
    // let identity_commitment_preimage: Vec<AssignedValue<Fr>> = sha256ed.input_bytes
    //     [PUBKEY_BEGINS / 8..(PUBKEY_BEGINS + RSA_KEY_SIZE) / 8]
    //     .iter()
    //     .copied()
    //     .chain(std::iter::once(password))
    //     .collect();
    // let identity_commitment = poseidon.hash_fix_len_array(ctx, rsa_chip.gate(), &identity_commitment_preimage);

    // let sha256ed_64s = bytes_to_64s(ctx, range_chip.clone(), &sha256ed.output_bytes);
    // let is_nation_sig_valid =
    //     rsa_chip.verify_pkcs1v15_signature(ctx, &nation_pubkey, &sha256ed_64s, &nation_sig).unwrap();
    // rsa_chip.biguint_config().gate().assert_is_const(ctx, &is_nation_sig_valid, &Fr::one());

    // builder.assigned_instances[0].push(identity_commitment);
    // builder.assigned_instances[0].extend(nation_pubkey.n.limbs());
    builder.assigned_instances[0].extend(sha256ed.output_bytes);
    let circuit_params = builder.calculate_params(None);
    builder.use_params(circuit_params)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::read_citizen_cert;
    use halo2_base::{halo2_proofs::halo2curves::ff::PrimeField, utils::testing::base_test};
    use num_traits::cast::ToPrimitive;

    // TODO: Write tests for failure cases
    // #[test]
    // fn extract_citizen_pubkey() {
    //     let (_, tbs_cert, expected_citizen_pubkey) = read_citizen_cert("certs/myna_cert.pem");
    //     base_test().k(LOOKUP_BITS as u32).bench_builder((), (), |pool, range_chip, _| {
    //         let biguint_chip: BigUintConfig<Fr> = BigUintConfig::construct(range_chip.clone(), LIMB_BITS);
    //         let tbs_cert = biguint_chip.assign_integer(pool.main(), &tbs_cert, 1 << 15).unwrap();
    //         let expected = biguint_chip.assign_constant(pool.main(), expected_citizen_pubkey.clone()).unwrap();
    //         let result =
    //             slice_biguint(pool.main(), &biguint_chip, tbs_cert, PUBKEY_BEGINS, PUBKEY_BEGINS + RSA_KEY_SIZE);
    //         let is_ok = biguint_chip.is_equal_fresh(pool.main(), &result, &expected).unwrap();
    //         let one = pool.main().load_constant(Fr::one());
    //         pool.main().constrain_equal(&is_ok, &one);
    //     });
    // }

    #[test]
    fn aaa() {
        let two_pow_16 = Fr::from_raw([1 << 16, 0, 0, 0]);
        let mut test_subject = Fr::from_raw([0, 0, 0, 1 << 46]);
        while test_subject != Fr::zero() {
            for j in 1..16 {
                let k = test_subject * Fr::from_raw([1 << j, 0, 0, 0]);
                if two_pow_16 >= k {
                    unreachable!("i:{:?},j:{:?},k:{:?}", test_subject.to_repr(), j, k.to_repr());
                }
            }

            test_subject += Fr::one();
        }
    }
}
