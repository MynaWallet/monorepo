use halo2_base::{
    gates::{circuit::builder::BaseCircuitBuilder, GateInstructions},
    halo2_proofs::halo2curves::bn256::Fr,
    utils::fs::gen_srs,
    AssignedValue,
    QuantumCell::{Constant, Existing},
};
use halo2_rsa::{BigUintConfig, BigUintInstructions, RSAConfig, RSAInstructions, RSAPubE, RSAPublicKey, RSASignature};
use halo2_sha256_unoptimized::Sha256Chip;
use snark_verifier_sdk::{gen_pk, halo2::gen_snark_shplonk, Snark};

use itertools::Itertools;
use num_bigint::BigUint;
use openssl::ssl::{SslConnector, SslMethod};
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::{Read, Write},
    net::TcpStream,
    vec,
};
use x509_parser::{pem::parse_x509_pem, public_key::PublicKey};

pub fn extract_public_key(cert_path: &str) -> BigUint {
    println!("{:?}", cert_path);
    let mut cert_file = File::open(cert_path).expect("Failed to open cert pem file");
    let mut cert_pem_buffer = Vec::new();
    cert_file.read_to_end(&mut cert_pem_buffer).expect("Failed to read cert PEM file");

    let cert_pem = parse_x509_pem(&cert_pem_buffer).expect("Failed to parse cert").1;
    let cert = cert_pem.parse_x509().expect("Failed to parse PEM certificate");

    match cert.public_key().parsed().unwrap() {
        PublicKey::RSA(public_key) => BigUint::from_bytes_be(public_key.modulus),
        _ => panic!("Failed to get modulus"),
    }
}

pub fn extract_tbs_and_sig(cert_path: &str) -> (Vec<u8>, BigUint) {
    // Read the PEM certificate from a file
    let mut cert_file = File::open(cert_path).expect("Failed to open PEM file");
    let mut cert_pem_buffer = Vec::new();
    cert_file.read_to_end(&mut cert_pem_buffer).expect("Failed to read PEM file");

    // Parse the PEM certificate using x509-parser
    let cert_pem =
        parse_x509_pem(&cert_pem_buffer).unwrap_or_else(|e| panic!("Failed to parse PEM ${:?} {:?}", &cert_path, e)).1;
    let cert = cert_pem.parse_x509().expect("Failed to parse PEM certificate");

    // Extract the TBS (To-Be-Signed) data from the certificate
    let tbs = cert.tbs_certificate.as_ref();
    // println!("TBS (To-Be-Signed): {:x?}", tbs);

    // Extract the signature from cert 3
    let signature_bytes = &cert.signature_value;
    let signature_bigint = BigUint::from_bytes_be(&signature_bytes.data);
    // println!("Signature: {:?}", signature_bigint);

    (tbs.to_vec(), signature_bigint)
}

pub fn create_default_rsa_circuit_with_instances(
    k: usize,
    tbs: Vec<u8>,
    public_key_modulus: BigUint,
    signature_bigint: BigUint,
) -> BaseCircuitBuilder<Fr> {
    // Circuit inputs
    let limb_bits = 64;
    let default_bits = 2048;
    let exp_bits = 5; // UNUSED
    let default_e = 65537_u32;

    let mut builder = BaseCircuitBuilder::new(false);
    // Set rows
    builder.set_k(k);
    builder.set_lookup_bits(k - 1);
    builder.set_instance_columns(1);

    let range = builder.range_chip();
    let ctx = builder.main(0);

    let bigint_chip = BigUintConfig::construct(range.clone(), limb_bits);
    let rsa_chip = RSAConfig::construct(bigint_chip.clone(), default_bits, exp_bits);

    // Hash in pure Rust vs in-circuit
    let hashed_tbs = Sha256::digest(tbs);
    println!("Hashed TBS: {:?}", hashed_tbs);
    let mut hashed_bytes: Vec<AssignedValue<Fr>> =
        hashed_tbs.iter().map(|limb| ctx.load_witness(Fr::from(*limb as u64))).collect_vec();
    hashed_bytes.reverse();
    let bytes_bits = hashed_bytes.len() * 8;
    let limb_bits = bigint_chip.limb_bits();
    let limb_bytes = limb_bits / 8;
    let mut hashed_u64s = vec![];
    let bases = (0..limb_bytes).map(|i| Fr::from(1u64 << (8 * i))).map(Constant).collect_vec();
    for i in 0..(bytes_bits / limb_bits) {
        let left = hashed_bytes[limb_bytes * i..limb_bytes * (i + 1)].iter().map(|x| Existing(*x)).collect_vec();
        let sum = bigint_chip.gate().inner_product(ctx, left, bases.clone());
        hashed_u64s.push(sum);
    }

    // Generate values to be fed into the circuit (Pure Rust)
    // Verify Cert
    let e_fix = RSAPubE::Fix(BigUint::from(default_e));
    let public_key = RSAPublicKey::new(public_key_modulus.clone(), e_fix); // cloning might be slow
    let public_key = rsa_chip.assign_public_key(ctx, public_key).unwrap();

    let signature = RSASignature::new(signature_bigint.clone()); // cloning might be slow
    let signature = rsa_chip.assign_signature(ctx, signature).unwrap();

    let is_valid = rsa_chip.verify_pkcs1v15_signature(ctx, &public_key, &hashed_u64s, &signature).unwrap();
    rsa_chip.biguint_config().gate().assert_is_const(ctx, &is_valid, &Fr::one());

    // Insert input hash as public instance for circuit
    hashed_bytes.reverse();
    builder.assigned_instances[0].extend(hashed_bytes);

    let circuit_params = builder.calculate_params(Some(10));
    println!("Circuit params: {:?}", circuit_params);
    builder.use_params(circuit_params)
}
