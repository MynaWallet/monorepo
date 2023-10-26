// import { expect, assert } from "chai";
// import { ethers } from "hardhat";
// import { groth16, plonk } from "snarkjs";
// import { describe, it } from "mocha";
// import { Contract, ContractFactory } from "ethers";
// import path = require("path");
// const circom_tester = require("circom_tester");
// const wasm_tester = circom_tester.wasm;

// const F1Field = require("ffjavascript").F1Field;
// const Scalar = require("ffjavascript").Scalar;
// exports.p = Scalar.fromString(
//   "21888242871839275222246405745257275088548364400416034343698204186575808495617"
// );
// const Fr = new F1Field(exports.p);

// function bigint_to_array(n: number, k: number, x: bigint) {
//   let mod: bigint = 1n;
//   for (var idx = 0; idx < n; idx++) {
//     mod = mod * 2n;
//   }

//   let ret: bigint[] = [];
//   var x_temp: bigint = x;
//   for (var idx = 0; idx < k; idx++) {
//     ret.push(x_temp % mod);
//     x_temp = x_temp / mod;
//   }
//   return ret;
// }

// function array_to_bigint(n: number, arr: bigint[]) {
//   let mod: bigint = 1n;
//   for (var idx = 0; idx < n; idx++) {
//     mod = mod * 2n;
//   }

//   let ret: bigint = 0n;
//   for (var idx = arr.length - 1; idx >= 0; idx--) {
//     ret = ret * mod + arr[idx];
//   }
//   return ret;
// }

// describe("Test rsa n = 121, k = 17", function () {
//   this.timeout(1000 * 1000);

//   function test_input() {
//     let m = BigInt(
//       "27333278531038650284292446400685983964543820405055158402397263907659995327446166369388984969315774410223081038389734916442552953312548988147687296936649645550823280957757266695625382122565413076484125874545818286099364801140117875853249691189224238587206753225612046406534868213180954324992542640955526040556053150097561640564120642863954208763490114707326811013163227280580130702236406906684353048490731840275232065153721031968704703853746667518350717957685569289022049487955447803273805415754478723962939325870164033644600353029240991739641247820015852898600430315191986948597672794286676575642204004244219381500407"
//     );
//     let sign = BigInt(
//       "27166015521685750287064830171899789431519297967327068200526003963687696216659347317736779094212876326032375924944649760206771585778103092909024744594654706678288864890801000499430246054971129440518072676833029702477408973737931913964693831642228421821166326489172152903376352031367604507095742732994611253344812562891520292463788291973539285729019102238815435155266782647328690908245946607690372534644849495733662205697837732960032720813567898672483741410294744324300408404611458008868294953357660121510817012895745326996024006347446775298357303082471522757091056219893320485806442481065207020262668955919408138704593"
//     );
//     let hashed = BigInt(
//       "83814198383102558219731078260892729932246618004265700685467928187377105751529"
//     );

//     const INPUT = {
//       signature: bigint_to_array(121, 17, sign),
//       modulus: bigint_to_array(121, 17, m),
//       sha256HashedMessage: bigint_to_array(121, 3, hashed),
//       userSecret: "42",
//     };

//     return INPUT;
//   }

//   // runs circom compilation
//   let Verifier: ContractFactory;
//   let verifier: any;
//   let circuit: any;
//   let benchmarks = {
//     calculate_witness_time: 0, // secs
//     prove_time: 0, // secs
//     verify_time: 0, // secs
//     proof_size: 0, // bytes
//   };

//   before(async function () {
//     Verifier = await ethers.getContractFactory("MynaWalletVerifier");
//     verifier = await Verifier.deploy();
//     await verifier.waitForDeployment();
//     circuit = await wasm_tester(
//       path.join(__dirname, "circuits", "test.circom")
//     );
//   });

//   it("Should pass Circuit Constraint Check", async function () {
//     const INPUT = test_input();
//     let startTime = performance.now();
//     const witness = await circuit.calculateWitness(INPUT, true);
//     let endTime = performance.now();
//     await circuit.checkConstraints(witness);
//     benchmarks.calculate_witness_time = endTime - startTime;
//   });

//   it("Should return true for correct proof", async function () {
//     const INPUT = test_input();
//     let startTime = performance.now();
//     const { proof, publicSignals } = await groth16.fullProve(
//       INPUT,
//       "out/main_js/main.wasm",
//       "setup/circuit_final.zkey"
//     );
//     let proveTime = performance.now();
//     const calldata = await groth16.exportSolidityCallData(proof, publicSignals);
//     console.log("calldata", calldata);

//     const argv = calldata
//       .replace(/["[\]\s]/g, "")
//       .split(",")
//       .map((x) => BigInt(x).toString());
//     console.log("argv", argv);

//     const a = [argv[0], argv[1]];
//     const b = [
//       [argv[2], argv[3]],
//       [argv[4], argv[5]],
//     ];
//     const c = [argv[6], argv[7]];
//     const Input = argv.slice(8);

//     expect(await verifier.verifyProof(a, b, c, Input)).to.be.true;
//     let verifyTime = performance.now();
//     benchmarks.prove_time = proveTime - startTime;
//     benchmarks.verify_time = verifyTime - proveTime;
//     benchmarks.proof_size = JSON.stringify(argv).length;
//   });

//   it("Should return false for invalid proof", async function () {
//     let a = [0, 0];
//     let b = [
//       [0, 0],
//       [0, 0],
//     ];
//     let c = [0, 0];
//     let d = Array(21).fill(0);

//     expect(await verifier.verifyProof(a, b, c, d)).to.be.false;
//   });

//   it("=== benchmark result ===", async function () {
//     console.log("---------BENCHMARK RESULT---------");
//     console.log(
//       `CALCULATE WITNESS TIME: ${benchmarks.calculate_witness_time / 1000}secs`
//     );
//     console.log(`PROVE TIME: ${benchmarks.prove_time / 1000}secs`);
//     console.log(`VERIFY TIME: ${benchmarks.verify_time / 1000}secs`);
//     console.log(`PROOF SIZE: ${benchmarks.proof_size}bytes`);
//     // TODO
//     //     1. estimated gas fees
//     //     2. bench per message size
//     console.log("----------------------------------");
//   });
// });
