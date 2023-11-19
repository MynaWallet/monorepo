use clap::{Parser, Subcommand};
use halo2_base::{
    gates::circuit::builder::BaseCircuitBuilder,
    halo2_proofs::{
        halo2curves::bn256::{Bn256, Fr, G1Affine},
        plonk::{create_proof, keygen_pk, keygen_vk, verify_proof, Circuit, ProvingKey, VerifyingKey},
        poly::{
            commitment::Params,
            kzg::{
                commitment::{KZGCommitmentScheme, ParamsKZG},
                multiopen::{ProverGWC, VerifierGWC},
                strategy::SingleStrategy,
            },
        },
        transcript::{Blake2bRead, Blake2bWrite, Challenge255, TranscriptReadBuffer, TranscriptWriterBuffer},
        SerdeFormat,
    },
    utils::{fs::gen_srs, BigPrimeField},
};
use halo2_circuits::{circuit, helpers::*};
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};
use std::{
    fmt::Binary,
    fs::File,
    io::{Read, Write},
};
// use snark_verifier_sdk::
// use snark_verifier_sdk::{
//     evm::{gen_evm_proof_shplonk, gen_evm_verifier_shplonk, write_calldata},
//     gen_pk,
//     halo2::gen_snark_shplonk,
//     read_pk, CircuitExt,
// };
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
        /// trusted setup parameters path. input
        #[arg(short, long, default_value = "./build/trusted_setup")]
        trusted_setup_path: String,
    },
    /// Generate the proving key and the verification key for RSA circuit
    GenerateKeys {
        /// trusted setup parameters path. input
        #[arg(short, long, default_value = "./build/trusted_setup")]
        trusted_setup_path: String,
        /// proving key path. output
        #[arg(long, default_value = "./build/vk")]
        vk_path: String,
        /// proving key path. output
        #[arg(long, default_value = "./build/pk")]
        pk_path: String,
        // citizen's certificate
        #[arg(long, default_value = "./certs/myna_cert.pem")]
        verify_cert_path: String,
        // nation's certificate
        #[arg(long, default_value = "./certs/ca_cert.pem")]
        issuer_cert_path: String,
        #[arg(default_value = "42")]
        password: u64,
    },
    Prove {
        /// trusted setup parameters path. input
        #[arg(short, long, default_value = "./build/trusted_setup")]
        trusted_setup_path: String,
        /// proving key path. output
        #[arg(long, default_value = "./build/pk")]
        pk_path: String,
        /// proof path. output
        #[arg(long, default_value = "./build/proof")]
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
        #[arg(long, default_value = "./build/vk")]
        vk_path: String,
        /// proof path. output
        #[arg(long, default_value = "./build/proof")]
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
    /// Generate the proving key and the verification key for RSA circuit
    GenerateSolidity {
        /// trusted setup parameters path. input
        #[arg(short, long, default_value = "./build/trusted_setup")]
        trusted_setup_path: String,
        /// proving key path. output
        #[arg(long, default_value = "./build/vk")]
        vk_path: String,
        /// proof path. output
        #[arg(long, default_value = "./build/myna_verify_rsa.proof")]
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
        Commands::GenerateKeys {
            trusted_setup_path,
            verify_cert_path,
            issuer_cert_path,
            password,
            vk_path,
            pk_path,
        } => {
            let nation_pubkey = read_nation_cert(&issuer_cert_path);
            let (nation_sig, tbs_cert, _citizen_pubkey) = read_citizen_cert(&verify_cert_path);

            let mut builder = BaseCircuitBuilder::new(false);
            builder.set_k(circuit::K as usize);
            builder.set_lookup_bits(circuit::LOOKUP_BITS);
            builder.set_instance_columns(1);
            let range_chip = builder.range_chip();
            let ctx = builder.main(0);

            let mut sha256ed = Sha256::digest(tbs_cert.to_bytes_le());
            sha256ed.reverse();
            let mut buf = [0; 32];
            buf[0..16].copy_from_slice(&sha256ed[0..16]);
            let sha256lo = Fr::from_bytes(&buf).unwrap();
            buf[0..16].copy_from_slice(&sha256ed[16..32]);
            let sha256hi = Fr::from_bytes(&buf).unwrap();

            let public_input = circuit::PublicInput { sha256: [sha256lo, sha256hi], nation_pubkey };
            let private_input = circuit::PrivateInput { nation_sig, password: Fr::from(password) };
            let public_output = circuit::proof_of_japanese_residence(ctx, range_chip, public_input, private_input);
            builder.assigned_instances[0].extend(public_output);

            let circuit_shape = builder.calculate_params(None);
            let circuit = circuit::ProofOfJapaneseResidence {
                halo2base: builder.use_params(circuit_shape),
                tbs_cert: tbs_cert.to_bytes_le(),
            };

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
        Commands::Prove { verify_cert_path, issuer_cert_path, password, trusted_setup_path, pk_path, proof_path } => {
            let nation_pubkey = read_nation_cert(&issuer_cert_path);
            let (nation_sig, tbs_cert, _citizen_pubkey) = read_citizen_cert(&verify_cert_path);

            let mut builder = BaseCircuitBuilder::new(false);
            builder.set_k(circuit::K as usize);
            builder.set_lookup_bits(circuit::LOOKUP_BITS);
            builder.set_instance_columns(1);
            let range_chip = builder.range_chip();
            let ctx = builder.main(0);

            let mut sha256ed = Sha256::digest(tbs_cert.to_bytes_le());
            sha256ed.reverse();
            let mut buf = [0; 32];
            buf[0..16].copy_from_slice(&sha256ed[0..16]);
            let sha256lo = Fr::from_bytes(&buf).unwrap();
            buf[0..16].copy_from_slice(&sha256ed[16..32]);
            let sha256hi = Fr::from_bytes(&buf).unwrap();

            let public_input = circuit::PublicInput { sha256: [sha256lo, sha256hi], nation_pubkey };
            let private_input = circuit::PrivateInput { nation_sig, password: Fr::from(password) };
            let public_output = circuit::proof_of_japanese_residence(ctx, range_chip, public_input, private_input);
            builder.assigned_instances[0].extend(public_output);

            let circuit_shape = builder.calculate_params(None);
            let circuit = circuit::ProofOfJapaneseResidence {
                halo2base: builder.use_params(circuit_shape),
                tbs_cert: tbs_cert.to_bytes_le(),
            };

            let instance_columns: Vec<Vec<Fr>> = circuit
                .halo2base
                .assigned_instances
                .iter()
                .map(|public_column| public_column.into_iter().map(|public_cell| public_cell.value().clone()).collect())
                .collect();
            let instance_columns: Vec<&[Fr]> =
                instance_columns.iter().map(|instance_column| instance_column.as_slice()).collect();

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

            let proof_file = File::create(proof_path).unwrap();
            println!("Proof generation started at: {:?}", std::time::Instant::now());
            let mut proof = Blake2bWrite::<_, _, Challenge255<_>>::init(proof_file);
            create_proof::<
                KZGCommitmentScheme<Bn256>,
                ProverGWC<'_, Bn256>,
                Challenge255<G1Affine>,
                _,
                Blake2bWrite<File, G1Affine, Challenge255<_>>,
                _,
            >(&trusted_setup, &pk, &[circuit], &[&instance_columns], OsRng, &mut proof)
            .expect("prover should not fail");
            proof.finalize();
            println!("Proof generation finished at: {:?}", std::time::Instant::now());
        }
        Commands::Verify { trusted_setup_path, vk_path, proof_path, verify_cert_path, issuer_cert_path, password } => {
            let nation_pubkey = read_nation_cert(&issuer_cert_path);
            let (nation_sig, tbs_cert, _citizen_pubkey) = read_citizen_cert(&verify_cert_path);

            let mut builder = BaseCircuitBuilder::new(false);
            builder.set_k(circuit::K as usize);
            builder.set_lookup_bits(circuit::LOOKUP_BITS);
            builder.set_instance_columns(1);
            let range_chip = builder.range_chip();
            let ctx = builder.main(0);

            let mut sha256ed = Sha256::digest(tbs_cert.to_bytes_le());
            sha256ed.reverse();
            let mut buf = [0; 32];
            buf[0..16].copy_from_slice(&sha256ed[0..16]);
            let sha256lo = Fr::from_bytes(&buf).unwrap();
            buf[0..16].copy_from_slice(&sha256ed[16..32]);
            let sha256hi = Fr::from_bytes(&buf).unwrap();

            let public_input = circuit::PublicInput { sha256: [sha256lo, sha256hi], nation_pubkey };
            let private_input = circuit::PrivateInput { nation_sig, password: Fr::from(password) };
            let public_output = circuit::proof_of_japanese_residence(ctx, range_chip, public_input, private_input);
            builder.assigned_instances[0].extend(public_output);

            let circuit_shape = builder.calculate_params(None);
            let circuit = circuit::ProofOfJapaneseResidence {
                halo2base: builder.use_params(circuit_shape),
                tbs_cert: tbs_cert.to_bytes_le(),
            };

            let instance_columns: Vec<Vec<Fr>> = circuit
                .halo2base
                .assigned_instances
                .iter()
                .map(|public_column| public_column.into_iter().map(|public_cell| public_cell.value().clone()).collect())
                .collect();
            let instance_columns: Vec<&[Fr]> =
                instance_columns.iter().map(|instance_column| instance_column.as_slice()).collect();

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

            let proof_file = File::open(proof_path).unwrap();
            let mut proof = Blake2bRead::init(&proof_file);

            let result = verify_proof::<
                KZGCommitmentScheme<Bn256>,
                VerifierGWC<'_, Bn256>,
                Challenge255<G1Affine>,
                Blake2bRead<&File, G1Affine, Challenge255<G1Affine>>,
                SingleStrategy<'_, Bn256>,
            >(
                &trusted_setup, &vk, SingleStrategy::new(&trusted_setup), &[&instance_columns], &mut proof
            );
            assert!(result.is_ok(), "{:?}", result)
        }
        Commands::GenerateSolidity {
            trusted_setup_path,
            vk_path,
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
