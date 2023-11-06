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

template MynaWalletVerify(n, k) {
    assert(n * k > 2048); // constraints for 2048 bit RSA
    assert(n < (255 \ 2)); // we want a multiplication to fit into a circom signal

    signal input userSecret; // user secret
    signal input modulus[k]; // rsa public key, verified with smart contract. split up into k parts of n bits each.
    signal input signature[k]; // rsa signature. split up into k parts of n bits each.
    signal input sha256HashedMessage[3]; // rsa message. split up into 3 parts of n bit each.
    signal input tbsCert[1586];
    signal input a[8];

    var messageLength = (256 + n) \ n;

    // VERIFY RSA SIGNATURE: 149,251 constraints
    // component rsa = RSAVerify65537(n, k);
    // rsa.modulus <== modulus;
    // rsa.signature <== signature;
    // for (var i = 0; i < messageLength; i++) {
    //     rsa.base_message[i] <== sha256HashedMessage[i];
    // }
    // for (var i = messageLength; i < k; i++) {
    //     rsa.base_message[i] <== 0;
    // }

    // // // VERIFY MODULUS IS INCLUDED IN TBS CERT
    // component num2Bits0[256];
    // var pubKeyInSepBits[256][8];
    // var pubKeyInBits[2048];
    // for (var i = 0; i < 256; i++) {
    //     num2Bits0[i] = Num2Bits(8);
    //     num2Bits0[i].in <== tbsCert[281 + i];
    //     pubKeyInSepBits[i] = num2Bits0[i].out;
    // }
    // for (var i = 0; i < 256; i++) {
    //     for (var j = 0; j < 8; j++) {
    //         pubKeyInBits[i * 8 + j] = pubKeyInSepBits[i][7 - j];
    //     }
    // }

    // var b[17][121];

    // for (var i = 0; i < 16; i++) {
    //     for (var j = 0; j < 121; j++) {
    //         b[i][j] = pubKeyInBits[2047 - i * 121 - j];
    //     }
    // }
    // for (var i = 0; i < 112 ; i++) {
    //     b[16][i] = pubKeyInBits[111 - i];
    // }

    // component bits2Num[17];

    // var num[17];
    // for (var i = 0; i < 17; i++) {
    //     bits2Num[i] = Bits2Num(121);
    //     bits2Num[i].in <== b[i];
    //     num[i] = bits2Num[i].out;
    // }

    // for (var i = 0; i < 17; i++) {
    //     num[i] === modulus[i];
    // }

    // TODO VERIFY RSA SIGNATURE FROM GOVERNMENT
    component num2Bits1[1306];
    var tbsInSepBits[1306][8];
    var tbsInBits[10448];
    for (var i = 0; i < 1306; i++) {
        num2Bits1[i] = Num2Bits(8);
        num2Bits1[i].in <== tbsCert[5 + i];
        tbsInSepBits[i] = num2Bits1[i].out;
    }
    for (var i = 0; i < 1306; i++) {
        for (var j = 0; j < 8; j++) {
            tbsInBits[i * 8 + j] = tbsInSepBits[i][7 - j];
        }
    }

    component rsa = RSAVerify65537(n, k);
    rsa.modulus <== modulus;
    rsa.signature <== signature;
    component sha256 = Sha256(10448);
    sha256.in <== tbsInBits;
    var hashedTbs[256] = sha256.out;

    var intermediate[3][121];
    for (var i = 0; i < 2; i++) {
        for (var j = 0; j < 121; j++) {
            intermediate[i][j] = hashedTbs[i * 121 + j];
        }
    }
    for(var i = 0; i < 14; i++) {
        intermediate[2][i] = hashedTbs[242 + i];
    }

    var hashedTbsInNum[3];
    component bits2Num[3];
    for (var i = 0; i < 3; i++) {
        bits2Num[i] = Bits2Num(121);
        bits2Num[i].in <== intermediate[i];
        hashedTbsInNum[i] = bits2Num[i].out;
    }
    for (var i = 0; i < messageLength; i++) {
        rsa.base_message[i] <== hashedTbsInNum[i];
    }
    for (var i = messageLength; i < k; i++) {
        rsa.base_message[i] <== 0;
    }

    // CALCULATE NULLIFIER:
    // component calculateHash = CalculateHash(k);
    // calculateHash.modulus <== modulus;
    // calculateHash.userSecret <== userSecret;
    // signal output hashed <== calculateHash.out; // poseidon(modulus, userSecret);
}
