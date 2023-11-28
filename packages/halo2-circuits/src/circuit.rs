use crate::helpers::{read_citizen_cert, read_nation_cert};
use halo2_base::{
    gates::{
        circuit::{builder::BaseCircuitBuilder, BaseCircuitParams, BaseConfig},
        GateInstructions, RangeChip, RangeInstructions,
    },
    halo2_proofs::{
        arithmetic::Field,
        circuit::{Layouter, SimpleFloorPlanner},
        halo2curves::bn256::Fr,
        plonk::{Assignment, Circuit, ConstraintSystem, Error},
    },
    poseidon::hasher::{spec::OptimizedPoseidonSpec, PoseidonHasher},
    utils::halo2::Halo2AssignedCell,
    AssignedValue, Context, QuantumCell,
};
use halo2_ecc::bigint::OverflowInteger;
use halo2_rsa::{
    AssignedBigUint, BigUintConfig, BigUintInstructions, Fresh, RSAConfig, RSAInstructions, RSAPubE, RSAPublicKey,
    RSASignature,
};
use halo2_sha256_unoptimized::Sha256Chip;
use num_bigint::BigUint;
use num_traits::One;
use pse_poseidon::Poseidon;
use std::{cmp::Ordering, path::PathBuf};
use zkevm_hashes::sha256::vanilla::{columns::Sha256CircuitConfig, param::NUM_WORDS_TO_ABSORB};

#[derive(Debug, Clone)]
pub struct PublicInput {
    // 2048 bits
    pub nation_pubkey: BigUint,
    // little endian
    pub sha256: [Fr; 2],
}

#[derive(Debug, Clone)]
pub struct PrivateInput {
    // 2048 bits
    pub nation_sig: BigUint,
    pub password: Fr,
}

const RSA_KEY_SIZE: usize = 2048;
const PUBKEY_BEGINS: usize = 2216;
const E: usize = 65537;
pub const K: usize = 23;
pub const LOOKUP_BITS: usize = K - 1;
const LIMB_BITS: usize = 64;
const SHA256_BLOCK_BITS: usize = 512;
const TBS_CERT_MAX_BITS: usize = 2048 * 8;

pub fn biguint_to_fr(src: BigUint) -> Fr {
    let mut buf = [0; 32];
    buf[0..src.to_bytes_le().len()].copy_from_slice(&src.to_bytes_le());
    Fr::from_bytes(&buf).expect("a BigUint was too big to fit in a Fr")
}

pub fn slice_bits(
    ctx: &mut Context<Fr>,
    range_chip: &RangeChip<Fr>,
    src_limbs: &[AssignedValue<Fr>],
    src_limb_width: usize,
    dest_limb_width: usize,
    since: usize,
    until: usize,
) -> Vec<AssignedValue<Fr>> {
    assert!(0 < src_limb_width);
    assert!(0 < dest_limb_width);
    assert!(254 > src_limb_width);

    assert!(254 > dest_limb_width);
    let zero_part = (ctx.load_zero(), 0);
    let mut parts: Vec<(AssignedValue<Fr>, usize)> = Vec::new();

    // split src into parts
    let mut read_bits = since;
    while read_bits < until {
        let read_limbs = read_bits / src_limb_width;
        let part_offset = read_bits % src_limb_width;
        let part_width = (src_limb_width - read_bits % src_limb_width)
            .min(dest_limb_width - (read_bits - since) % dest_limb_width)
            .min(until - read_bits);
        let part_biguint = (BigUint::from_bytes_le(&src_limbs[read_limbs].value().to_bytes()) >> part_offset)
            % (BigUint::one() << part_width);
        let part_witness = ctx.load_witness(biguint_to_fr(part_biguint));
        range_chip.range_check(ctx, part_witness, part_width);
        parts.push((part_witness, part_width));
        read_bits += part_width;
    }

    // constrain against dest
    let mut dest_parts: Vec<(AssignedValue<Fr>, usize)> = Vec::new();
    let mut dest_limbs: Vec<AssignedValue<Fr>> = Vec::new();
    for (i, part) in parts.iter().cloned().enumerate() {
        dest_parts.push(part);

        if dest_parts.iter().map(|(_, part_width)| *part_width).sum::<usize>() == dest_limb_width
            || i == parts.len() - 1
        {
            let dest_limb = range_chip.gate().inner_product(
                ctx,
                dest_parts.iter().map(|(part_witness, _)| part_witness.clone()),
                std::iter::once(&zero_part)
                    .chain(dest_parts.iter())
                    .scan(BigUint::one(), |acc, (_, part_width)| {
                        *acc <<= *part_width;
                        Some(acc.clone())
                    })
                    .map(|part_base| QuantumCell::Constant(biguint_to_fr(part_base))),
            );
            dest_limbs.push(dest_limb);
            dest_parts.clear();
        }
    }

    // constrain against src
    let first_part_width = since % src_limb_width;
    if 0 < first_part_width {
        let first_part_witness = ctx.load_witness(biguint_to_fr(
            BigUint::from_bytes_le(&src_limbs[since / src_limb_width].value().to_bytes())
                % (BigUint::one() << first_part_width),
        ));
        parts.insert(0, (first_part_witness, first_part_width));
    } else {
        parts.insert(0, zero_part.clone());
    };

    let last_part_offset = until % src_limb_width;
    if 0 < last_part_offset {
        let last_part_witness = ctx.load_witness(biguint_to_fr(
            BigUint::from_bytes_le(&src_limbs[until / src_limb_width].value().to_bytes()) >> last_part_offset,
        ));
        let last_part_width = src_limb_width - last_part_offset;
        parts.push((last_part_witness, last_part_width));
    } else {
        parts.push(zero_part.clone());
    }

    let mut src_parts: Vec<(AssignedValue<Fr>, usize)> = Vec::new();
    let mut read_limbs = since / src_limb_width;
    for part in parts {
        src_parts.push(part);

        if src_parts.iter().map(|(_, part_width)| *part_width).sum::<usize>() == src_limb_width {
            let src_limb = range_chip.gate().inner_product(
                ctx,
                src_parts.iter().map(|(part_witness, _)| part_witness.clone()),
                std::iter::once(&zero_part)
                    .chain(src_parts.iter())
                    .scan(BigUint::one(), |acc, (_, part_width)| {
                        *acc <<= *part_width;
                        Some(acc.clone())
                    })
                    .map(|part_base| QuantumCell::Constant(biguint_to_fr(part_base))),
            );

            ctx.constrain_equal(&src_limbs[read_limbs], &src_limb);
            src_parts.clear();
            read_limbs += 1;
        }
    }

    dest_limbs
}

#[derive(Debug, Clone)]
pub struct Config {
    halo2base: BaseConfig<Fr>,
    sha256: Sha256CircuitConfig<Fr>,
}

#[derive(Debug, Clone)]
pub struct ProofOfJapaneseResidence {
    pub tbs_cert: Vec<u8>,
    // 2048 bits
    pub nation_sig: BigUint,
    // 2048 bits
    pub nation_pubkey: BigUint,
    pub user_secret: Fr,
    pub citizen_pubkey: BigUint,
}

impl Circuit<Fr> for ProofOfJapaneseResidence {
    type Config = Config;
    type Params = BaseCircuitParams;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        unreachable!()
    }

    fn params(&self) -> Self::Params {
        Self::Params {
            k: K,
            num_advice_per_phase: vec![1],
            num_fixed: 1,
            num_lookup_advice_per_phase: vec![1, 0, 0],
            lookup_bits: Some(LOOKUP_BITS),
            num_instance_columns: 1,
        }
    }

    fn configure_with_params(meta: &mut ConstraintSystem<Fr>, params: BaseCircuitParams) -> Self::Config {
        Self::Config { halo2base: BaseConfig::configure(meta, params), sha256: Sha256CircuitConfig::new(meta) }
    }

    fn configure(_: &mut ConstraintSystem<Fr>) -> Self::Config {
        unreachable!("halo2-base says I must not call configure");
    }

    fn synthesize(&self, config: Self::Config, mut layouter: impl Layouter<Fr>) -> Result<(), Error> {
        // let mut assigned_blocks = Vec::new();
        // layouter.assign_region(
        //     || "SHA256",
        //     |mut region| {
        //         assigned_blocks = config.sha256.multi_sha256(
        //             &mut region,
        //             vec![self.tbs_cert.clone()],
        //             Some(TBS_CERT_MAX_BITS / SHA256_BLOCK_BITS),
        //         );
        //         Ok(())
        //     },
        // )?;

        // let mut final_block = None;
        // for (i, block) in assigned_blocks.iter().enumerate() {
        //     block.is_final().value().map(|is_final| {
        //         if Fr::zero() < is_final.evaluate() {
        //             final_block = Some(block);
        //         }
        //     });

        //     // if let Some(_) = final_block {
        //     //     dbg!(i);
        //     //     break;
        //     // }
        // }
        // let final_block = final_block.expect("zkevm-hashes failed to generate a SHA256 hash");

        // The final block appears in [20] because of the length of certs/myna_cert.pem.
        // TODO: Support pem with dynamic length.
        // let sha256out = &assigned_blocks[20].output();

        let mut halo2base = BaseCircuitBuilder::new(false).use_params(self.params());
        // let (sha256lo, sha256hi) = {
        //     let mut lock = halo2base.core_mut().copy_manager.lock().unwrap();
        //     (lock.load_external_assigned(sha256out.lo()), lock.load_external_assigned(sha256out.hi()))
        // };
        // let tbs_cert_32s: Vec<AssignedValue<Fr>> = {
        //     let mut lock = halo2base.core_mut().copy_manager.lock().unwrap();
        //     assigned_blocks
        //         .iter()
        //         .flat_map(|assigned_block| {
        //             assigned_block.word_values().clone().map(|cell| lock.load_external_assigned(cell))
        //         })
        //         .collect()
        // };

        let range_chip = halo2base.range_chip();
        let ctx = halo2base.main(0);

        let biguint_chip: BigUintConfig<Fr> = BigUintConfig::construct(range_chip.clone(), LIMB_BITS);
        let rsa_chip = RSAConfig::construct(biguint_chip, RSA_KEY_SIZE, 5);
        let mut sha256_chip: Sha256Chip<Fr> =
            Sha256Chip::construct(vec![TBS_CERT_MAX_BITS / 8], range_chip.clone(), true);
        let mut poseidon = PoseidonHasher::new(OptimizedPoseidonSpec::<Fr, 3, 2>::new::<8, 57, 0>());
        poseidon.initialize_consts(ctx, rsa_chip.gate());

        // load public inputs
        let nation_pubkey = rsa_chip
            .assign_public_key(ctx, RSAPublicKey::new(self.nation_pubkey.clone(), RSAPubE::Fix(E.into())))
            .unwrap();

        // load private inputs
        let nation_sig = rsa_chip.assign_signature(ctx, RSASignature::new(self.nation_sig.clone())).unwrap();
        let user_secret = ctx.load_witness(self.user_secret);

        let sha256ed = sha256_chip.digest(ctx, &self.tbs_cert, None).unwrap();
        let sha256ed_64s = slice_bits(
            ctx,
            rsa_chip.range(),
            &sha256ed.output_bytes.iter().rev().copied().collect::<Vec<_>>(),
            8,
            64,
            0,
            256,
        );
        let is_nation_sig_valid =
            rsa_chip.verify_pkcs1v15_signature(ctx, &nation_pubkey, &sha256ed_64s, &nation_sig).unwrap();
        rsa_chip.biguint_config().gate().assert_is_const(ctx, &is_nation_sig_valid, &Fr::one());

        let mut identity_commitment_preimage = vec![user_secret];
        let citizen_pubkey = slice_bits(
            ctx,
            rsa_chip.range(),
            &sha256ed.input_bytes,
            8,
            253,
            PUBKEY_BEGINS,
            PUBKEY_BEGINS + RSA_KEY_SIZE,
        );
        identity_commitment_preimage.extend(citizen_pubkey);
        let identity_commitment = poseidon.hash_fix_len_array(ctx, rsa_chip.gate(), &identity_commitment_preimage);

        halo2base.assigned_instances[0].extend(nation_pubkey.n.limbs().to_vec());
        halo2base.assigned_instances[0].push(identity_commitment);
        halo2base.synthesize(config.halo2base, layouter).unwrap();

        Ok(())
    }
}

impl ProofOfJapaneseResidence {
    pub fn new(nation_cert_path: PathBuf, citizen_cert_path: PathBuf, user_secret: Fr) -> Self {
        let nation_pubkey = read_nation_cert(nation_cert_path.to_str().unwrap());
        let (nation_sig, tbs_cert, citizen_pubkey) = read_citizen_cert(citizen_cert_path.to_str().unwrap());

        Self { tbs_cert: tbs_cert.to_bytes_le(), user_secret, nation_sig, nation_pubkey, citizen_pubkey }
    }

    pub fn instance_column(&self) -> Vec<Fr> {
        let mut instance_column: Vec<Fr> = self.nation_pubkey.iter_u64_digits().map(Fr::from).collect();
        let mut hasher = Poseidon::<Fr, 3, 2>::new(8, 57);
        let mut preimage = vec![self.user_secret];

        for i in 0..=(2048 / 253) {
            let limb = (self.citizen_pubkey.clone() >> (i * 253)) % (BigUint::one() << 253);
            preimage.push(biguint_to_fr(limb));
        }

        hasher.update(&preimage);
        let identity_commitment = hasher.squeeze();
        instance_column.push(identity_commitment);

        instance_column
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::{read_citizen_cert, read_nation_cert};
    use halo2_base::{
        halo2_proofs::{dev::MockProver, halo2curves::ff::PrimeField},
        utils::testing::base_test,
    };
    use num_traits::cast::ToPrimitive;
    use sha2::Sha256;

    #[test]
    fn mock() {
        let circuit =
            ProofOfJapaneseResidence::new("./certs/ca_cert.pem".into(), "./certs/myna_cert.pem".into(), 0xA42.into());
        let instance_column = circuit.instance_column();

        MockProver::run(K as u32, &circuit, vec![instance_column])
            .expect("The circuit generation failed")
            .assert_satisfied();
    }
}
