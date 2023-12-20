pragma circom 2.1.5;

include "../../../node_modules/circomlib/circuits/poseidon.circom";
include "../../../node_modules/circomlib/circuits/sha256/sha256.circom";
include "../../../node_modules/circomlib/circuits/bitify.circom";
include "./helpers/rsa.circom";

template HashUserSecret() {
    signal input userSecret;
    signal output hashedUserSecretInNum[3];

    component num2Bits = Num2Bits(8);

    num2Bits.in <== userSecret;

    component sha256 = Sha256(8);
    for(var i = 0; i < 8; i++) {
       sha256.in[i] <== num2Bits.out[7 - i];
    }
    // sha256.in <== num2Bits.out;
    signal hashedUserSecret[256] <== sha256.out;

    var intermediate[3][121];
    for (var i = 0; i < 2; i++) {
        for (var j = 0; j < 121; j++) {
            intermediate[i][j] = hashedUserSecret[255 - i * 121 - j];
        }
    }
    for(var i = 0; i < 14; i++) {
        intermediate[2][i] = hashedUserSecret[13 - i];
    }

    component bits2Num2[3];
    for (var i = 0; i < 3; i++) {
        bits2Num2[i] = Bits2Num(121);
        bits2Num2[i].in <== intermediate[i];
        hashedUserSecretInNum[i] <== bits2Num2[i].out;
    }
}

template MainMynaRegistration(n, k) {
    assert(n * k > 2048);
    assert(n < (255 \ 2));

    signal input userSecret;
    signal input modulus[k];
    signal input signature[k];

    var messageLength = (256 + n) \ n;

    component hashUserSecret = HashUserSecret();
    hashUserSecret.userSecret <== userSecret;

    // Verify signature for user secret
    component rsa = RSAVerify65537(n, k);
    rsa.modulus <== modulus;
    rsa.signature <== signature;
    for (var i = 0; i < messageLength; i++) {
        rsa.base_message[i] <== hashUserSecret.hashedUserSecretInNum[i];
    }
    for (var i = messageLength; i < k; i++) {
        rsa.base_message[i] <== 0;
    }

    // Verify modulus hash
    component hashModulus = Poseidon(k - 1);
    component hashModulus2 = Poseidon(2);
    component poseidonHashUserSecret = Poseidon(1);
 
    for (var i = 0; i < k - 1; i++) {
        hashModulus.inputs[i] <== modulus[i];
    }
    hashModulus2.inputs[0] <== hashModulus.out;
    hashModulus2.inputs[1] <== modulus[k - 1];

    signal output hashedModulus <== hashModulus2.out;

    // Verify user secret hash
    poseidonHashUserSecret.inputs[0] <== userSecret;
    signal output hashedUserSecret <== poseidonHashUserSecret.out;
}

component main { public [ signature ] } = MainMynaRegistration(121, 17);