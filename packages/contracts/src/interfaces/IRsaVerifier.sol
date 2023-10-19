// SPDX-License-Identifier: GPL-3.0

pragma solidity ^0.8.0;

interface IRsaVerifier {
    function verifyProof(uint[2] calldata _pA, uint[2][2] calldata _pB, uint[2] calldata _pC, uint[21] calldata _pubSignals) external view returns (bool);
}