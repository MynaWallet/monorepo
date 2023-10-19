init:
	mkdir ./setup && mkdir ./build
	cp ./test/input.json ./setup/input.json
	curl -L https://hermez.s3-eu-west-1.amazonaws.com/powersOfTau28_hez_final_19.ptau -o ./setup/powersOfTau28_hez_final_19.ptau

test-circuit:
	npm run test

build-circuit:
	circom src/main.circom --r1cs --wasm --sym -o ./build
	node build/main_js/generate_witness.js ./build/main_js/main.wasm ./setup/input.json ./setup/witness.wtns

setup-groth16-local:
	snarkjs groth16 setup ./build/main.r1cs ./setup/powersOfTau28_hez_final_19.ptau ./setup/circuit_0000.zkey
	snarkjs zkey contribute ./setup/circuit_0000.zkey ./setup/circuit_0001.zkey --name="1st Contributor Name 1" -v -e="1st random entropy"
	snarkjs zkey contribute ./setup/circuit_0001.zkey ./setup/circuit_0002.zkey --name="2st Contributor Name 2" -v -e="2st random entropy"
	snarkjs zkey contribute ./setup/circuit_0002.zkey ./setup/circuit_0003.zkey --name="3st Contributor Name 3" -v -e="3st random entropy"
	snarkjs zkey verify ./build/main.r1cs ./setup/powersOfTau28_hez_final_19.ptau ./setup/circuit_0003.zkey
	snarkjs zkey beacon ./setup/circuit_0003.zkey ./setup/circuit_final.zkey 0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f 10 -n="Final Beacon phase2"
	snarkjs zkey verify ./build/main.r1cs ./setup/powersOfTau28_hez_final_19.ptau ./setup/circuit_final.zkey
	snarkjs zkey export verificationkey ./setup/circuit_final.zkey ./setup/verification_key.json

create-proof:
	snarkjs groth16 prove ./setup/circuit_final.zkey ./setup/witness.wtns ./setup/proof.json ./setup/public.json

verify-proof:
	snarkjs groth16 verify ./setup/verification_key.json ./setup/public.json ./setup/proof.json

export-verifier:
	snarkjs zkey export solidityverifier ./setup/circuit_final.zkey ../contracts/src/circom-verifier/verifier.sol

generate-calldata:
	snarkjs generatecall ./setup/public.json ./setup/proof.json
