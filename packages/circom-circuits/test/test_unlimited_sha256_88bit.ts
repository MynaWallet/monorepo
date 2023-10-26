// /**
//  * sha256 circuit test
//  * 
//  * etherのバージョンはv5
//  * 
//  * **sha256インスタンス時のnBitsをハッシュ入力値のビット数とし、入力は2進数配列に直さないとエラーになります**
//  */

// // for circom
// import path from "path"; // .circom file
// // import { describe, it } from 'mocha';
// const wasm_tester = require("circom_tester").wasm;
// import { expect } from 'chai';

// // for solidity
// const { ethers } = require("hardhat");
// // import { ethers } from "hardhat"; // v5の書き方
// import { groth16, plonk } from "snarkjs";
// import { Contract, ContractFactory } from "ethers";

// // 文字列 -> 2進数配列
// function stringToBitArray(str: string) {
//     const result: Array<number> = [];
//     for (let i = 0; i < str.length; i++) {
//         const charCode = str.charCodeAt(i); // 8bitの文字コードを取得
//         for (let j = 7; j >= 0; j--) {
//             result.push((charCode >> j) & 1); // 8bitの文字コードを1bitずつ配列に格納
//         }
//     }
//     console.log(result);
//     return result;
// }

// // describe: テストのグループ化
// describe("sha256 test", function () {
//     // this test can take about 16 minutes
//     this.timeout(1000 * 1000);
//     let circuit: any;
//     let VerifierFactory: ContractFactory;
//     let VerifierContract: any;
    

//     // テストが始まる前に実行される
//     // circomサーキットを取得
//     // VerifierLibraryコントラクトをデプロイしアドレスを取得
//     // Verifierコントラクトをデプロイ
//     before(async function () {
//         // circom circuit
//         circuit = await wasm_tester(path.join(__dirname, "circuits", "sha256.circom"));

//         // solidity library contract
//         VerifierFactory = await ethers.getContractFactory("contracts/sha256_verifier.sol:Verifier");
//         VerifierContract = await VerifierFactory.deploy();
//         await VerifierContract.deployed();
//         console.log("VerifierLibrary deployed to:", VerifierContract.address);
//     });

//     // define test cases
//     const test_cases: Array<[string, string]> = [
//         ["Hello World", "a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e"],
//     ];

//     const test_sha256 = function (x: [string, string]) {
//         const [input, expected_hash] = x;
//         console.log("input", input);
//         console.log("expected_hash", expected_hash);
//         console.log(input.length * 8);

//         // 制約の確認
//         it('Testing SHA-256 circuit Constraint', async function () {
//             const witness = await circuit.calculateWitness({
//                 "in": stringToBitArray(input)
//             });
//             await circuit.checkConstraints(witness);
//         });

//         // 正しい証明でtrueが返ることを確認
//         it('Testing SHA-256 circuit with correct proof', async function () {
//             // 証明を生成
//             const { proof, publicSignals } = await groth16.fullProve(
//                 {
//                     "in": stringToBitArray(input)
//                 },
//                 "/Users/yamamotoyuta/Desktop/a42/code/MynaWallet-circom/monorepo/monorepo/packages/circom-circuits/test/sha256_circom/sha256_js/sha256.wasm",
//                 "/Users/yamamotoyuta/Desktop/a42/code/MynaWallet-circom/monorepo/monorepo/packages/circom-circuits/test/sha256_circom/sha256_js/sha256_88bit_input1.zkey"
//             );
//             console.log("proof", proof);
//             console.log("publicSignals", publicSignals);

//             const calldata = await groth16.exportSolidityCallData(proof, publicSignals);

//             const argv = calldata.replace(/["[\]\s]/g, "").split(',').map(x => BigInt(x).toString());

//             const a = [argv[0], argv[1]];
//             const b = [[argv[2], argv[3]], [argv[4], argv[5]]];
//             const c = [argv[6], argv[7]];
//             const Input = argv.slice(8);

//             console.log("a", a);
//             console.log("b", b);
//             console.log("c", c);
//             console.log("Input", Input);

//             expect(await VerifierContract.verifyProof(a, b, c, Input)).to.be.true;
//             console.log("verifyProof is true");
//         });

//         // 間違った証明でfalseが返ることを確認
//         // it('Testing SHA-256 circuit with incorrect proof', async function (){

//         // });

//         // n bit 以外の入力の場合にエラーが出ることを確認

//         // 256bit のinputでベンチマークをとる
//     }
//     test_cases.forEach(test_sha256);
// });