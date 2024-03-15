use clap::{Parser, Subcommand};
use halo2_base::{
    gates::circuit::{builder::BaseCircuitBuilder, CircuitBuilderStage},
    halo2_proofs::{
        halo2curves::bn256::{Bn256, Fr, G1Affine},
        plonk::{keygen_pk, keygen_vk, verify_proof, Circuit, VerifyingKey},
        poly::{
            commitment::Params,
            kzg::{commitment::ParamsKZG, multiopen::VerifierSHPLONK, strategy::AccumulatorStrategy},
        },
        transcript::TranscriptReadBuffer,
        SerdeFormat,
    },
};
use halo2_circuits::circuit;
use rand::rngs::OsRng;
use snark_verifier_sdk::{
    evm::{evm_verify, gen_evm_proof_shplonk, gen_evm_verifier_shplonk, write_calldata},
    halo2::{
        aggregation::{AggregationCircuit, AggregationConfigParams, VerifierUniversality},
        gen_snark_shplonk, read_snark,
    },
    read_pk,
    snark_verifier::system::halo2::transcript::evm::EvmTranscript,
    CircuitExt, SHPLONK,
};
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
    #[command(subcommand)]
    Agg(AggCommands),
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
        #[arg(short, long, default_value = "./build/app/trusted_setup")]
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
        #[arg(short, long, default_value = "./build/app/trusted_setup")]
        trusted_setup_path: String,
        /// verification key path. input
        #[arg(long, default_value = "./build/app/vk")]
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
    Snark {
        /// trusted setup parameters path. input
        #[arg(short, long, default_value = "./build/app/trusted_setup")]
        trusted_setup_path: String,
        /// proving key path. input
        #[arg(long, default_value = "./build/app/pk")]
        pk_path: String,
        /// partial proof path. output
        #[arg(long, default_value = "./build/app/snark")]
        snark_path: String,
        // citizen's certificate. input
        #[arg(long, default_value = "./certs/myna_cert.pem")]
        verify_cert_path: String,
        // nation's certificate. input
        #[arg(long, default_value = "./certs/ca_cert.pem")]
        issuer_cert_path: String,
        #[arg(default_value = "42")]
        password: u64,
    },
}

#[derive(Debug, Subcommand, Clone)]
enum AggCommands {
    /// Generate a trusted setup paramter
    TrustedSetup {
        /// trusted setup parameters path. input
        #[arg(short, long, default_value = "./build/agg/trusted_setup")]
        trusted_setup_path: String,
    },
    Keys {
        /// trusted setup parameters path. input
        #[arg(short, long, default_value = "./build/agg/trusted_setup")]
        trusted_setup_path: String,
        /// partial proof path. input
        #[arg(long, default_value = "./build/app/snark")]
        snark_path: String,
        /// verification key path. output
        #[arg(long, default_value = "./build/agg/vk")]
        vk_path: String,
        /// proving key path. output
        #[arg(long, default_value = "./build/agg/pk")]
        pk_path: String,
        /// break points path. output
        #[arg(long, default_value = "./build/agg/break_points")]
        break_points_path: String,
    },
    Prove {
        /// trusted setup parameters path. input
        #[arg(short, long, default_value = "./build/agg/trusted_setup")]
        trusted_setup_path: String,
        /// proving key path. input
        #[arg(long, default_value = "./build/agg/pk")]
        pk_path: String,
        /// partial proof path. input
        #[arg(long, default_value = "./build/app/snark")]
        snark_path: String,
        /// break points path. input
        #[arg(long, default_value = "./build/agg/break_points")]
        break_points_path: String,
        /// proof path. output
        #[arg(long, default_value = "./build/agg/proof")]
        proof_path: String,
    },
    Verify {
        /// trusted setup parameters path. input
        #[arg(short, long, default_value = "./build/agg/trusted_setup")]
        trusted_setup_path: String,
        /// verification key path. input
        #[arg(long, default_value = "./build/agg/vk")]
        vk_path: String,
        /// proof path. input
        #[arg(long, default_value = "./build/agg/proof")]
        proof_path: String,
        /// partial proof path. input
        #[arg(long, default_value = "./build/app/snark")]
        snark_path: String,
    },
    Evm {
        /// trusted setup parameters path. input
        #[arg(short, long, default_value = "./build/agg/trusted_setup")]
        trusted_setup_path: String,
        /// verification key path. input
        #[arg(long, default_value = "./build/agg/vk")]
        vk_path: String,
        /// proof path. input
        #[arg(long, default_value = "./build/agg/proof")]
        proof_path: String,
        /// partial proof path. input
        #[arg(long, default_value = "./build/app/snark")]
        snark_path: String,
        /// verifier.sol path. output
        #[arg(short, long, default_value = "./build/agg/verifier.sol")]
        solidity_path: String,
        /// calldata path. output
        #[arg(long, default_value = "./build/agg/calldata.txt")]
        calldata_path: String,
    },
}

const AGGREGATION_CONFIG: AggregationConfigParams =
    AggregationConfigParams { degree: 23, num_advice: 7, num_fixed: 1, num_lookup_advice: 1, lookup_bits: 22 };

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
                let trusted_setup_file = ParamsKZG::<Bn256>::setup(circuit::K as u32, OsRng);
                trusted_setup_file.write(&mut file).expect("Failed to write a trusted setup");
            }
            AppCommands::Keys { trusted_setup_path, pk_path, vk_path } => {
                let circuit = circuit::ProofOfJapaneseResidence::new(
                    "./certs/ca_cert.pem".into(),
                    "./certs/myna_cert.pem".into(),
                    0xA42.into(),
                );

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
                let instance_column = circuit.instance_column();

                let mut trusted_setup_file = File::open(trusted_setup_path).expect("Couldn't open the trusted setup");
                let trusted_setup =
                    ParamsKZG::<Bn256>::read_custom(&mut trusted_setup_file, SerdeFormat::RawBytes).unwrap();

                let pk = read_pk::<circuit::ProofOfJapaneseResidence>(pk_path.as_ref(), circuit.params())
                    .expect("pk not found");

                let mut proof_file = BufWriter::new(File::create(proof_path).unwrap());
                let proof = gen_evm_proof_shplonk(&trusted_setup, &pk, circuit, vec![instance_column]);
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
                let trusted_setup =
                    ParamsKZG::<Bn256>::read_custom(&mut trusted_setup_file, SerdeFormat::RawBytes).unwrap();

                let mut vk_file = File::open(vk_path).expect("vk not found.");
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
                let trusted_setup =
                    ParamsKZG::<Bn256>::read_custom(&mut trusted_setup_file, SerdeFormat::RawBytes).unwrap();

                let mut proof_file = File::open(&proof_path).expect("proof not found.");
                let mut proof: Vec<u8> = Vec::new();
                proof_file.read_to_end(&mut proof).unwrap();

                let mut vk_file = File::open(vk_path).expect("vk not found.");
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
            AppCommands::Snark {
                issuer_cert_path,
                verify_cert_path,
                password,
                trusted_setup_path,
                pk_path,
                snark_path,
            } => {
                let circuit = circuit::ProofOfJapaneseResidence::new(
                    issuer_cert_path.into(),
                    verify_cert_path.into(),
                    password.into(),
                );

                let mut trusted_setup_file = File::open(trusted_setup_path).expect("Couldn't open the trusted setup");
                let trusted_setup =
                    ParamsKZG::<Bn256>::read_custom(&mut trusted_setup_file, SerdeFormat::RawBytes).unwrap();

                let pk = read_pk::<circuit::ProofOfJapaneseResidence>(pk_path.as_ref(), circuit.params())
                    .expect("pk not found.");

                gen_snark_shplonk(&trusted_setup, &pk, circuit, Some(&snark_path));
            }
        },
        Commands::Agg(command) => match command {
            AggCommands::TrustedSetup { trusted_setup_path } => {
                let trusted_setup_path = Path::new(&trusted_setup_path);
                if trusted_setup_path.exists() {
                    println!("Trusted setup already exists. Overwriting...");
                }

                let mut file =
                    BufWriter::new(File::create(trusted_setup_path).expect("Failed to create a trusted setup"));
                let trusted_setup_file = ParamsKZG::<Bn256>::setup(AGGREGATION_CONFIG.degree, OsRng);
                trusted_setup_file.write(&mut file).expect("Failed to write a trusted setup");
            }
            AggCommands::Keys { trusted_setup_path, break_points_path, snark_path, pk_path, vk_path } => {
                let snark = read_snark(&snark_path).expect("proof not found.");

                let mut trusted_setup_file = File::open(trusted_setup_path).expect("Couldn't open the trusted setup");
                let trusted_setup =
                    ParamsKZG::<Bn256>::read_custom(&mut trusted_setup_file, SerdeFormat::RawBytes).unwrap();

                let circuit = AggregationCircuit::new::<SHPLONK>(
                    CircuitBuilderStage::Keygen,
                    AGGREGATION_CONFIG,
                    &trusted_setup,
                    vec![snark],
                    VerifierUniversality::None,
                );

                let vk = keygen_vk(&trusted_setup, &circuit).unwrap();
                let mut vk_file = BufWriter::new(File::create(vk_path).unwrap());
                vk.write(&mut vk_file, SerdeFormat::RawBytes).unwrap();

                let pk = keygen_pk(&trusted_setup, vk, &circuit).unwrap();
                let mut pk_file = BufWriter::new(File::create(pk_path).unwrap());
                pk.write(&mut pk_file, SerdeFormat::RawBytes).unwrap();

                let mut break_points_file = BufWriter::new(File::create(break_points_path).unwrap());
                bincode::serialize_into(&mut break_points_file, &circuit.break_points()).unwrap();
            }
            AggCommands::Prove { trusted_setup_path, pk_path, break_points_path, snark_path, proof_path } => {
                let mut trusted_setup_file = File::open(trusted_setup_path).expect("Couldn't open the trusted setup");
                let trusted_setup =
                    ParamsKZG::<Bn256>::read_custom(&mut trusted_setup_file, SerdeFormat::RawBytes).unwrap();

                let pk = read_pk::<AggregationCircuit>(pk_path.as_ref(), AGGREGATION_CONFIG).expect("pk not found.");
                let snark = read_snark(&snark_path).expect("proof not fonud.");

                let break_points_file = BufReader::new(File::open(break_points_path).expect("break points not found."));
                let break_points = bincode::deserialize_from(break_points_file).unwrap();

                let circuit = AggregationCircuit::new::<SHPLONK>(
                    CircuitBuilderStage::Prover,
                    AGGREGATION_CONFIG,
                    &trusted_setup,
                    vec![snark],
                    VerifierUniversality::None,
                )
                .use_break_points(break_points);
                let instance_columns = circuit.instances();

                let mut proof_file = BufWriter::new(File::create(proof_path).unwrap());
                let proof = gen_evm_proof_shplonk(&trusted_setup, &pk, circuit, instance_columns);
                proof_file.write_all(&proof).unwrap();
            }
            AggCommands::Verify { snark_path, proof_path, trusted_setup_path, vk_path } => {
                let mut trusted_setup_file = File::open(trusted_setup_path).expect("Couldn't open the trusted setup");
                let trusted_setup =
                    ParamsKZG::<Bn256>::read_custom(&mut trusted_setup_file, SerdeFormat::RawBytes).unwrap();

                let snark = read_snark(&snark_path).expect("proof not found.");

                let circuit = AggregationCircuit::new::<SHPLONK>(
                    CircuitBuilderStage::Prover,
                    AGGREGATION_CONFIG,
                    &trusted_setup,
                    vec![snark],
                    VerifierUniversality::None,
                );

                let mut vk_file = File::open(vk_path).expect("vk not found.");
                let vk = VerifyingKey::<G1Affine>::read::<_, AggregationCircuit>(
                    &mut vk_file,
                    SerdeFormat::RawBytes,
                    AGGREGATION_CONFIG,
                )
                .expect("vk not found.");

                let proof_file = File::open(&proof_path).expect("proof not found.");
                let mut proof = TranscriptReadBuffer::<_, _, _>::init(&proof_file);

                let instances = circuit.instances();
                let instance_refs: Vec<&[Fr]> = instances.iter().map(|x| x.as_ref()).collect();

                let result = verify_proof::<_, VerifierSHPLONK<'_, Bn256>, _, EvmTranscript<_, _, _, _>, _>(
                    &trusted_setup,
                    &vk,
                    AccumulatorStrategy::new(&trusted_setup),
                    &[&instance_refs],
                    &mut proof,
                );
                assert!(result.is_ok(), "Verification failed!");
                println!("Verification succeeded!");
            }
            AggCommands::Evm { trusted_setup_path, vk_path, proof_path, snark_path, solidity_path, calldata_path } => {
                let mut trusted_setup_file = File::open(trusted_setup_path).expect("Couldn't open the trusted setup");
                let trusted_setup =
                    ParamsKZG::<Bn256>::read_custom(&mut trusted_setup_file, SerdeFormat::RawBytes).unwrap();

                let snark = read_snark(&snark_path).expect("proof not found.");

                let mut vk_file = File::open(vk_path).expect("vk not found.");
                let vk = VerifyingKey::<G1Affine>::read::<_, AggregationCircuit>(
                    &mut vk_file,
                    SerdeFormat::RawBytes,
                    AGGREGATION_CONFIG,
                )
                .expect("vk not found.");

                let mut proof_file = File::open(&proof_path).expect("proof not found.");
                let mut proof: Vec<u8> = Vec::new();
                proof_file.read_to_end(&mut proof).unwrap();

                let circuit = AggregationCircuit::new::<SHPLONK>(
                    CircuitBuilderStage::Prover,
                    AGGREGATION_CONFIG,
                    &trusted_setup,
                    vec![snark],
                    VerifierUniversality::None,
                );

                write_calldata(&circuit.instances(), &proof, Path::new(&calldata_path)).unwrap();

                let verifier = gen_evm_verifier_shplonk::<AggregationCircuit>(
                    &trusted_setup,
                    &vk,
                    circuit.num_instance(),
                    Some(Path::new(&solidity_path)),
                );
                evm_verify(verifier, circuit.instances(), proof);
            }
        },
    }
}
