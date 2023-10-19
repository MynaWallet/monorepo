use clap::{Parser, Subcommand};
use halo2_base::{
    gates::circuit::builder::BaseCircuitBuilder, halo2_proofs::halo2curves::bn256::Fr,
    halo2_proofs::plonk::Circuit, utils::fs::gen_srs,
};
use halo2_circuits::helpers::*;
use snark_verifier_sdk::{
    evm::{evm_verify, gen_evm_proof_shplonk, gen_evm_verifier_shplonk, write_calldata},
    gen_pk,
    halo2::gen_snark_shplonk,
    read_pk, CircuitExt,
};
use std::env;
use std::fs::remove_file;
use std::path::Path;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand, Clone)]
enum Commands {
    /// Generate a setup paramter
    GenParams {
        /// k parameter for circuit.
        #[arg(long)]
        k: u32,
        #[arg(short, long, default_value = "./params")]
        params_path: String,
    },
    /// Generate proving keys for RSA circuit
    GenRsaKeys {
        /// k parameter for circuit.
        #[arg(long, default_value = "17")]
        k: u32,
        /// setup parameters path
        #[arg(short, long, default_value = "./params")]
        params_path: String,
        /// proving key path
        #[arg(long, default_value = "./build/rsa.pk")]
        pk_path: String,
        #[arg(long, default_value = "./certs/myna_cert.pem")]
        verify_cert_path: String,
        #[arg(long, default_value = "./certs/ca_cert.pem")]
        issuer_cert_path: String,
    },
    ProveRsa {
        /// k parameter for circuit.
        #[arg(long, default_value = "17")]
        k: u32,
        /// setup parameters path
        #[arg(short, long, default_value = "./params")]
        params_path: String,
        /// proving key path
        #[arg(long, default_value = "./build/rsa.pk")]
        pk_path: String,
        #[arg(long, default_value = "./certs/myna_cert.pem")]
        verify_cert_path: String,
        #[arg(long, default_value = "./certs/ca_cert.pem")]
        issuer_cert_path: String,
        /// output proof file
        #[arg(long, default_value = "./build/myna_verify_rsa.proof")]
        proof_path: String,
    },
    GenRsaVerifyEVMProof {
        /// k parameter for circuit.
        #[arg(long, default_value = "17")]
        k: u32,
        /// setup parameters path
        #[arg(short, long, default_value = "./params")]
        params_path: String,
        /// proving key path
        #[arg(long, default_value = "./build/rsa.pk")]
        pk_path: String,
        #[arg(long, default_value = "./certs/myna_cert.pem")]
        verify_cert_path: String,
        #[arg(long, default_value = "./certs/ca_cert.pem")]
        issuer_cert_path: String,
        /// output proof file
        #[arg(long, default_value = "./build/myna_verify_rsa.proof")]
        proof_path: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::GenParams { k, params_path } => {
            env::set_var("PARAMS_DIR", params_path);
            gen_srs(k);
        }
        Commands::GenRsaKeys {
            k,
            params_path,
            pk_path,
            verify_cert_path,
            issuer_cert_path,
        } => {
            env::set_var("PARAMS_DIR", params_path);
            let params = gen_srs(k);

            let (tbs, signature_bigint) = extract_tbs_and_sig(&verify_cert_path);
            let public_key_modulus = extract_public_key(&issuer_cert_path);

            let builder = create_default_rsa_circuit_with_instances(
                k as usize,
                tbs,
                public_key_modulus,
                signature_bigint,
            );

            if Path::new(&pk_path).exists() {
                match remove_file(&pk_path) {
                    Ok(_) => println!("File found, overwriting..."),
                    Err(e) => println!("An error occurred: {}", e),
                }
            }
            gen_pk(&params, &builder, Some(Path::new(&pk_path)));
        }
        Commands::ProveRsa {
            k,
            params_path,
            pk_path,
            verify_cert_path,
            issuer_cert_path,
            proof_path,
        } => {
            env::set_var("PARAMS_DIR", params_path);
            let params = gen_srs(k);

            let (tbs, signature_bigint) = extract_tbs_and_sig(&verify_cert_path);
            let public_key_modulus = extract_public_key(&issuer_cert_path);

            let builder = create_default_rsa_circuit_with_instances(
                k as usize,
                tbs,
                public_key_modulus,
                signature_bigint,
            );
            let pk =
                read_pk::<BaseCircuitBuilder<Fr>>(Path::new(&pk_path), builder.params()).unwrap();

            if Path::new(&proof_path).exists() {
                match remove_file(&proof_path) {
                    Ok(_) => println!("File found, overwriting..."),
                    Err(e) => println!("An error occurred: {}", e),
                }
            }
            gen_snark_shplonk(&params, &pk, builder.clone(), Some(Path::new(&proof_path)));
        }
        Commands::GenRsaVerifyEVMProof {
            k,
            params_path,
            pk_path,
            verify_cert_path,
            issuer_cert_path,
            proof_path,
        } => {
            env::set_var("PARAMS_DIR", params_path);
            let params = gen_srs(k);

            let (tbs, signature_bigint) = extract_tbs_and_sig(&verify_cert_path);
            let public_key_modulus = extract_public_key(&issuer_cert_path);

            let builder = create_default_rsa_circuit_with_instances(
                k as usize,
                tbs,
                public_key_modulus,
                signature_bigint,
            );
            let pk =
                read_pk::<BaseCircuitBuilder<Fr>>(Path::new(&pk_path), builder.params()).unwrap();

            if Path::new(&proof_path).exists() {
                match remove_file(&proof_path) {
                    Ok(_) => println!("File found, overwriting..."),
                    Err(e) => println!("An error occurred: {}", e),
                }
            }
            gen_snark_shplonk(&params, &pk, builder.clone(), Some(Path::new(&proof_path)));

            let deployment_code = gen_evm_verifier_shplonk::<BaseCircuitBuilder<Fr>>(
                &params,
                pk.get_vk(),
                builder.num_instance(),
                Some(Path::new("./build/VerifyRsa.sol")),
            );

            let proof = gen_evm_proof_shplonk(&params, &pk, builder.clone(), builder.instances());

            println!("Size of the contract: {} bytes", deployment_code.len());
            println!("Deploying contract...");

            evm_verify(deployment_code, builder.instances(), proof.clone());

            println!("Verification success!");

            write_calldata(&builder.instances(), &proof, Path::new("./build/calldata.txt")).unwrap();
            println!("Succesfully generate calldata!");
           
        }
    }
}
