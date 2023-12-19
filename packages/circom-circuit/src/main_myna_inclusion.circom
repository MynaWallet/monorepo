pragma circom 2.1.5;

include "../../../node_modules/circomlib/circuits/poseidon.circom";
include "./helpers/MerkleInclusionProof.circom";

template CalculateHash(k) {
    signal input modulus[k];
    signal input userSecret;
    component poseidon = Poseidon(k-1);
    component poseidon2 = Poseidon(3);

    for (var i = 0; i < k - 1; i++) {
        poseidon.inputs[i] <== modulus[i];
    }
    poseidon2.inputs[0] <== poseidon.out;
    poseidon2.inputs[1] <== modulus[k-1];
    poseidon2.inputs[2] <== userSecret;

    signal output out <== poseidon2.out;
}

template MainMynaInclusion(nLevels, k) {
    signal input userSecret;
    signal input modulus[k];
    signal input treePathIndices[nLevels];
    signal input treeSiblings[nLevels];

    component calculateHash = CalculateHash(k);
    calculateHash.modulus <== modulus;
    calculateHash.userSecret <== userSecret;
    signal output identityCommitment <== calculateHash.out;

    component hashUserSecret = Poseidon(1);
    component hashModulus = Poseidon(k - 1);
    component hashModulus2 = Poseidon(2);

    hashUserSecret.inputs[0] <== userSecret; 
    for (var i = 0; i < k - 1; i++) {
        hashModulus.inputs[i] <== modulus[i];
    }
    hashModulus2.inputs[0] <== hashModulus.out;
    hashModulus2.inputs[1] <== modulus[k - 1];

    component poseidon = Poseidon(2);
    poseidon.inputs[0] <== hashUserSecret.out;
    poseidon.inputs[1] <== hashModulus2.out;

    signal leaf <== poseidon.out;

    component mtp = MerkleTreeInclusionProof(nLevels);
    mtp.leaf <== leaf;
    for (var i = 0; i < nLevels; i++){
        mtp.pathIndices[i] <== treePathIndices[i];
        mtp.siblings[i] <== treeSiblings[i];
    }
    signal output root <== mtp.root;
}

component main = MainMynaInclusion(15, 17);