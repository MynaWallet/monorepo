const fs = require("fs");
const solidityRegex = /pragma\s+solidity\s+[^\n;]+;/

const verifierRegex = /contract\s+\w+Verifier/

let content = fs.readFileSync("./src/circom-verifier/verifier.sol", { encoding: 'utf-8' });
let bumped = content.replace(solidityRegex, 'pragma solidity ^0.8.0;');
bumped = bumped.replace(verifierRegex, 'contract MynaWalletVerifier');

fs.writeFileSync("./src/circom-verifier/verifier.sol", bumped);
