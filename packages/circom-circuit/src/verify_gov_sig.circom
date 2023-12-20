pragma circom 2.1.5;

include "../../../node_modules/circomlib/circuits/poseidon.circom";
include "./helpers/rsa.circom";
include "../../../node_modules/circomlib/circuits/bitify.circom";
include "../../../node_modules/circomlib/circuits/sha256/sha256.circom";

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

template VerifyInclusion() {
    signal input tbsCert[1600];
    signal input modulus[17];

    component num2Bits0[256];
    var pubKeyInSepBits[256][8];
    var pubKeyInBits[2048];
    for (var i = 0; i < 256; i++) {
        num2Bits0[i] = Num2Bits(8);
        num2Bits0[i].in <== tbsCert[281 + i];
        pubKeyInSepBits[i] = num2Bits0[i].out;
    }
    for (var i = 0; i < 256; i++) {
        for (var j = 0; j < 8; j++) {
            pubKeyInBits[i * 8 + j] = pubKeyInSepBits[i][7 - j];
        }
    }

    var b[17][121];

    for (var i = 0; i < 16; i++) {
        for (var j = 0; j < 121; j++) {
            b[i][j] = pubKeyInBits[2047 - i * 121 - j];
        }
    }
    for (var i = 0; i < 112 ; i++) {
        b[16][i] = pubKeyInBits[111 - i];
    }

    component bits2Num[17];
    var num[17];
    for (var i = 0; i < 17; i++) {
        bits2Num[i] = Bits2Num(121);
        bits2Num[i].in <== b[i];
        num[i] = bits2Num[i].out;
    }

    for (var i = 0; i < 17; i++) {
        num[i] === modulus[i];
    }
}

template HashTbs() {
    signal input tbsCert[1600];
    signal output hashedTbsInNum[3];

    component num2Bits[1306];

    var tbsInSepBits[1306][8];
    var tbsInBits[10448];
    for (var i = 0; i < 1306; i++) {
        num2Bits[i] = Num2Bits(8);
        num2Bits[i].in <== tbsCert[4 + i];
        tbsInSepBits[i] = num2Bits[i].out;
    }
    for (var i = 0; i < 1306; i++) {
        for (var j = 0; j < 8; j++) {
            tbsInBits[i * 8 + j] = tbsInSepBits[i][7 - j];
        }
    }

    component sha256 = Sha256(10448);
    sha256.in <== tbsInBits;
    var hashedTbs[256] = sha256.out;

    var intermediate[3][121];
    for (var i = 0; i < 2; i++) {
        for (var j = 0; j < 121; j++) {
            intermediate[i][j] = hashedTbs[255 - i * 121 - j];
        }
    }
    for(var i = 0; i < 14; i++) {
        intermediate[2][i] = hashedTbs[13 - i];
    }

    component bits2Num2[3];
    for (var i = 0; i < 3; i++) {
        bits2Num2[i] = Bits2Num(121);
        bits2Num2[i].in <== intermediate[i];
        hashedTbsInNum[i] <== bits2Num2[i].out;
    }
}

template MynaVerifyGovSig(n, k) {
    assert(n * k > 2048); // constraints for 2048 bit RSA
    assert(n < (255 \ 2)); // we want a multiplication to fit into a circom signal

    signal input userSecret; // user secret
    signal input modulus[k]; // rsa public key, verified with smart contract. split up into k parts of n bits each.
    signal input govModulus[k];
    signal input govSignature[k];
    signal input tbsCert[1600];

    var messageLength = (256 + n) \ n;

    // // VERIFY MODULUS IS INCLUDED IN TBS CERT
    component verifyInclusion = VerifyInclusion();
    verifyInclusion.tbsCert <== tbsCert;
    verifyInclusion.modulus <== modulus;

    // // // HASH TBS CERT
    component hashTbs = HashTbs();
    hashTbs.tbsCert <== tbsCert;

    // // RSA VERIFICATION FOR TBS CERT
    component rsa2 = RSAVerify65537(n, k);
    rsa2.modulus <== govModulus;
    rsa2.signature <== govSignature;
    for (var i = 0; i < messageLength; i++) {
        rsa2.base_message[i] <== hashTbs.hashedTbsInNum[i];
    }
    for (var i = messageLength; i < k; i++) {
        rsa2.base_message[i] <== 0;
    }

    // CALsCULATE NULLIFIER:
    component calculateHash = CalculateHash(k);
    calculateHash.modulus <== modulus;
    calculateHash.userSecret <== userSecret;
    signal output hashed <== calculateHash.out; // poseidon(modulus, userSecret);
}

component main { public [ govModulus, govSignature ] } = MynaVerifyGovSig(121, 17);
