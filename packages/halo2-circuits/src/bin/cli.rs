use clap::{Parser, Subcommand};
use halo2_base::{
    gates::circuit::builder::BaseCircuitBuilder,
    halo2_proofs::{halo2curves::bn256::Fr, plonk::Circuit},
    utils::{fs::gen_srs, BigPrimeField},
};
use halo2_circuits::{circuit, helpers::*};
use snark_verifier_sdk::{
    evm::{gen_evm_proof_shplonk, gen_evm_verifier_shplonk, write_calldata},
    gen_pk,
    halo2::gen_snark_shplonk,
    read_pk, CircuitExt,
};
use std::{env, fs::remove_file, path::Path};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand, Clone)]
enum Commands {
    /// Generate a trusted setup paramter
    TrustedSetup {
        /// k parameter for circuit.
        #[arg(long)]
        k: u32,
        #[arg(short, long, default_value = "./params")]
        params_path: String,
    },
    /// Generate the proving key and the verification key for RSA circuit
    GenerateKeys {
        /// k parameter for circuit.
        #[arg(long, default_value = "17")]
        k: u32,
        /// trusted setup parameters path. input
        #[arg(short, long, default_value = "./params")]
        params_path: String,
        /// proving key path. output
        #[arg(long, default_value = "./build/rsa.pk")]
        pk_path: String,
        /// proof path. output
        #[arg(long, default_value = "./build/myna_verify_rsa.proof")]
        proof_path: String,
        // citizen's certificate
        #[arg(long, default_value = "./certs/myna_cert.pem")]
        verify_cert_path: String,
        // nation's certificate
        #[arg(long, default_value = "./certs/ca_cert.pem")]
        issuer_cert_path: String,
        password: u64,
    },
    /// Generate the proving key and the verification key for RSA circuit
    Prove {
        /// k parameter for circuit.
        #[arg(long, default_value = "17")]
        k: u32,
        /// trusted setup parameters path. input
        #[arg(short, long, default_value = "./params")]
        params_path: String,
        /// proving key path. output
        #[arg(long, default_value = "./build/rsa.pk")]
        pk_path: String,
        /// proof path. output
        #[arg(long, default_value = "./build/myna_verify_rsa.proof")]
        proof_path: String,
        // citizen's certificate
        #[arg(long, default_value = "./certs/myna_cert.pem")]
        verify_cert_path: String,
        // nation's certificate
        #[arg(long, default_value = "./certs/ca_cert.pem")]
        issuer_cert_path: String,
        password: u64,
    },
    /// Generate the proving key and the verification key for RSA circuit
    GenerateSolidity {
        /// k parameter for circuit.
        #[arg(long, default_value = "17")]
        k: u32,
        /// trusted setup parameters path. input
        #[arg(short, long, default_value = "./params")]
        params_path: String,
        /// proving key path. output
        #[arg(long, default_value = "./build/rsa.pk")]
        pk_path: String,
        /// proof path. output
        #[arg(long, default_value = "./build/myna_verify_rsa.proof")]
        proof_path: String,
        // citizen's certificate
        #[arg(long, default_value = "./certs/myna_cert.pem")]
        verify_cert_path: String,
        // nation's certificate
        #[arg(long, default_value = "./certs/ca_cert.pem")]
        issuer_cert_path: String,
        password: u64,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::TrustedSetup { k, params_path } => {
            env::set_var("PARAMS_DIR", params_path);
            gen_srs(k);
        }
        Commands::GenerateKeys {
            k,
            params_path,
            pk_path,
            proof_path,
            verify_cert_path,
            issuer_cert_path,
            password,
        } => {
            // let nation_pubkey = read_nation_cert(&issuer_cert_path);
            // let (nation_sig, tbs_cert, _citizen_pubkey) = read_citizen_cert(&verify_cert_path);

            // let mut builder = BaseCircuitBuilder::new(false);
            // builder.set_k(k as usize);
            // builder.set_lookup_bits(circuit::LOOKUP_BITS);
            // builder.set_instance_columns(1);
            // let range_chip = builder.range_chip();
            // let ctx = builder.main(0);

            // let public = circuit::PublicInput { nation_pubkey };
            // let private =
            //     circuit::PrivateInput { tbs_cert: tbs_cert.to_bytes_le(), nation_sig, password: Fr::from(password) };
            // dbg!(tbs_cert.to_bytes_le().len());
            // let outputs = circuit::proof_of_japanese_residence(ctx, range_chip, public, private);

            // builder.assigned_instances[0].extend(outputs);
            // let circuit_params = builder.calculate_params(None);
            // builder = builder.use_params(circuit_params);

            // if Path::new(&pk_path).exists() {
            //     match remove_file(&pk_path) {
            //         Ok(_) => println!("File found, overwriting..."),
            //         Err(e) => println!("An error occurred: {}", e),
            //     }
            // }

            // env::set_var("PARAMS_DIR", params_path);
            // let trusted_setup = gen_srs(k);
            // gen_pk(&trusted_setup, &builder, Some(Path::new(&pk_path)));
        }
        Commands::Prove { k, params_path, pk_path, proof_path, verify_cert_path, issuer_cert_path, password } => {
            // let nation_pubkey = read_nation_cert(&issuer_cert_path);
            // let (nation_sig, tbs_cert, citizen_pubkey) = read_citizen_cert(&verify_cert_path);

            // let mut builder = BaseCircuitBuilder::new(false);
            // builder.set_k(k as usize);
            // builder.set_lookup_bits(circuit::LOOKUP_BITS);
            // builder.set_instance_columns(1);
            // let range_chip = builder.range_chip();
            // let ctx = builder.main(0);

            // let public = circuit::PublicInput { nation_pubkey };
            // let private =
            //     circuit::PrivateInput { tbs_cert: tbs_cert.to_bytes_le(), nation_sig, password: Fr::from(password) };
            // let outputs = circuit::proof_of_japanese_residence(ctx, range_chip, public, private);

            // builder.assigned_instances[0].extend(outputs);
            // let circuit_params = builder.calculate_params(None);
            // builder = builder.use_params(circuit_params);

            // if Path::new(&proof_path).exists() {
            //     match remove_file(&proof_path) {
            //         Ok(_) => println!("File found, overwriting..."),
            //         Err(e) => println!("An error occurred: {}", e),
            //     }
            // }

            // env::set_var("PARAMS_DIR", params_path);
            // let trusted_setup = gen_srs(k);
            // let pk = read_pk::<BaseCircuitBuilder<Fr>>(Path::new(&pk_path), builder.params()).unwrap();
            // gen_snark_shplonk(&trusted_setup, &pk, builder, Some(Path::new(&proof_path)));
        }
        Commands::GenerateSolidity {
            k,
            params_path,
            pk_path,
            proof_path,
            verify_cert_path,
            issuer_cert_path,
            password,
        } => {
            // let nation_pubkey = read_nation_cert(&issuer_cert_path);
            // let (nation_sig, tbs_cert, citizen_pubkey) = read_citizen_cert(&verify_cert_path);

            // let mut builder = BaseCircuitBuilder::new(false);
            // builder.set_k(k as usize);
            // builder.set_lookup_bits(circuit::LOOKUP_BITS);
            // builder.set_instance_columns(1);
            // let range_chip = builder.range_chip();
            // let ctx = builder.main(0);

            // let public = circuit::PublicInput { nation_pubkey };
            // let private =
            //     circuit::PrivateInput { tbs_cert: tbs_cert.to_bytes_le(), nation_sig, password: Fr::from(password) };
            // let outputs = circuit::proof_of_japanese_residence(ctx, range_chip, public, private);

            // builder.assigned_instances[0].extend(outputs);
            // let circuit_params = builder.calculate_params(None);
            // builder = builder.use_params(circuit_params);

            // env::set_var("PARAMS_DIR", params_path);
            // let trusted_setup = gen_srs(k);
            // let pk = read_pk::<BaseCircuitBuilder<Fr>>(Path::new(&pk_path), builder.params()).unwrap();

            // let deployment_code = gen_evm_verifier_shplonk::<BaseCircuitBuilder<Fr>>(
            //     &trusted_setup,
            //     pk.get_vk(),
            //     builder.num_instance(),
            //     Some(Path::new("./build/VerifyRsa.sol")),
            // );

            // let proof = gen_evm_proof_shplonk(&trusted_setup, &pk, builder.clone(), builder.instances());

            // println!("Size of the contract: {} bytes", deployment_code.len());
            // println!("Deploying contract...");

            // evm_verify(deployment_code, builder.instances(), proof.clone());

            // println!("Verification success!");

            // write_calldata(&builder.instances(), &proof, Path::new("./build/calldata.txt")).unwrap();
            // println!("Succesfully generate calldata!");
        }
    }
}
