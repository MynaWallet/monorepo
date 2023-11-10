// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.19;

import {_parseValidationData, ValidationData} from "@account-abstraction/contracts/core/Helpers.sol";
import "@libraries/ZKAccountStorage.sol";
import "@libraries/Errors.sol";
import "@auth/ZKAuth.sol";
import "@managers/ZKStorageManager.sol";

abstract contract ZKEIP1271Manager is ZKAuth, ZKStorageManager {
    /// @notice EIP1271: Standard Signature Validation Method for Contracts
    /// https://eips.ethereum.org/EIPS/eip-1271
    bytes4 internal constant _MAGICVALUE = 0x1626ba7e;
    bytes4 internal constant _INVALID_ID = 0xffffffff;
    bytes4 internal constant _INVALID_TIME_RANGE = 0xfffffffe;

    /**
     * @notice Verify if the signature is valid
     * @dev we update the signature validation logic to support time range validation
     * @param hash Hash of the data
     * @param signature Signature of the data
     * @return magicValue Magic value if the signature is valid or invalid id / invalid time range
     */
    function isValidSignature(bytes32 hash, bytes calldata signature) external view returns (bytes4 magicValue) {
        // todo
        (hash, signature);
        return _INVALID_ID;
    }

    /**
     * @notice Verify if the signature is valid
     * @param hash Hash of the data
     * @param signature Signature of the data
     * @return validationData Validation data
     * @return isValid Signature is valid or not
     */
    function _isValidSignature(bytes32 hash, bytes calldata signature)
        internal
        view
        returns (uint256 validationData, bool isValid)
    {
        // todo
        (hash, signature);
        return (0, false);
    }
}
