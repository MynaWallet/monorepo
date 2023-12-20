// //SPDX-License-Identifier: MIT
// pragma solidity ^0.8.19;

// interface IMainMynaTree {
//     error Myna__RegisteredModulus();
//     error Myna__InvalidSignature();
//     error Myna__NeedToRegisterTwoValue();

//     event MainAccountAdded(uint256 index, uint256 hashedModulus, uint256 hashedUserSecret);

//     function addMainAccount(
//         uint hashedModulus, 
//         uint hashedUserSecret,
//         uint[17] calldata signature,
//         uint[2] calldata _pA,
//         uint[2][2] calldata _pB,
//         uint[2] calldata _pC
//     ) external;

//     function getMerkleTreeRoot() external view returns (uint256);

//     function getMerkleTreeDepth() external view returns (uint256);

//     function getNumberOfMerkleTreeLeaves() external view returns (uint256);
    
//     function verifyMainAccount(
//         uint[2] calldata _pA,
//         uint[2][2] calldata _pB,
//         uint[2] calldata _pC,
//         uint identityCommitment
//     ) external view returns (bool);
// }