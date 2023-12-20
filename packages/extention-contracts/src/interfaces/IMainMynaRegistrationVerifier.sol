//SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

interface IMainMynaRegistrationVerifier {
    function verifyProof(
        uint256[2] calldata _pA,
        uint256[2][2] calldata _pB,
        uint256[2] calldata _pC,
        uint256[19] calldata _pubSignals
    ) external view returns (bool);
}