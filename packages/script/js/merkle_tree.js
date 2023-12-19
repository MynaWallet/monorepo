const { MerkleTree } = require('merkletreejs');

async function main() {
    const input = 'hello';
    const output = poseidon(input);
    console.log(output);
}

main();