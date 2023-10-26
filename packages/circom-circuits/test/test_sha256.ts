/**
 * sha256 circuit test
 * 
 * etherのバージョンはv5
 * 
 * **sha256インスタンス時のnBitsをハッシュ入力値のビット数とし、入力は2進数配列に直さないとエラーになります**
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

const wasm_path = "./test/sha256_circom/sha256_js/sha256.wasm";
// ./test/sha256_circom/sha256_js/sha256.wasm
const zkey_path = "./test/sha256_circom/sha256_js/sha256_88bit_input1.zkey";

// 文字列 -> 2進数配列
function stringToBitArray(str: string) {
    const result: Array<number> = [];
    for (let i = 0; i < str.length; i++) {
        const charCode = str.charCodeAt(i); // 8bitの文字コードを取得
        for (let j = 7; j >= 0; j--) {
            result.push((charCode >> j) & 1); // 8bitの文字コードを1bitずつ配列に格納
        }
    }
    console.log(result);
    return result;
}

describe("sha256 test", function () {
    this.timeout(1000 * 10);
    let circuit88: any;
    let VerifierContract: Contract;
    let VerifierLibraryContract: Contract;
    let PartOfVerifyingKeyContract: Contract;

    before(async function () {
        // circom circuit
        circuit88 = await circom_tester(path.join(__dirname, "circuits", "sha256-88bit.circom"));

        // solidity library contract
        VerifierLibraryContract = await ethers.deployContract("contracts/verifier_library.sol:VerifierLibrary");
        console.log("VerifierLibrary deployed to:", VerifierLibraryContract.target);

        // solidity part of verifying key contract
        PartOfVerifyingKeyContract = await ethers.deployContract("contracts/part_of_verifyingKey.sol:PartOfVerifyingKey");
        console.log("PartOfVerifyingKey deployed to:", PartOfVerifyingKeyContract.target);

        // solidity verifier contract
        VerifierContract = await ethers.deployContract("contracts/sha256_verify_contract.sol:Verifier", [VerifierLibraryContract.target, PartOfVerifyingKeyContract.target]);
        console.log("Verifier deployed to:", VerifierContract.target);
    });

    // define test cases
    const test_cases: Array<[string, string]> = [
        ["Hello World", "a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e"],
    ];

    const test_sha256 = function (x: [string, string]) {
        const [input, expected_hash] = x;
        console.log("input_word", input);
        console.log("expected_hash", expected_hash);
        console.log(input.length * 8); // 何ビットか取得できる

        // 制約の確認
        it('Testing SHA-256 circuit Constraint', async function () {
            const witness = await circuit88.calculateWitness({
                "in": stringToBitArray(input)
            });
            await circuit88.checkConstraints(witness);
        });

        // 正しい証明でtrueが返ることを確認
        it('Testing SHA-256 circuit with correct proof', async function () {
            // 証明を生成
            const { proof, publicSignals } = await groth16.fullProve(
                { "in": stringToBitArray(input) },
                wasm_path,
                zkey_path
            );

            const calldata = await groth16.exportSolidityCallData(proof, publicSignals);
            const argv = calldata.replace(/["[\]\s]/g, "").split(',').map(x => BigInt(x).toString());

            const a = [argv[0], argv[1]];
            const b = [[argv[2], argv[3]], [argv[4], argv[5]]];
            const c = [argv[6], argv[7]];
            const Input = argv.slice(8);

            // console.log("a", a);
            // console.log("b", b);
            // console.log("c", c);
            // console.log("Input", Input);

            expect(await VerifierContract.verifyProof(a, b, c, Input)).to.be.true;
        });

        // 間違った証明でfalseが返ることを確認
    //     it('Testing SHA-256 circuit with incorrect proof', async function () {
    //         const { proof, publicSignals } = await groth16.fullProve(
    //             {
    //                 "in": stringToBitArray(input)
    //             },
    //             wasm_path,
    //             zkey_path
    //         );
    //         const calldata = await groth16.exportSolidityCallData(proof, publicSignals);
    //         const argv = calldata.replace(/["[\]\s]/g, "").split(',').map(x => BigInt(x).toString());

    //         let a = [argv[0], argv[1]];
    //         const b = [[argv[2], argv[3]], [argv[4], argv[5]]];
    //         const c = [argv[6], argv[7]];
    //         const Input = argv.slice(8);
    //         // console.log("a", a);
    //         // console.log("b", b);
    //         // console.log("c", c);
    //         // console.log("Input", Input);
    //         // 1bit間違える
    //         a = [
    //             '1593332994367869238032444261388358189155131120537987106818324989472843906547',
    //             '10557282739731478900877448887495135009127965694426126024569680877740988313063'
    //         ];
    //         // const result = await VerifierContract.verifyProof(a, b, c, Input);
    //         // console.log(result);
    //         let result: boolean;
    //         try {
    //             result = await VerifierContract.verifyProof(a, b, c, Input);
    //         } catch (error) {
    //             result = false;
    //             console.log("error");
    //         }
    //         expect(result).to.be.false;
    //     });

    //     // n bit 以外の入力の場合にエラーが出ることを確認
    //     it('Testing SHA-256 circuit with incorrect proof', async function () {
    //         let a: Array<string>;
    //         let b: Array<Array<string>>;
    //         let c: Array<string>;
    //         let Input: Array<string>;
    //         let result: boolean;
    //         try {
    //             const { proof, publicSignals } = await groth16.fullProve(
    //                 {
    //                     "in": stringToBitArray("Hello Hello World")
    //                 },
    //                 wasm_path,
    //                 zkey_path
    //             );

    //             const calldata = await groth16.exportSolidityCallData(proof, publicSignals);
    //             const argv = calldata.replace(/["[\]\s]/g, "").split(',').map(x => BigInt(x).toString());

    //             a = [argv[0], argv[1]];
    //             b = [[argv[2], argv[3]], [argv[4], argv[5]]];
    //             c = [argv[6], argv[7]];
    //             Input = argv.slice(8);

    //             result = await VerifierContract.verifyProof(a, b, c, Input);
    //         } catch (error) {
    //             result = false;
    //             console.log("error");
    //         }

    //         // console.log("a", a);
    //         // console.log("b", b);
    //         // console.log("c", c);
    //         // console.log("Input", Input);

    //         expect(result).to.be.false;
    //     });
    //     // 256bit のinputでベンチマークをとる
    }
    test_cases.forEach(test_sha256);
});