pragma circom 2.1.5;

include "../../../node_modules/circomlib/circuits/bitify.circom";
include "../../../node_modules/circomlib/circuits/poseidon.circom";
include "./helpers/bigint.circom";
include "./helpers/rsa.circom";
include "./helpers/sha.circom";
include "./helpers/extract.circom";

template SelectiveDisclosure(max_cert_bytes, n, k) {
    signal input raw_tbs_cert[max_cert_bytes];
    signal input mask[max_cert_bytes];

    signal input message_padded_bytes;

    signal input modulus[k];
    signal input signature[k];
    // To prevent brute force attack you need to set this number more than 12+ digits
    // Should we add assertion?
    signal input userSecret;
    signal input start;

    // ======== Variables Verification ========
    assert(max_cert_bytes % 64 == 0);
    assert(n * k > 2048);
    assert(n < (255 \ 2));
    for (var i = 0; i < max_cert_bytes; i++) {
        mask[i] * (mask[i] - 1) === 0;
    }

    // ======== Hash Raw TBS Certificate ========
    component sha = Sha256Bytes(max_cert_bytes);
    for (var i = 0; i < max_cert_bytes; i++) {
        sha.in_padded[i] <== raw_tbs_cert[i];
    }
    sha.in_len_padded_bytes <== message_padded_bytes;

    var msg_len = (256+n)\n;
    component base_msg[msg_len];
    for (var i = 0; i < msg_len; i++) {
        base_msg[i] = Bits2Num(n);
    }
    for (var i = 0; i < 256; i++) {
        base_msg[i\n].in[i%n] <== sha.out[255 - i];
    }
    for (var i = 256; i < n*msg_len; i++) {
        base_msg[i\n].in[i%n] <== 0;
    }

    // ======== Verify RSA Signature ========
    component rsa = RSAVerify65537(n, k);
    for (var i = 0; i < msg_len; i++) {
        rsa.base_message[i] <== base_msg[i].out;
    }
    for (var i = msg_len; i < k; i++) {
        rsa.base_message[i] <== 0;
    }
    for (var i = 0; i < k; i++) {
        rsa.modulus[i] <== modulus[i];
    }
    for (var i = 0; i < k; i++) {
        rsa.signature[i] <== signature[i];
    }

    // ======== Mask Raw TBS Certificate ========
    signal maskedTbsCert[max_cert_bytes];
    for (var i = 0; i < max_cert_bytes; i++) {
        maskedTbsCert[i] <== raw_tbs_cert[i] * mask[i];
    }

    // ======== Shift Masked TBS Certificate ========
    component shiftLeft = VarShiftLeft(2048, 150);
    shiftLeft.in <== maskedTbsCert;
    shiftLeft.shift <== start;

    // ======== Check Object Identifier ========
    shiftLeft.out[2] === 42;
    shiftLeft.out[3] === 131;
    shiftLeft.out[4] === 8;
    shiftLeft.out[5] === 140;
    shiftLeft.out[6] === 155;
    shiftLeft.out[7] === 85;
    shiftLeft.out[8] === 8;
    shiftLeft.out[9] === 5;
    shiftLeft.out[10] === 5;
    shiftLeft.out[11] === 5;

    // ======== Pack and Hash Masked TBS Certificate ========
    component packBytes = PackBytes(150, 5, 31);
    packBytes.in <== shiftLeft.out;
    signal output out[5];
    component poseidon[5];
    for (var i = 0; i < 5; i++) {
        poseidon[i] = Poseidon(2);
        // Is this secure?
        poseidon[i].inputs[0] <== packBytes.out[i];
        poseidon[i].inputs[1] <== userSecret;
        out[i] <== poseidon[i].out;
    }
}

component main { public [ modulus ] } = SelectiveDisclosure(2048, 121, 17);