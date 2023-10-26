pragma circom 2.1.5;
include "./mynawallet.circom";

// * n = 121 is the number of bits in each chunk of the modulus (RSA parameter)
// * k = 17 is the number of chunks in the modulus (RSA parameter)
component main { public [ signature, sha256HashedMessage, govModulus ] } = MynaWalletVerify(121, 17);
