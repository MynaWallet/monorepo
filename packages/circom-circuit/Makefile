init:
	mkdir -p ./setup && mkdir -p ./build
	cp ./test/input.json ./setup/input.json
	curl -L https://hermez.s3-eu-west-1.amazonaws.com/powersOfTau28_hez_final_20.ptau -o ./setup/powersOfTau28_hez_final_20.ptau

test-circuit:
	npm run test

gov-sig-build-circuit:
	circom src/verify-gov-sig.circom --r1cs --wasm --sym -o ./gov-sig-build
	node gov-sig-build/verify-gov-sig_js/generate_witness.js ./gov-sig-build/verify-gov-sig_js/verify-gov-sig.wasm ./gov-sig-setup/input.json ./gov-sig-setup/witness.wtns

user-sig-build-circuit:
	circom src/verify-user-sig.circom --r1cs --wasm --sym -o ./user-sig-build
	node user-sig-build/verify-user-sig_js/generate_witness.js ./user-sig-build/verify-user-sig_js/verify-user-sig.wasm ./user-sig-setup/input.json ./user-sig-setup/witness.wtns

gov-sig-setup-groth16:
	snarkjs groth16 setup ./gov-sig-build/verify-gov-sig.r1cs ./setup/powersOfTau28_hez_final_20.ptau ./gov-sig-setup/circuit_0000.zkey
	snarkjs zkey contribute ./gov-sig-setup/circuit_0000.zkey ./gov-sig-setup/circuit_0001.zkey --name="1st Contributor Name 1" -v -e="1st random entropy"
	snarkjs zkey contribute ./gov-sig-setup/circuit_0001.zkey ./gov-sig-setup/circuit_0002.zkey --name="2st Contributor Name 2" -v -e="2st random entropy"
	snarkjs zkey contribute ./gov-sig-setup/circuit_0002.zkey ./gov-sig-setup/circuit_0003.zkey --name="3st Contributor Name 3" -v -e="3st random entropy"
	snarkjs zkey verify ./gov-sig-build/verify-gov-sig.r1cs ./setup/powersOfTau28_hez_final_20.ptau ./gov-sig-setup/circuit_0003.zkey
	snarkjs zkey beacon ./gov-sig-setup/circuit_0003.zkey ./gov-sig-setup/circuit_final.zkey 0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f 10 -n="Final Beacon phase2"
	snarkjs zkey verify ./gov-sig-build/verify-gov-sig.r1cs ./setup/powersOfTau28_hez_final_20.ptau ./gov-sig-setup/circuit_final.zkey
	snarkjs zkey export verificationkey ./gov-sig-setup/circuit_final.zkey ./gov-sig-setup/verification_key.json

user-sig-setup-groth16:
	snarkjs groth16 setup ./user-sig-build/verify-user-sig.r1cs ./setup/powersOfTau28_hez_final_20.ptau ./user-sig-setup/circuit_0000.zkey
	snarkjs zkey contribute ./user-sig-setup/circuit_0000.zkey ./user-sig-setup/circuit_0001.zkey --name="1st Contributor Name 1" -v -e="1st random entropy"
	snarkjs zkey contribute ./user-sig-setup/circuit_0001.zkey ./user-sig-setup/circuit_0002.zkey --name="2st Contributor Name 2" -v -e="2st random entropy"
	snarkjs zkey contribute ./user-sig-setup/circuit_0002.zkey ./user-sig-setup/circuit_0003.zkey --name="3st Contributor Name 3" -v -e="3st random entropy"
	snarkjs zkey verify ./user-sig-build/verify-user-sig.r1cs ./setup/powersOfTau28_hez_final_20.ptau ./user-sig-setup/circuit_0003.zkey
	snarkjs zkey beacon ./user-sig-setup/circuit_0003.zkey ./user-sig-setup/circuit_final.zkey 0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f 10 -n="Final Beacon phase2"
	snarkjs zkey verify ./user-sig-build/verify-user-sig.r1cs ./setup/powersOfTau28_hez_final_20.ptau ./user-sig-setup/circuit_final.zkey
	snarkjs zkey export verificationkey ./user-sig-setup/circuit_final.zkey ./user-sig-setup/verification_key.json

gov-sig-create-proof:
	snarkjs groth16 prove ./gov-sig-setup/circuit_final.zkey ./gov-sig-setup/witness.wtns ./gov-sig-setup/proof.json ./gov-sig-setup/public.json

user-sig-create-proof:
	snarkjs groth16 prove ./user-sig-setup/circuit_final.zkey ./user-sig-setup/witness.wtns ./user-sig-setup/proof.json ./user-sig-setup/public.json

gov-sig-verify-proof:
	snarkjs groth16 verify ./gov-sig-setup/verification_key.json ./gov-sig-setup/public.json ./gov-sig-setup/proof.json

user-sig-verify-proof:
	snarkjs groth16 verify ./user-sig-setup/verification_key.json ./user-sig-setup/public.json ./user-sig-setup/proof.json

gov-sig-export-verifier:
	snarkjs zkey export solidityverifier ./gov-sig-setup/circuit_final.zkey ../contracts/src/circom-verifier/govSigVerifier.sol

user-sig-export-verifier:
	snarkjs zkey export solidityverifier ./user-sig-setup/circuit_final.zkey ../contracts/src/circom-verifier/userSigVerifier.sol

gov-sig-generate-calldata:
	snarkjs generatecall ./gov-sig-setup/public.json ./gov-sig-setup/proof.json >> ./gov-sig-build/solidity-input.json

user-sig-generate-calldata:
	snarkjs generatecall ./user-sig-setup/public.json ./user-sig-setup/proof.json >> ./user-sig-build/solidity-input.json
