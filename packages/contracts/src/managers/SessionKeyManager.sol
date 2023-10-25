// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.19;
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
     * @notice Sets the new session key merkle root.
     * @param newSessionKeyMerkleRoot The new session key merkle root to be set.
     * Can only be called by the contract itself.
     * Emits a SessionKeyChanged event with the new session key merkle root and the previous session key merkle root.
     */
    function setSssionKeyMerkleRoot(bytes32 newSessionKeyMerkleRoot) public onlySelf {
        AccountStorage.layout().sessionKeyMerkleRoot = newSessionKeyMerkleRoot;
        emit SessionKeyChanged(newSessionKeyMerkleRoot, getSessionKeyMerkleRoot());
    }

}
