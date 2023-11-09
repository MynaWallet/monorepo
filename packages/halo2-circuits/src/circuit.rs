use halo2_base::{
    gates::{
        circuit::{builder::BaseCircuitBuilder, BaseCircuitParams, BaseConfig},
        GateInstructions, RangeChip, RangeInstructions,
    },
    halo2_proofs::{
        arithmetic::Field,
        circuit::{Layouter, SimpleFloorPlanner},
        halo2curves::bn256::Fr,
        plonk::{Circuit, ConstraintSystem, Error},
    },
    poseidon::hasher::{spec::OptimizedPoseidonSpec, PoseidonHasher},
    AssignedValue, Context, QuantumCell,
};
use halo2_ecc::bigint::OverflowInteger;
use halo2_rsa::{
    AssignedBigUint, BigUintConfig, BigUintInstructions, Fresh, RSAConfig, RSAInstructions, RSAPubE, RSAPublicKey,
    RSASignature,
};
use num_bigint::BigUint;
use sha2::{Digest, Sha256};
// use zkevm_hashes::Sha256Chip;

#[derive(Debug, Clone)]
pub struct PublicInput {
    // 2048 bits
    pub nation_pubkey: BigUint,
}

#[derive(Debug, Clone)]
pub struct PrivateInput {
    pub tbs_cert: [u32; MAX_TBS_CERT_BITS / 32],
    // 2048 bits
    pub nation_sig: BigUint,
    pub password: Fr,
}

// halo2-sha256-unoptimized takes inputs byte by byte so I guess 8 is optimimal
pub const LOOKUP_BITS: usize = 8;
const RSA_KEY_SIZE: usize = 2048;
const PUBKEY_BEGINS: usize = 2216;
const E: usize = 65537;
const K: usize = 22;
const LIMB_BITS: usize = 64;
const SHA256_BLOCK_BITS: usize = 512;
const MAX_TBS_CERT_BITS: usize = 1 << 15;
const SHA256_INPUT_BLOCKS: usize = MAX_TBS_CERT_BITS / SHA256_BLOCK_BITS; // the remainder must be 0

pub fn bytes_to_biguint(
    ctx: &mut Context<Fr>,
    biguint_chip: BigUintConfig<Fr>,
    bytes: &[AssignedValue<Fr>],
) -> AssignedBigUint<Fr, Fresh> {
    let num_bases = (biguint_chip.limb_bits() / 8) as u64;
    let bases: Vec<QuantumCell<Fr>> =
        (0..num_bases).map(|i| QuantumCell::Constant(Fr::from(2).pow_vartime([i * 8]))).collect();
    let dest_limbs = bytes
        .chunks(biguint_chip.limb_bits() / 8)
        .map(|bytes_in_limb| {
            let dest_limb = biguint_chip.gate().inner_product(ctx, bytes_in_limb.to_vec(), bases.clone());
            biguint_chip.range().range_check(ctx, dest_limb, biguint_chip.limb_bits());
            dest_limb
        })
        .collect();

    let bytes_in_rust: Vec<u8> = bytes.iter().map(|byte| byte.value().to_bytes()[0].clone()).collect();
    let dest_in_rust = BigUint::from_bytes_le(&bytes_in_rust);
    AssignedBigUint::new(OverflowInteger::new(dest_limbs, biguint_chip.limb_bits()), dest_in_rust)
}

pub fn bytes_to_64s(
    ctx: &mut Context<Fr>,
    range_chip: RangeChip<Fr>,
    src: &[AssignedValue<Fr>],
) -> Vec<AssignedValue<Fr>> {
    // bug
    let biguint_chip = BigUintConfig::construct(range_chip, 64);
    bytes_to_biguint(ctx, biguint_chip, src).limbs().to_vec()
}

#[derive(Debug, Clone)]
struct Config {
    halo2base: BaseConfig<Fr>,
}

#[derive(Debug, Clone, Default)]
struct ProofOfJapaneseResidence {
    halo2base: BaseCircuitBuilder<Fr>,
}

impl Circuit<Fr> for ProofOfJapaneseResidence {
    type Config = Config;
    type Params = BaseCircuitParams;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn params(&self) -> Self::Params {
        self.halo2base.config_params.clone()
    }

    fn configure_with_params(meta: &mut ConstraintSystem<Fr>, params: BaseCircuitParams) -> Self::Config {
        Self::Config { halo2base: BaseConfig::configure(meta, params) }
    }

    fn configure(_: &mut ConstraintSystem<Fr>) -> Self::Config {
        unreachable!("halo2-base says I must not call configure");
    }

    fn synthesize(&self, config: Self::Config, mut layouter: impl Layouter<Fr>) -> Result<(), Error> {
        self.halo2base.synthesize(config.halo2base, layouter)?;

        // TODO: SHA256

        Ok(())
    }
}

pub fn proof_of_japanese_residence(
    ctx: &mut Context<Fr>,
    range_chip: RangeChip<Fr>,
    public: PublicInput,
    private: PrivateInput,
) -> Vec<AssignedValue<Fr>> {
    let biguint_chip: BigUintConfig<Fr> = BigUintConfig::construct(range_chip.clone(), LIMB_BITS);
    let rsa_chip = RSAConfig::construct(biguint_chip, RSA_KEY_SIZE, 5);
    // let mut sha256_chip =
    //     Sha256Chip::construct(vec![SHA256_INPUT_BLOCKS * SHA256_BLOCK_BITS / 8], range_chip.clone(), true);
    let mut poseidon = PoseidonHasher::new(OptimizedPoseidonSpec::<Fr, 3, 2>::new::<8, 57, 0>());
    poseidon.initialize_consts(ctx, rsa_chip.gate());

    // load public inputs
    let nation_pubkey =
        rsa_chip.assign_public_key(ctx, RSAPublicKey::new(public.nation_pubkey, RSAPubE::Fix(E.into()))).unwrap();

    // load private inputs
    let nation_sig = rsa_chip.assign_signature(ctx, RSASignature::new(private.nation_sig)).unwrap();
    let password = ctx.load_witness(private.password);

    // extract citizen's public key from the tbs certificate
    // let n = bytes_to_biguint(
    //     ctx,
    //     rsa_chip.biguint_config().clone(),
    //     &sha256ed.input_bytes[PUBKEY_BEGINS / 8..PUBKEY_BEGINS / 8 + RSA_KEY_SIZE / 8],
    // );
    // let citizen_pubkey = AssignedRSAPublicKey::new(n.clone(), AssignedRSAPubE::Fix(E.into()));

    // let sha256ed = sha256_chip.digest(ctx, &private.tbs_cert, None).unwrap();
    // let identity_commitment_preimage: Vec<AssignedValue<Fr>> = sha256ed.input_bytes
    //     [PUBKEY_BEGINS / 8..(PUBKEY_BEGINS + RSA_KEY_SIZE) / 8]
    //     .iter()
    //     .copied()
    //     .chain(std::iter::once(password))
    //     .collect();

    // println!("sha256ed");
    // for byte in &sha256ed.input_bytes[PUBKEY_BEGINS / 8..(PUBKEY_BEGINS + RSA_KEY_SIZE) / 8] {
    //     let byte = byte.value().to_repr()[0];
    //     print!("{:0b}", byte);
    // }

    // let identity_commitment = poseidon.hash_fix_len_array(ctx, rsa_chip.gate(), &identity_commitment_preimage);

    // let sha256ed_64s =
    //     bytes_to_64s(ctx, range_chip.clone(), &sha256ed.output_bytes.iter().rev().copied().collect::<Vec<_>>());

    // assert_eq!(
    //     sha256ed_64s.iter().flat_map(|a| a.value().to_bytes()[0..8].to_vec()).collect::<Vec<_>>(),
    //     Sha256::digest(private.tbs_cert).to_vec()
    // );
    // println!("sha256ed_64s.len(): {}", sha256ed_64s.len());
    // for bytes in sha256ed_64s.iter() {
    //     let limbs = bytes.value().to_bytes();
    //     for limb in limbs {
    //         print!("{:0b}", limb);
    //     }
    // }
    // println!("aa");

    // let hashed_tbs = Sha256::digest(private.tbs_cert);
    // println!("Hashed TBS: {:?}", hashed_tbs);
    // let mut hashed_bytes: Vec<AssignedValue<Fr>> =
    //     hashed_tbs.iter().map(|limb| ctx.load_witness(Fr::from(*limb as u64))).collect();
    // hashed_bytes.reverse();
    // let bytes_bits = hashed_bytes.len() * 8;
    // let limb_bits = 64;
    // let limb_bytes = limb_bits / 8;
    // let mut hashed_u64s = vec![];
    // let bases: Vec<_> = (0..limb_bytes).map(|i| Fr::from(1u64 << (8 * i))).map(QuantumCell::Constant).collect();
    // for i in 0..(bytes_bits / limb_bits) {
    //     let left: Vec<_> =
    //         hashed_bytes[limb_bytes * i..limb_bytes * (i + 1)].iter().map(|x| QuantumCell::Existing(*x)).collect();
    //     let sum = rsa_chip.gate().inner_product(ctx, left, bases.clone());
    //     hashed_u64s.push(sum);
    // }

    // assert_eq!(
    //     sha256ed_64s.iter().map(|a| a.value()).collect::<Vec<_>>(),
    //     hashed_u64s.iter().map(|a| a.value()).collect::<Vec<_>>()
    // );

    // let is_nation_sig_valid =
    //     rsa_chip.verify_pkcs1v15_signature(ctx, &nation_pubkey, &sha256ed_64s, &nation_sig).unwrap();
    // rsa_chip.biguint_config().gate().assert_is_const(ctx, &is_nation_sig_valid, &Fr::one());

    let mut outputs = nation_pubkey.n.limbs().to_vec();
    // outputs.push(identity_commitment);
    outputs
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

    // #[test]
    // fn aaa() {
    //     let two_pow_16 = Fr::from_raw([1 << 16, 0, 0, 0]);
    //     let mut test_subject = Fr::from_raw([0, 0, 0, 1 << 46]);
    //     while test_subject != Fr::zero() {
    //         for j in 1..16 {
    //             let k = test_subject * Fr::from_raw([1 << j, 0, 0, 0]);
    //             if two_pow_16 >= k {
    //                 unreachable!("i:{:?},j:{:?},k:{:?}", test_subject.to_repr(), j, k.to_repr());
    //             }
    //         }

    //         test_subject += Fr::one();
    //     }
    // }

    #[test]
    fn mock() {
        let mut builder = BaseCircuitBuilder::default();
        builder.set_k(K);
        builder.set_lookup_bits(LOOKUP_BITS);
        builder.set_instance_columns(1);
        let range_chip = builder.range_chip().clone();

        let nation_pubkey = read_nation_cert("./certs/ca_cert.pem");
        let (nation_sig, tbs_cert, citizen_pubkey) = read_citizen_cert("./certs/myna_cert.pem");

        let mut tbs_cert_32 = [0u32; MAX_TBS_CERT_BITS / 32];
        tbs_cert_32[0..tbs_cert.to_u32_digits().len()].copy_from_slice(&tbs_cert.to_u32_digits());

        let public_input = PublicInput { nation_pubkey: nation_pubkey.clone() };
        let private_input =
            PrivateInput { tbs_cert: tbs_cert_32, nation_sig: nation_sig.clone(), password: Fr::from(0xA42) };
        let public_output =
            proof_of_japanese_residence(builder.pool(0).main(), range_chip, public_input, private_input);

        builder.assigned_instances[0].extend(public_output);
        // AUDIT: Is K enough to achieve zero knowledge?
        builder.calculate_params(Some(9));

        let circuit = ProofOfJapaneseResidence { halo2base: builder };
        dbg!(circuit.params());

        MockProver::run(
            K as u32,
            &circuit,
            circuit
                .halo2base
                .assigned_instances
                .iter()
                .map(|public_column| public_column.into_iter().map(|public_cell| public_cell.value().clone()).collect())
                .collect(),
        )
        .expect("The circuit generation failed")
        .assert_satisfied();
    }
}
