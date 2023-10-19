// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.19;

import "@libraries/AccountStorage.sol";
import "@libraries/Errors.sol";
import "@auth/OwnerAuth.sol";

/// @title OwnerManager contract
/// @author a42x
/// @notice You can use this contract for owner manager
abstract contract OwnerManager is OwnerAuth {
    /// @notice Length of the RSA public key modulus
    uint256 private constant _MODULUS_LENGTH = 256;
    /// @notice Exponent of the RSA public key
    bytes internal constant _EXPONENT =
        hex"0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010001";

    /// @notice Emitted when the owner is changed
    event OwnerChanged(bytes newOwner, bytes oldOwner);

    /**
     * Get owner
     * @return owner owner, modulus of the RSA public key
     * @return exponent exponent of the RSA public key
     */
    function getOwner() public view returns (bytes memory owner, bytes memory exponent) {
        return (AccountStorage.layout().owner, _EXPONENT);
    }

    /**
     * Check if the caller is the owner
     * @param modulus modulus of the RSA public key
     * @return bool true if the caller is the owner
     */
    function isOwner(bytes memory modulus) public view returns (bool) {
        return _isOwner(modulus);
    }

    /**
     * @notice Set owner
     * @dev emit OwnerChanged event
     * @param newOwner new owner, modulus of the RSA public key
     */
    function _setOwner(bytes memory newOwner) internal {
        if (newOwner.length != _MODULUS_LENGTH) {
            revert Errors.INVALID_MODULUS(newOwner);
        }
        bytes memory oldOwner = AccountStorage.layout().owner;
        AccountStorage.layout().owner = newOwner;
        emit OwnerChanged(newOwner, oldOwner);
    }

    /**
     * @notice Check if the caller is the owner
     * @dev compare the caller with the owner using the modulus of the RSA public key
     * @param modulus modulus of the RSA public key
     * @return bool true if the caller is the owner
     */
    function _isOwner(bytes memory modulus) internal view override returns (bool) {
        return keccak256(AccountStorage.layout().owner) == keccak256(modulus);
    }
}
