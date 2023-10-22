// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.19;

import "@libraries/ZKAccountStorage.sol";
import "@libraries/Errors.sol";
import "@auth/ZKOwnerAuth.sol";

/// @title ZKOwnerManager contract
/// @author a42x
/// @notice You can use this contract for owner manager
abstract contract ZKOwnerManager is ZKOwnerAuth {
    /// @notice Emitted when the owner is changed
    event OwnerChanged(bytes32 newOwner, bytes32 oldOwner);

    /**
     * Get owner
     * @return owner idnetityCommitment
     */
    function getOwner() public view returns (bytes32) {
        return ZKAccountStorage.layout().zkOwner;
    }

    /**
     * Check if the caller is the owner
     * @param identityCommitment identityCommitment
     * @return bool true if the caller is the owner
     */
    function isOwner(bytes32 identityCommitment) public view returns (bool) {
        return _isOwner(identityCommitment);
    }

    /**
     * @notice Set owner
     * @dev emit OwnerChanged event
     * @param newIdentityCommitment new owner, modulus of the RSA public key
     */
    function _setOwner(bytes32 newIdentityCommitment) internal {
        bytes32 oldOwner = ZKAccountStorage.layout().zkOwner;
        ZKAccountStorage.layout().zkOwner = newIdentityCommitment;
        emit OwnerChanged(newIdentityCommitment, oldOwner);
    }

    /**
     * @notice Check if the caller is the owner
     * @dev compare the caller with the owner using the modulus of the RSA public key
     * @param identityCommitment idnetityCommitment
     * @return bool true if the caller is the owner
     */
    function _isOwner(bytes32 identityCommitment) internal view override returns (bool) {
        return ZKAccountStorage.layout().zkOwner == identityCommitment;
    }
}
