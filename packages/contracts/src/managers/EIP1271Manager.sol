// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.19;

import {_parseValidationData, ValidationData} from "@account-abstraction/contracts/core/Helpers.sol";
import "@libraries/AccountStorage.sol";
import "@libraries/Errors.sol";
import "@auth/Auth.sol";
import "@managers/OwnerManager.sol";
import {SolRsaVerify} from "@libraries/RsaVerify.sol";

abstract contract EIP1271Manager is Auth, OwnerManager {
    using SolRsaVerify for bytes32;

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
        if (signature.length > 0) {
            (uint256 _validationData, bool sigValid) = _isValidSignature(hash, signature);
            if (!sigValid) {
                return _INVALID_ID;
            }
            if (_validationData > 0) {
                ValidationData memory validationData = _parseValidationData(_validationData);
                bool outOfTimeRange =
                    (block.timestamp > validationData.validUntil) || (block.timestamp < validationData.validAfter);
                if (outOfTimeRange) {
                    return _INVALID_TIME_RANGE;
                }
            }
            return _MAGICVALUE;
        } else {
            return _INVALID_ID;
        }
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
        (bytes memory modulus, bytes memory exponent) = getOwner();
        if (signature.length == 256) {
            return (0, hash.pkcs1Sha256Verify(signature, exponent, modulus) == 0);
        } else {
            // extract validation data - first 32 bytes is the validation data
            validationData = abi.decode(signature[0:32], (uint256));

            // extract signature - 256 bytes after the validation data is the signature
            signature = signature[32:288];
            isValid = hash.pkcs1Sha256Verify(signature, exponent, modulus) == 0;
        }
    }
}
