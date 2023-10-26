/**
 * sha256 256bit テスト
 */
// for circom
import path from "path";
import { expect } from 'chai';
// @ts-expect-error
import { wasm } from "circom_tester";
const circom_tester: any = wasm;

// for solidity
// import ethers from "hardhat";// <- これはマジで動かない
const { ethers } = require("hardhat");
type Contract = typeof ethers.Contract;
import { groth16 } from "snarkjs";

const wasm_path = "./test/sha256_256bit_circom/sha256_js/sha256.wasm";
const zkey_path = "./test/sha256_256bit_circom/sha256_js/multiplier2_0001.zkey";


// 文字列 -> 2進数配列
function stringToBitArray(str: string) {
    const result: Array<number> = [];
    for (let i = 0; i < str.length; i++) {
        const charCode = str.charCodeAt(i); // 8bitの文字コードを取得
        for (let j = 7; j >= 0; j--) {
            result.push((charCode >> j) & 1); // 8bitの文字コードを1bitずつ配列に格納
        }
    }
    // console.log(result);
    return result;
}

describe("sha256 256bit input test", function () {
    this.timeout(1000 * 10);
    let circuit256: any;
    let VerifierContract: Contract;
    let StoreConstContract: Contract;

    before(async function () {
        // circom circuit
        circuit256 = await circom_tester(path.join(__dirname, "sha256_256bit_circom", "sha256.circom"));

        // solidity store const contract
        // StoreConstContract = await ethers.deployContract("contracts/sha256_256bit/sha256_const.sol:StoreConstValues");

        // solidity verifier contract
        VerifierContract = await ethers.deployContract("contracts/sha256_256bit/verifier.sol:Groth16Verifier");
        // VerifierContract = await ethers.deployContract("contracts/sha256_256bit/sha256_256bit.sol:Groth16Verifier", [StoreConstContract.target]);
        // VerifierContract = await ethers.deployContract("Groth16VerifierSplit", [StoreConstContract.target]);
        console.log("Verifier deployed to:", VerifierContract.target);
    });


    const input: string = "b94d27b9934d3e08a52e52d7da7dabfa";
    console.log('input: ', input);
    console.log('bit:', input.length * 8);

    // it('Testing sha256 circuti constraint', async function () {
    //     const witness = await circuit256.calculateWitness({
    //         "in": stringToBitArray(input)
    //     });
    //     await circuit256.checkConstraints(witness);
    // })

    it('Testing sha256 groth16 proof', async function () {
        const witness = await circuit256.calculateWitness({
            "in": stringToBitArray(input)
        });
        const { proof, publicSignals } = await groth16.fullProve(
            { "in": stringToBitArray(input) },
            wasm_path,
            zkey_path
        );

        // const q = await VerifierContract.checkQ();
        // console.log('-----q value----');
        // console.log(q);
        // console.log('------------');

        // const q_slot = await VerifierContract.checkQ_slot();
        // console.log('-----q slot----');
        // console.log(q_slot);
        // console.log('------------');

        // await VerifierContract.setVariable();

        const calldata = await groth16.exportSolidityCallData(proof, publicSignals);
        const argv = calldata.replace(/["[\]\s]/g, "").split(',').map(x => BigInt(x).toString());

        const a = [argv[0], argv[1]];
        const b = [[argv[2], argv[3]], [argv[4], argv[5]]];
        const c = [argv[6], argv[7]];
        const Input = argv.slice(8);
        const result = await VerifierContract.verifyProof(a, b, c, Input)
        console.log("result", result);
        expect(result).to.be.true;
    });
});
