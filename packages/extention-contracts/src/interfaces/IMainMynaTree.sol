//SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

interface IMainMynaTree {
    function addMainAccount(
        uint hashedModulus, 
        uint hashedUserSecret,
        uint[17] calldata signature,
        uint[8] calldata proof
    ) external;

    function getMerkleTreeRoot() external view returns (uint256);

    function getMerkleTreeDepth() external view returns (uint256);

    function getNumberOfMerkleTreeLeaves() external view returns (uint256);
    
    function verifyMainAccount(
        uint[8] calldata proof,
        uint identityCommitment
    ) external view returns (bool);
}