const pubKeyInHex = "4bf5122f344554c53bde2ebb8cd2b7e3d1600ad631c385a5d7cce23c7785459a";

function hexToBinary(hex) {
    var binary = "";
    var remainingSize = hex.length;
    for (var p = 0; p < hex.length/8; p++) {
        //In case remaining hex length (or initial) is not multiple of 8
        var blockSize = remainingSize < 8 ? remainingSize  : 8;

        binary += parseInt(hex.substr(p * 8, blockSize), 16).toString(2).padStart(blockSize*4,"0");

        remainingSize -= blockSize;
    }
    return binary;
}

let binary = hexToBinary(pubKeyInHex);
binary = "1011100111011110101111110111110101010010111100110110111001100100011010001010010101001000000101111100000111111010000001110001000101100110110000111010011000111101001110000100100001010000111000010101011101011011010000101111011100000010110111000101101010100001";

let num = BigInt('0b' + binary);

console.log(num);
