// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.19;

/// @title Errors contract
/// @author a42x
/// @notice You can use this contract for errors
library Errors {
    error CALLER_MUST_BE_ENTRYPOINT(address sender);
    error CALLER_MUST_BE_SELF(address sender);
    error INVALID_ARRAY_LENGTH(uint256 destLength, uint256 valueLength, uint256 funcLength);
    error INVALID_MODULUS(bytes modulus);
    error HASH_ALREADY_APPROVED(bytes32 hash);
    error HASH_ALREADY_REJECTED(bytes32 hash);
    error INVALID_CALLDATA();
    error ENTRYPOINT_SELECTOR_MUST_BE_EXECUTE_OR_EXECUTE_BATCH(bytes4 givenSelector);
    error CANNOT_CALL_SET_SESSION_KEY_MERKLE_ROOT();
}
