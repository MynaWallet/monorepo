const fs = require('fs');
const { join } = require('path');
const snarkjs = require('snarkjs');

const INPUT_PATH = join(__dirname, '../../circom-circuit/gov-sig-setup/input.json');
const input = JSON.parse(fs.readFileSync(INPUT_PATH, 'utf8'));
const WASM_PATH = join(__dirname, "../../circom-circuit/gov-sig-build/verify-gov-sig_js/verify-gov-sig.wasm");
const ZKEY_PATH = join(__dirname, "../../circom-circuit/gov-sig-setup/circuit_final.zkey");

async function main() {
    const { proof, publicSignals } = await snarkjs.groth16.fullProve(
        input, 
        WASM_PATH,
        ZKEY_PATH);
    console.log("publicSignals", publicSignals);
    console.log("proof", proof);

    console.log("generating calldata");

    const calldataBlob = await snarkjs.groth16.exportSolidityCallData(proof, publicSignals);
    const calldata = calldataBlob.split(',');

    console.log(calldata);
    
}
main().catch(console.error);