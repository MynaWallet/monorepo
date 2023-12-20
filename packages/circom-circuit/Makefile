init:
	mkdir ./setup && mkdir ./build
	cp ./test/input.json ./setup/input.json
	curl -L https://hermez.s3-eu-west-1.amazonaws.com/powersOfTau28_hez_final_19.ptau -o ./setup/powersOfTau28_hez_final_19.ptau

test-circuit:
	npm run test

build-main-registration-circuit:
	circom src/main_myna_registration.circom --r1cs --wasm --sym -o ./build/main_myna_registration
	node ./build/main_myna_registration/main_myna_registration_js/generate_witness.js ./build/main_myna_registration/main_myna_registration_js/main_myna_registration.wasm ./setup/main_myna_registration/input.json ./setup/main_myna_registration/witness.wtns

build-main-inclusion-circuit:
	circom src/main_myna_inclusion.circom --r1cs --wasm --sym -o ./build/main_myna_inclusion
	node ./build/main_myna_inclusion/main_myna_inclusion_js/generate_witness.js ./build/main_myna_inclusion/main_myna_inclusion_js/main_myna_inclusion.wasm ./setup/main_myna_inclusion/input.json ./setup/main_myna_inclusion/witness.wtns

build-gov-sig-circuit:
	circom src/verify_gov_sig.circom --r1cs --wasm --sym -o ./build/gov_sig
	node ./build/gov_sig/verify_gov_sig_js/generate_witness.js ./build/gov_sig/verify_gov_sig_js/verify_gov_sig.wasm ./setup/gov_sig/input.json ./setup/gov_sig/witness.wtns

build-user-sig-circuit:
	circom src/verify_user_sig.circom --r1cs --wasm --sym -o ./build/user_sig
	node ./build/user_sig/verify_user_sig_js/generate_witness.js ./build/user_sig/verify_user_sig_js/verify_user_sig.wasm ./setup/user_sig/input.json ./setup/user_sig/witness.wtns

setup-main-registration-groth16:
	snarkjs groth16 setup ./build/main_myna_registration/main_myna_registration.r1cs ./powersOfTau28_hez_final_20.ptau ./setup/main_myna_registration/circuit_0000.zkey
	snarkjs zkey contribute ./setup/main_myna_registration/circuit_0000.zkey ./setup/main_myna_registration/circuit_0001.zkey --name="1st Contributor Name 1" -v -e="1st random entropy"
	snarkjs zkey contribute ./setup/main_myna_registration/circuit_0001.zkey ./setup/main_myna_registration/circuit_0002.zkey --name="2st Contributor Name 2" -v -e="2st random entropy"
	snarkjs zkey contribute ./setup/main_myna_registration/circuit_0002.zkey ./setup/main_myna_registration/circuit_0003.zkey --name="3st Contributor Name 3" -v -e="3st random entropy"
	snarkjs zkey verify ./build/main_myna_registration/main_myna_registration.r1cs ./powersOfTau28_hez_final_20.ptau ./setup/main_myna_registration/circuit_0003.zkey
	snarkjs zkey beacon ./setup/main_myna_registration/circuit_0003.zkey ./setup/main_myna_registration/circuit_final.zkey 0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f 10 -n="Final Beacon phase2"
	snarkjs zkey verify ./build/main_myna_registration/main_myna_registration.r1cs ./powersOfTau28_hez_final_20.ptau ./setup/main_myna_registration/circuit_final.zkey
	snarkjs zkey export verificationkey ./setup/main_myna_registration/circuit_final.zkey ./setup/main_myna_registration/verification_key.json

setup-main-inclusion-groth16:
	snarkjs groth16 setup ./build/main_myna_inclusion/main_myna_inclusion.r1cs ./powersOfTau28_hez_final_20.ptau ./setup/main_myna_inclusion/circuit_0000.zkey
	snarkjs zkey contribute ./setup/main_myna_inclusion/circuit_0000.zkey ./setup/main_myna_inclusion/circuit_0001.zkey --name="1st Contributor Name 1" -v -e="1st random entropy"
	snarkjs zkey contribute ./setup/main_myna_inclusion/circuit_0001.zkey ./setup/main_myna_inclusion/circuit_0002.zkey --name="2st Contributor Name 2" -v -e="2st random entropy"
	snarkjs zkey contribute ./setup/main_myna_inclusion/circuit_0002.zkey ./setup/main_myna_inclusion/circuit_0003.zkey --name="3st Contributor Name 3" -v -e="3st random entropy"
	snarkjs zkey verify ./build/main_myna_inclusion/main_myna_inclusion.r1cs ./powersOfTau28_hez_final_20.ptau ./setup/main_myna_inclusion/circuit_0003.zkey
	snarkjs zkey beacon ./setup/main_myna_inclusion/circuit_0003.zkey ./setup/main_myna_inclusion/circuit_final.zkey 0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f 10 -n="Final Beacon phase2"
	snarkjs zkey verify ./build/main_myna_inclusion/main_myna_inclusion.r1cs ./powersOfTau28_hez_final_20.ptau ./setup/main_myna_inclusion/circuit_final.zkey
	snarkjs zkey export verificationkey ./setup/main_myna_inclusion/circuit_final.zkey ./setup/main_myna_inclusion/verification_key.json

setup-gov-sig-groth16:
	snarkjs groth16 setup ./build/gov_sig/verify_gov_sig.r1cs ./powersOfTau28_hez_final_20.ptau ./setup/gov_sig/circuit_0000.zkey
	snarkjs zkey contribute ./setup/gov_sig/circuit_0000.zkey ./setup/gov_sig/circuit_0001.zkey --name="1st Contributor Name 1" -v -e="1st random entropy"
	snarkjs zkey contribute ./setup/gov_sig/circuit_0001.zkey ./setup/gov_sig/circuit_0002.zkey --name="2st Contributor Name 2" -v -e="2st random entropy"
	snarkjs zkey contribute ./setup/gov_sig/circuit_0002.zkey ./setup/gov_sig/circuit_0003.zkey --name="3st Contributor Name 3" -v -e="3st random entropy"
	snarkjs zkey verify ./build/gov_sig/verify_gov_sig.r1cs ./powersOfTau28_hez_final_20.ptau ./setup/gov_sig/circuit_0003.zkey
	snarkjs zkey beacon ./setup/gov_sig/circuit_0003.zkey ./setup/gov_sig/circuit_final.zkey 0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f 10 -n="Final Beacon phase2"
	snarkjs zkey verify ./build/gov_sig/verify_gov_sig.r1cs ./powersOfTau28_hez_final_20.ptau ./setup/gov_sig/circuit_final.zkey
	snarkjs zkey export verificationkey ./setup/gov_sig/circuit_final.zkey ./setup/gov_sig/verification_key.json

setup-user-sig-groth16:
	snarkjs groth16 setup ./build/user_sig/verify_user_sig.r1cs ./powersOfTau28_hez_final_20.ptau ./setup/user_sig/circuit_0000.zkey
	snarkjs zkey contribute ./setup/user_sig/circuit_0000.zkey ./setup/user_sig/circuit_0001.zkey --name="1st Contributor Name 1" -v -e="1st random entropy"
	snarkjs zkey contribute ./setup/user_sig/circuit_0001.zkey ./setup/user_sig/circuit_0002.zkey --name="2st Contributor Name 2" -v -e="2st random entropy"
	snarkjs zkey contribute ./setup/user_sig/circuit_0002.zkey ./setup/user_sig/circuit_0003.zkey --name="3st Contributor Name 3" -v -e="3st random entropy"
	snarkjs zkey verify ./build/user_sig/verify_user_sig.r1cs ./powersOfTau28_hez_final_20.ptau ./setup/user_sig/circuit_0003.zkey
	snarkjs zkey beacon ./setup/user_sig/circuit_0003.zkey ./setup/user_sig/circuit_final.zkey 0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f 10 -n="Final Beacon phase2"
	snarkjs zkey verify ./build/user_sig/verify_user_sig.r1cs ./powersOfTau28_hez_final_20.ptau ./setup/user_sig/circuit_final.zkey
	snarkjs zkey export verificationkey ./setup/user_sig/circuit_final.zkey ./setup/user_sig/verification_key.json

create-main-registration-proof:
	snarkjs groth16 prove ./setup/main_myna_registration/circuit_final.zkey ./setup/main_myna_registration/witness.wtns ./setup/main_myna_registration/proof.json ./setup/main_myna_registration/public.json

create-main-inclusion-proof:
	snarkjs groth16 prove ./setup/main_myna_inclusion/circuit_final.zkey ./setup/main_myna_inclusion/witness.wtns ./setup/main_myna_inclusion/proof.json ./setup/main_myna_inclusion/public.json

create-gov-sig-proof:
	snarkjs groth16 prove ./setup/gov_sig/circuit_final.zkey ./setup/gov_sig/witness.wtns ./setup/gov_sig/proof.json ./setup/gov_sig/public.json

create-user-sig-proof:
	snarkjs groth16 prove ./setup/user_sig/circuit_final.zkey ./setup/user_sig/witness.wtns ./setup/user_sig/proof.json ./setup/user_sig/public.json

verify-main-registration-proof:
	snarkjs groth16 verify ./setup/main_myna_registration/verification_key.json ./setup/main_myna_registration/public.json ./setup/main_myna_registration/proof.json

verify-main-inclusion-proof:
	snarkjs groth16 verify ./setup/main_myna_inclusion/verification_key.json ./setup/main_myna_inclusion/public.json ./setup/main_myna_inclusion/proof.json

verify-gov-sig-proof:
	snarkjs groth16 verify ./setup/gov_sig/verification_key.json ./setup/gov_sig/public.json ./setup/gov_sig/proof.json

verify-user-sig-proof:
	snarkjs groth16 verify ./setup/user_sig/verification_key.json ./setup/user_sig/public.json ./setup/user_sig/proof.json

export-main-registration-verifier:
	snarkjs zkey export solidityverifier ./setup/main_myna_registration/circuit_final.zkey ../extention-contracts/src/circom-verifier/MainMynaRegistraionVerifier.sol

export-main-inclusion-verifier:
	snarkjs zkey export solidityverifier ./setup/main_myna_inclusion/circuit_final.zkey ../extention-contracts/src/circom-verifier/MainMynaInclusionVerifier.sol

export-gov-sig-verifier:
	snarkjs zkey export solidityverifier ./setup/gov_sig/circuit_final.zkey ../contracts/src/circom-verifier/GovSigVerifier.sol

export-user-sig-verifier:
	snarkjs zkey export solidityverifier ./setup/user_sig/circuit_final.zkey ../contracts/src/circom-verifier/UserSigVerifier.sol

generate-main-registration-calldata:
	snarkjs generatecall ./setup/main_myna_registration/public.json ./setup/main_myna_registration/proof.json >> ./build/main_myna_registration/solidity_input.json

generate-main-inclusion-calldata:
	snarkjs generatecall ./setup/main_myna_inclusion/public.json ./setup/main_myna_inclusion/proof.json >> ./build/main_myna_inclusion/solidity_input.json

generate-gov-sig-calldata:
	snarkjs generatecall ./setup/gov_sig/public.json ./setup/gov_sig/proof.json >> ./build/gov_sig/solidity_input.json

generate-user-sig-calldata:
	snarkjs generatecall ./setup/user_sig/public.json ./setup/user_sig/proof.json >> ./build/user_sig/solidity_input.json