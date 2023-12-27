pragma circom 2.1.5;

include "../../../node_modules/circomlib/circuits/bitify.circom";
include "../../../node_modules/circomlib/circuits/poseidon.circom";
include "./helpers/bigint.circom";
include "./helpers/rsa.circom";
include "./helpers/sha.circom";
include "./helpers/extract.circom";

template SelectiveDisclosure(max_cert_bytes, n, k) {
    signal input rawTbsCert[max_cert_bytes];
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
        sha.in_padded[i] <== rawTbsCert[i];
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

    // TODO: Make this output smaller
    // ======== Mask and Hash Raw TBS Certificate ========
    signal maskedTbsCert[max_cert_bytes];
    // component poseidon[max_cert_bytes];
    // for (var i = 0; i < max_cert_bytes; i++) {
    //     poseidon[i] = Poseidon(2);
    //     // Is this secure?
    //     poseidon[i].inputs[0] <== rawTbsCert[i] * mask[i];
    //     poseidon[i].inputs[1] <== userSecret;
    //     maskedTbsCert[i] <== poseidon[i].out;
    // }
    for (var i = 0; i < max_cert_bytes; i++) {
        maskedTbsCert[i] <== rawTbsCert[i] * mask[i];
    }
    component shiftAndPack = ShiftAndPack(2048, 150, 31);
    shiftAndPack.in <== maskedTbsCert;
    shiftAndPack.shift <== start;
    signal output out[5] <== shiftAndPack.out;
}

component main = SelectiveDisclosure(2048, 121, 17);