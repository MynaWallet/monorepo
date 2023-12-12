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

let num = BigInt('0b' + binary);

console.log(num);
