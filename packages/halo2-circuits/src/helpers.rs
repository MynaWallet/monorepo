use halo2_base::{
    gates::{circuit::builder::BaseCircuitBuilder, GateInstructions},
    halo2_proofs::halo2curves::bn256::Fr,
    utils::fs::gen_srs,
    AssignedValue,
    QuantumCell::{Constant, Existing},
};
use halo2_rsa::{BigUintConfig, BigUintInstructions, RSAConfig, RSAInstructions, RSAPubE, RSAPublicKey, RSASignature};
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

pub fn read_nation_cert(cert_path: &str) -> BigUint {
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

// returns
// - nation's signature
// - citizen's tbs certificate
// - citizen's public key
pub fn read_citizen_cert(cert_path: &str) -> (BigUint, BigUint, BigUint) {
    // Read the PEM certificate from a file
    let mut cert_file = File::open(cert_path).expect("Failed to open PEM file");
    let mut cert_pem_buffer = Vec::new();
    cert_file.read_to_end(&mut cert_pem_buffer).expect("Failed to read PEM file");

    // Parse the PEM certificate using x509-parser
    let cert_pem =
        parse_x509_pem(&cert_pem_buffer).unwrap_or_else(|e| panic!("Failed to parse PEM ${:?} {:?}", &cert_path, e)).1;
    let cert = cert_pem.parse_x509().expect("Failed to parse PEM certificate");

    // Extract the TBS (To-Be-Signed) data from the certificate
    let tbs_bytes = cert.tbs_certificate.as_ref();
    dbg!(tbs_bytes.len());
    let tbs_biguint = BigUint::from_bytes_le(tbs_bytes);
    // println!("TBS (To-Be-Signed): {:x?}", tbs);

    // Extract the signature from cert 3
    let nation_sig_bytes = &cert.signature_value;
    let nation_sig_biguint = BigUint::from_bytes_be(&nation_sig_bytes.data);

    let citizen_pubkey_bytes = cert.tbs_certificate.subject_pki.subject_public_key.as_ref();
    let citizen_pubkey_biguint = BigUint::from_bytes_le(&citizen_pubkey_bytes[9..256 + 9]);

    (nation_sig_biguint, tbs_biguint, citizen_pubkey_biguint)
}
