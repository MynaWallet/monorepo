// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.17;

import "./EntryPointAuth.sol";
import "./OwnerAuth.sol";
import "@libraries/Errors.sol";

/// @title Auth contract
/// @author a42x
/// @notice You can use this contract for auth
abstract contract Auth is EntryPointAuth, OwnerAuth {
    /// @notice modifier which only allows self call
    modifier onlySelf() {
        if (msg.sender != address(this)) {
            revert Errors.CALLER_MUST_BE_SELF(msg.sender);
        }
        _;
    }
}
