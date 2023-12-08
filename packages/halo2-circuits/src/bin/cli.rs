use clap::{Parser, Subcommand};
use halo2_base::{
    gates::circuit::{builder::BaseCircuitBuilder, CircuitBuilderStage},
    halo2_proofs::{
        dev::MockProver,
        halo2curves::bn256::{Bn256, Fr, G1Affine},
        plonk::{create_proof, keygen_pk, keygen_vk, verify_proof, Circuit, ProvingKey, VerifyingKey},
        poly::{
            commitment::Params,
            kzg::{
                commitment::{KZGCommitmentScheme, ParamsKZG},
                multiopen::{ProverSHPLONK, VerifierSHPLONK},
                strategy::{AccumulatorStrategy, SingleStrategy},
            },
        },
        transcript::{Challenge255, Keccak256Read, Keccak256Write, TranscriptReadBuffer, TranscriptWriterBuffer},
        SerdeFormat,
    },
    utils::{fs::gen_srs, BigPrimeField},
};
use halo2_circuits::{
    circuit::{self, ProofOfJapaneseResidence},
    helpers::*,
};
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};
use snark_verifier_sdk::{
    evm::{evm_verify, gen_evm_proof_shplonk, gen_evm_verifier_shplonk, write_calldata},
    halo2::{
        aggregation::{AggregationCircuit, AggregationConfigParams, VerifierUniversality},
        gen_snark_shplonk,
    },
    snark_verifier::system::halo2::transcript::evm::EvmTranscript,
    CircuitExt, SHPLONK,
};
use std::{
    env,
    fmt::Binary,
    fs::{remove_file, File},
    io::{BufWriter, Read, Write},
    path::{Path, PathBuf},
};

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
        /// trusted setup parameters path. input
        #[arg(short, long, default_value = "./build/trusted_setup")]
        trusted_setup_path: String,
    },
    #[command(subcommand)]
    App(AppCommands),
    #[command(subcommand)]
    Agg(AggCommands),
}

#[derive(Debug, Subcommand, Clone)]
enum AppCommands {
    /// Generate the proving key and the verification key for RSA circuit
    Keys {
        /// trusted setup parameters path. input
        #[arg(short, long, default_value = "./build/trusted_setup")]
        trusted_setup_path: String,
        /// proving key path. output
        #[arg(long, default_value = "./build/app/vk")]
        vk_path: String,
        /// proving key path. output
        #[arg(long, default_value = "./build/app/pk")]
        pk_path: String,
    },
    Prove {
        /// trusted setup parameters path. input
        #[arg(short, long, default_value = "./build/trusted_setup")]
        trusted_setup_path: String,
        /// proving key path. input
        #[arg(long, default_value = "./build/app/pk")]
        pk_path: String,
        /// proof path. output
        #[arg(long, default_value = "./build/app/proof")]
        proof_path: String,
        // citizen's certificate. input
        #[arg(long, default_value = "./certs/myna_cert.pem")]
        verify_cert_path: String,
        // nation's certificate. input
        #[arg(long, default_value = "./certs/ca_cert.pem")]
        issuer_cert_path: String,
        #[arg(default_value = "42")]
        password: u64,
    },
    Verify {
        /// trusted setup parameters path. input
        #[arg(short, long, default_value = "./build/trusted_setup")]
        trusted_setup_path: String,
        /// verification key path. input
        #[arg(long, default_value = "./build/app/vk")]
        vk_path: String,
        /// proof path. input
        #[arg(long, default_value = "./build/app/proof")]
        proof_path: String,
        // citizen's certificate. inut
        #[arg(long, default_value = "./certs/myna_cert.pem")]
        verify_cert_path: String,
        // nation's certificate. input
        #[arg(long, default_value = "./certs/ca_cert.pem")]
        issuer_cert_path: String,
        #[arg(default_value = "42")]
        password: u64,
    },
    Evm {
        /// trusted setup parameters path. input
        #[arg(short, long, default_value = "./build/trusted_setup")]
        trusted_setup_path: String,
        /// verification key path. input
        #[arg(long, default_value = "./build/app/pk")]
        vk_path: String,
        /// proof path. input
        #[arg(long, default_value = "./build/app/proof")]
        proof_path: String,
        /// verifier.sol path. output
        #[arg(short, long, default_value = "./build/app/verifier.sol")]
        solidity_path: String,
        /// calldata path. output
        #[arg(long, default_value = "./build/app/calldata.txt")]
        calldata_path: String,
    },
}

#[derive(Debug, Subcommand, Clone)]
enum AggCommands {
    Keys {
        /// trusted setup parameters path. input
        #[arg(short, long, default_value = "./build/trusted_setup")]
        trusted_setup_path: String,
        /// proving key path. output
        #[arg(long, default_value = "./build/agg/vk")]
        vk_path: String,
        /// proving key path. output
        #[arg(long, default_value = "./build/agg/pk")]
        pk_path: String,
    },
    Prove {
        /// trusted setup parameters path. input
        #[arg(short, long, default_value = "./build/trusted_setup")]
        trusted_setup_path: String,
        /// proving key path. output
        #[arg(long, default_value = "./build/agg/pk")]
        pk_path: String,
        /// proof path. output
        #[arg(long, default_value = "./build/agg/proof")]
        proof_path: String,
        // citizen's certificate
        #[arg(long, default_value = "./certs/myna_cert.pem")]
        verify_cert_path: String,
        // nation's certificate
        #[arg(long, default_value = "./certs/ca_cert.pem")]
        issuer_cert_path: String,
        #[arg(default_value = "42")]
        password: u64,
    },
    Verify {
        /// trusted setup parameters path. input
        #[arg(short, long, default_value = "./build/trusted_setup")]
        trusted_setup_path: String,
        /// proving key path. output
        #[arg(long, default_value = "./build/agg/vk")]
        vk_path: String,
        /// proof path. output
        #[arg(long, default_value = "./build/agg/proof")]
        proof_path: String,
        // citizen's certificate
        #[arg(long, default_value = "./certs/myna_cert.pem")]
        verify_cert_path: String,
        // nation's certificate
        #[arg(long, default_value = "./certs/ca_cert.pem")]
        issuer_cert_path: String,
        #[arg(default_value = "42")]
        password: u64,
    },
    Evm {
        /// trusted setup parameters path. input
        #[arg(short, long, default_value = "./build/trusted_setup")]
        trusted_setup_path: String,
        /// proving key path. output
        #[arg(long, default_value = "./build/agg/pk")]
        pk_path: String,
        #[arg(short, long, default_value = "./build/agg/verifier.sol")]
        solidity_path: String,
        #[arg(short, long, default_value = "./build/agg/calldata.txt")]
        calldata_path: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::TrustedSetup { trusted_setup_path } => {
            let trusted_setup_path = Path::new(&trusted_setup_path);
            if trusted_setup_path.exists() {
                println!("Trusted setup already exists. Overwriting...");
            }

            let mut file = File::create(trusted_setup_path).expect("Failed to create a trusted setup");
            let trusted_setup_file = ParamsKZG::<Bn256>::setup(circuit::K as u32, OsRng);
            trusted_setup_file.write(&mut file).expect("Failed to write a trusted setup");
        }
        Commands::App(command) => match command {
            AppCommands::Keys { trusted_setup_path, pk_path, vk_path } => {
                let circuit = circuit::ProofOfJapaneseResidence::new(
                    "./certs/ca_cert.pem".into(),
                    "./certs/myna_cert.pem".into(),
                    0xA42.into(),
                );

                let mut trusted_setup_file = File::open(trusted_setup_path).expect("Couldn't open the trusted setup");
                let trusted_setup = ParamsKZG::<Bn256>::read_custom(&mut trusted_setup_file, SerdeFormat::RawBytes)
                    .expect("The trusted setup is corrupted");

                let vk = keygen_vk(&trusted_setup, &circuit).unwrap();
                let mut vk_file = File::create(vk_path).unwrap();
                vk.write(&mut vk_file, SerdeFormat::RawBytes).unwrap();

                let pk = keygen_pk(&trusted_setup, vk, &circuit).unwrap();
                let mut pk_file = File::create(pk_path).unwrap();
                pk.write(&mut pk_file, SerdeFormat::RawBytes).unwrap();
            }
            AppCommands::Prove {
                verify_cert_path,
                issuer_cert_path,
                password,
                trusted_setup_path,
                pk_path,
                proof_path,
            } => {
                let circuit = circuit::ProofOfJapaneseResidence::new(
                    issuer_cert_path.into(),
                    verify_cert_path.into(),
                    password.into(),
                );

                let mut trusted_setup_file = File::open(trusted_setup_path).expect("Couldn't open the trusted setup");
                let trusted_setup = ParamsKZG::<Bn256>::read_custom(&mut trusted_setup_file, SerdeFormat::RawBytes)
                    .expect("The trusted setup is corrupted");

                let mut pk_file = File::open(pk_path).expect("pk not found. Run generate-keys first.");
                let pk = ProvingKey::<G1Affine>::read::<_, circuit::ProofOfJapaneseResidence>(
                    &mut pk_file,
                    SerdeFormat::RawBytes,
                    circuit.params(),
                )
                .unwrap();

                let mut proof_file = BufWriter::new(File::create(proof_path).unwrap());
                let proof =
                    gen_evm_proof_shplonk(&trusted_setup, &pk, circuit.clone(), vec![circuit.instance_column()]);
                proof_file.write_all(&proof).unwrap();
            }
            AppCommands::Verify {
                proof_path,
                verify_cert_path,
                issuer_cert_path,
                password,
                trusted_setup_path,
                vk_path,
            } => {
                let circuit = circuit::ProofOfJapaneseResidence::new(
                    issuer_cert_path.into(),
                    verify_cert_path.into(),
                    password.into(),
                );

                let mut trusted_setup_file = File::open(trusted_setup_path).expect("Couldn't open the trusted setup");
                let trusted_setup = ParamsKZG::<Bn256>::read_custom(&mut trusted_setup_file, SerdeFormat::RawBytes)
                    .expect("The trusted setup is corrupted");

                let mut vk_file = File::open(vk_path).expect("vk not found. Run generate-keys first.");
                let vk = VerifyingKey::<G1Affine>::read::<_, circuit::ProofOfJapaneseResidence>(
                    &mut vk_file,
                    SerdeFormat::RawBytes,
                    circuit.params(),
                )
                .unwrap();

                let proof_file = File::open(&proof_path).unwrap();
                let mut proof = TranscriptReadBuffer::<_, _, _>::init(&proof_file);
                let result = verify_proof::<_, VerifierSHPLONK<'_, Bn256>, _, EvmTranscript<_, _, _, _>, _>(
                    &trusted_setup,
                    &vk,
                    AccumulatorStrategy::new(&trusted_setup),
                    &[&[&circuit.instance_column()]],
                    &mut proof,
                );
                assert!(result.is_ok(), "Verification failed!");
                println!("Verification succeeded!");
            }
            AppCommands::Evm { trusted_setup_path, vk_path, proof_path, solidity_path, calldata_path } => {
                let circuit = circuit::ProofOfJapaneseResidence::new(
                    "./certs/ca_cert.pem".into(),
                    "./certs/myna_cert.pem".into(),
                    0xA42.into(),
                );

                let mut trusted_setup_file = File::open(trusted_setup_path).expect("Couldn't open the trusted setup");
                let trusted_setup = ParamsKZG::<Bn256>::read_custom(&mut trusted_setup_file, SerdeFormat::RawBytes)
                    .expect("The trusted setup is corrupted");

                let mut proof_file = File::open(&proof_path).expect("proof not found. Do `cargo run app prove` first.");
                let mut proof: Vec<u8> = Vec::new();
                proof_file.read_to_end(&mut proof).unwrap();

                let mut vk_file = File::open(vk_path).expect("vk not found. Run generate-keys first.");
                let vk = VerifyingKey::<G1Affine>::read::<_, circuit::ProofOfJapaneseResidence>(
                    &mut vk_file,
                    SerdeFormat::RawBytes,
                    circuit.params(),
                )
                .unwrap();

                write_calldata(&[circuit.instance_column()], &proof, Path::new(&calldata_path)).unwrap();

                let verifier = gen_evm_verifier_shplonk::<BaseCircuitBuilder<Fr>>(
                    &trusted_setup,
                    &vk,
                    vec![circuit.instance_column().len()],
                    Some(Path::new(&solidity_path)),
                );

                evm_verify(verifier, vec![circuit.instance_column()], proof.clone());
            }
        },
        Commands::Agg(_) => todo!(),
    }
}
