use clap::{Parser, Subcommand};
use halo2_circuits::{circuit, der};
use halo2_proofs::{
    halo2curves::bn256::{Bn256, Fr, G1Affine},
    plonk::{create_proof, keygen_pk, keygen_vk, verify_proof, ProvingKey, VerifyingKey},
    poly::{
        commitment::Params,
        kzg::{
            commitment::ParamsKZG,
            multiopen::{ProverSHPLONK, VerifierSHPLONK},
            strategy::SingleStrategy,
        },
    },
    transcript::{Challenge255, Keccak256Read, Keccak256Write, TranscriptReadBuffer, TranscriptWriterBuffer},
    SerdeFormat,
};
use rand::rngs::OsRng;
use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand, Clone)]
enum Commands {
    #[command(subcommand)]
    App(AppCommands),
}

#[derive(Debug, Subcommand, Clone)]
enum AppCommands {
    /// Generate a trusted setup paramter
    TrustedSetup {
        /// trusted setup parameters path. input
        #[arg(short, long, default_value = "./build/app/trusted_setup")]
        trusted_setup_path: String,
    },
    /// Generate the proving key and the verification key for RSA circuit
    Keys {
        /// trusted setup parameters path. input
        #[arg(short, long, default_value = "./build/app/trusted_setup")]
        trusted_setup_path: String,
        /// verification key path. output
        #[arg(long, default_value = "./build/app/vk")]
        vk_path: String,
        /// proving key path. output
        #[arg(long, default_value = "./build/app/pk")]
        pk_path: String,
    },
    Prove {
        /// trusted setup parameters path. input
        #[arg(short, long, default_value = "./build/app/trusted_setup")]
        trusted_setup_path: String,
        /// proving key path. input
        #[arg(long, default_value = "./build/app/pk")]
        pk_path: String,
        /// proof path. output
        #[arg(long, default_value = "./build/app/proof")]
        proof_path: String,
    },
    Verify {
        /// trusted setup parameters path. input
        #[arg(short, long, default_value = "./build/app/trusted_setup")]
        trusted_setup_path: String,
        /// verification key path. input
        #[arg(long, default_value = "./build/app/vk")]
        vk_path: String,
        /// proof path. input
        #[arg(long, default_value = "./build/app/proof")]
        proof_path: String,
    },
    // Evm {
    //     /// trusted setup parameters path. input
    //     #[arg(short, long, default_value = "./build/app/trusted_setup")]
    //     trusted_setup_path: String,
    //     /// verification key path. input
    //     #[arg(long, default_value = "./build/app/vk")]
    //     vk_path: String,
    //     /// proof path. input
    //     #[arg(long, default_value = "./build/app/proof")]
    //     proof_path: String,
    //     /// verifier.sol path. output
    //     #[arg(short, long, default_value = "./build/app/verifier.sol")]
    //     solidity_path: String,
    //     /// calldata path. output
    //     #[arg(long, default_value = "./build/app/calldata.txt")]
    //     calldata_path: String,
    // },
    // Snark {
    //     /// trusted setup parameters path. input
    //     #[arg(short, long, default_value = "./build/app/trusted_setup")]
    //     trusted_setup_path: String,
    //     /// proving key path. input
    //     #[arg(long, default_value = "./build/app/pk")]
    //     pk_path: String,
    //     /// partial proof path. output
    //     #[arg(long, default_value = "./build/app/snark")]
    //     snark_path: String,
    //     // citizen's certificate. input
    //     #[arg(long, default_value = "./certs/myna_cert.pem")]
    //     verify_cert_path: String,
    //     // nation's certificate. input
    //     #[arg(long, default_value = "./certs/ca_cert.pem")]
    //     issuer_cert_path: String,
    //     #[arg(default_value = "42")]
    //     password: u64,
    // },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::App(command) => match command {
            AppCommands::TrustedSetup { trusted_setup_path } => {
                let trusted_setup_path = Path::new(&trusted_setup_path);
                if trusted_setup_path.exists() {
                    println!("Trusted setup already exists. Overwriting...");
                }

                let mut file =
                    BufWriter::new(File::create(trusted_setup_path).expect("Failed to create a trusted setup"));
                let trusted_setup_file = ParamsKZG::<Bn256>::setup(der::K as u32, OsRng);
                trusted_setup_file.write(&mut file).expect("Failed to write a trusted setup");
            }
            AppCommands::Keys { trusted_setup_path, pk_path, vk_path } => {
                let der_hex: String = std::env::var("TBS_CERTIFICATE")
                    .expect("You must set $TBS_CERTIFICATE the certificate to use in the test");
                let mut der_bytes = hex::decode(der_hex).expect("Invalid $TBS_CERTIFICATE");
                der_bytes.extend(vec![0; (1 << der::K) - der::Circuit::BLINDING_FACTORS - der_bytes.len()]);
                let path = [7, 0, 1, 1, 0, 0, 1, 0];
                let circuit = crate::der::Circuit { der_bytes, path_table: crate::der::PathTable::new(&path) };

                let mut trusted_setup_file = File::open(trusted_setup_path).expect("Couldn't open the trusted setup");
                let trusted_setup =
                    ParamsKZG::<Bn256>::read_custom(&mut trusted_setup_file, SerdeFormat::RawBytes).unwrap();

                let vk = keygen_vk(&trusted_setup, &circuit).unwrap();
                let mut vk_file = BufWriter::new(File::create(vk_path).unwrap());
                vk.write(&mut vk_file, SerdeFormat::RawBytes).unwrap();

                let pk = keygen_pk(&trusted_setup, vk, &circuit).unwrap();
                let mut pk_file = BufWriter::new(File::create(pk_path).unwrap());
                pk.write(&mut pk_file, SerdeFormat::RawBytes).unwrap();
            }
            AppCommands::Prove { trusted_setup_path, pk_path, proof_path } => {
                let der_hex: String = std::env::var("TBS_CERTIFICATE")
                    .expect("You must set $TBS_CERTIFICATE the certificate to use in the test");
                let mut der_bytes = hex::decode(der_hex).expect("Invalid $TBS_CERTIFICATE");
                der_bytes.extend(vec![0; (1 << der::K) - der::Circuit::BLINDING_FACTORS - der_bytes.len()]);
                let path = [7, 0, 1, 1, 0, 0, 1, 0];
                let circuit = crate::der::Circuit { der_bytes, path_table: crate::der::PathTable::new(&path) };

                let mut trusted_setup_file = File::open(trusted_setup_path).expect("Couldn't open the trusted setup");
                let trusted_setup =
                    ParamsKZG::<Bn256>::read_custom(&mut trusted_setup_file, SerdeFormat::RawBytes).unwrap();

                let pk = ProvingKey::read::<BufReader<File>, der::Circuit>(
                    &mut BufReader::new(File::open(&pk_path).unwrap()),
                    SerdeFormat::RawBytes,
                )
                .unwrap();

                let proof_file = BufWriter::new(File::create(proof_path).unwrap());
                let mut proof = Keccak256Write::<_, _, Challenge255<_>>::init(proof_file);
                create_proof::<_, ProverSHPLONK<'_, Bn256>, _, _, Keccak256Write<_, _, _>, _>(
                    &trusted_setup,
                    &pk,
                    &[circuit],
                    &[&[]],
                    OsRng,
                    &mut proof,
                )
                .expect("prover should not fail");
                proof.finalize();
                println!("Proof generation finished at: {:?}", std::time::Instant::now());
            }
            AppCommands::Verify { proof_path, trusted_setup_path, vk_path } => {
                let mut trusted_setup_file = File::open(trusted_setup_path).expect("Couldn't open the trusted setup");
                let trusted_setup =
                    ParamsKZG::<Bn256>::read_custom(&mut trusted_setup_file, SerdeFormat::RawBytes).unwrap();

                let vk = VerifyingKey::read::<BufReader<File>, der::Circuit>(
                    &mut BufReader::new(File::open(vk_path).unwrap()),
                    SerdeFormat::RawBytes,
                )
                .unwrap();

                let mut proof = Keccak256Read::init(File::open(&proof_path).unwrap());
                let result = verify_proof::<_, VerifierSHPLONK<'_, Bn256>, _, Keccak256Read<_, _, _>, _>(
                    &trusted_setup,
                    &vk,
                    SingleStrategy::new(&trusted_setup),
                    &[&[&[]]],
                    &mut proof,
                );
                assert!(result.is_ok(), "Verification failed!");
                println!("Verification succeeded!");
            }
        },
    }
}
