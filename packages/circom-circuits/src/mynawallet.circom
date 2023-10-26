pragma circom 2.1.5;

include "../../../node_modules/circomlib/circuits/poseidon.circom";
include "./helpers/rsa.circom";

// Create a public hashed value from user secret and modulus
template CalculateHash(k) {
    signal input modulus[k];
    signal input userSecret;
    component poseidon = Poseidon(k-1);
    component poseidon2 = Poseidon(3);

    for (var i = 0; i < k-1; i++) {
        poseidon.inputs[i] <== modulus[i];
    }
    poseidon2.inputs[0] <== poseidon.out;
    poseidon2.inputs[1] <== modulus[k-1];
    poseidon2.inputs[2] <== userSecret;

    signal output out <== poseidon2.out;
}

template IsModulusExist() {

}

template MynaWalletVerify(n, k) {
    assert(n * k > 2048); // constraints for 2048 bit RSA
    assert(n < (255 \ 2)); // we want a multiplication to fit into a circom signal

    signal input userSecret; // user secret
    signal input modulus[k]; // rsa public key, verified with smart contract. split up into k parts of n bits each.
    signal input signature[k]; // rsa signature. split up into k parts of n bits each.
    signal input sha256HashedMessage[3]; // rsa message. split up into 3 parts of n bit each.
    signal input TBSCert[k];

    var messageLength = (256 + n) \ n;

    // VERIFY RSA SIGNATURE: 149,251 constraints
    component rsa = RSAVerify65537(n, k);
    rsa.modulus <== modulus;
    rsa.signature <== signature;
    for (var i = 0; i < messageLength; i++) {
        rsa.base_message[i] <== sha256HashedMessage[i];
    }
    for (var i = messageLength; i < k; i++) {
        rsa.base_message[i] <== 0;
    }

    // VERIFY MODULUS IS INCLUDED IN TBS CERT

    // TODO VERIFY RSA SIGNATURE FROM GOVERNMENT

    // CALCULATE NULLIFIER:
    component calculateHash = CalculateHash(k);
    calculateHash.modulus <== modulus;
    calculateHash.userSecret <== userSecret;
    signal output hashed <== calculateHash.out; // poseidon(modulus, userSecret);
}
