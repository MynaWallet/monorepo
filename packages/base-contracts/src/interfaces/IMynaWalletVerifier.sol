// SPDX-License-Identifier: GPL-3.0

pragma solidity 0.8.20;

interface IMynaGovSigVerifier {
    function verifyProof(
        uint256[2] calldata _pA,
        uint256[2][2] calldata _pB,
        uint256[2] calldata _pC,
        uint256[35] calldata _pubSignals
    ) external view returns (bool);
}

interface IMynaUserSigVerifier {
    function verifyProof(
        uint256[2] calldata _pA,
        uint256[2][2] calldata _pB,
        uint256[2] calldata _pC,
        uint256[4] calldata _pubSignals
    ) external view returns (bool);
}
