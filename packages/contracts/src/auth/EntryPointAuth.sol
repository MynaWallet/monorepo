// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.19;

import "@account-abstraction/contracts/interfaces/IEntryPoint.sol";
import "@libraries/Errors.sol";

/// @title EntryPointAuth contract
/// @author a42x
/// @notice You can use this contract for entry point auth
abstract contract EntryPointAuth {
    /**
     * @notice Get entryPoint contract address
     * @return IEntryPoint entryPoint contract address
     */
    function _entryPoint() internal view virtual returns (IEntryPoint);

    /// @notice modifier which only allows entryPoint call
    modifier onlyEntryPoint() {
        if (msg.sender != address(_entryPoint())) {
            revert Errors.CALLER_MUST_BE_ENTRYPOINT(msg.sender);
        }
        _;
    }
}
