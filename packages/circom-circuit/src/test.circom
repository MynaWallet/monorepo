pragma circom 2.1.5;

include "../../../node_modules/circomlib/circuits/sha256/sha256.circom";

template hashTbs() {
    signal input tbsInBits[10448];
    signal output hashedTbs[256];

    component sha256 = Sha256(10448);
    sha256.in <== tbsInBits;
    hashedTbs <== sha256.out;
}

component main = hashTbs();