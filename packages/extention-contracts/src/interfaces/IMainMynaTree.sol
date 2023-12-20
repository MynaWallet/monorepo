//SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

interface IMainMynaTree {
    function addMainAccount(
        uint256 hashedModulus,
        uint256 hashedUserSecret,
        uint256[17] calldata signature,
        uint256[8] calldata proof
    ) external;

    function getMerkleTreeRoot() external view returns (uint256);

    function getMerkleTreeDepth() external view returns (uint256);

    function getNumberOfMerkleTreeLeaves() external view returns (uint256);

    function verifyMainAccount(uint256[8] calldata proof, uint256 identityCommitment) external view returns (bool);
}
