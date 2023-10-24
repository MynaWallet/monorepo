// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/utils/cryptography/MerkleProof.sol";
import "@auth/Auth.sol";
import "@libraries/AccountStorage.sol";
import "@libraries/Errors.sol";

abstract contract SessionKeyManager is Auth {
    /// @notice Emitted when the session key merkle root is changed
    event SessionKeyChanged(bytes32 newSessionKeyMerkleRoot, bytes32 oldSessionKeyMerkleRoot);

    /**
     * @notice Get session key merkle root
     * @return session key merkle root
     */
    function getSessionKeyMerkleRoot() public view returns (bytes32) {
        return AccountStorage.layout().sessionKeyMerkleRoot;
    }

    /**
     * @notice Verifies a session key Merkle proof.
     * @param merkleProof The Merkle proof to verify.
     * @param leaf The leaf node of the Merkle tree.
     * @return A boolean indicating whether the Merkle proof is valid or not.
     */
    function verifySessionkeyMerkeProof(bytes32[] memory merkleProof, bytes32 leaf) public view returns (bool) {
        return MerkleProof.verify(merkleProof, getSessionKeyMerkleRoot(), leaf);
    }

    /**
     * @notice Sets the new session key merkle root.
     * @param newSessionKeyMerkleRoot The new session key merkle root to be set.
     * Can only be called by the contract itself.
     * Emits a SessionKeyChanged event with the new session key merkle root and the previous session key merkle root.
     */
    function setSssionKeyMerkleRoot(bytes32 newSessionKeyMerkleRoot) public onlySelf {
        AccountStorage.layout().sessionKeyMerkleRoot = newSessionKeyMerkleRoot;
        emit SessionKeyChanged(newSessionKeyMerkleRoot, getSessionKeyMerkleRoot());
    }

    function decomposeSignatureToSessionKey(bytes memory signature) internal pure returns (bytes memory) {
        // todo: implement
    }
}
