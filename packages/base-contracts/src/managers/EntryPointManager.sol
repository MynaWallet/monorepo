// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.19;

import "@auth/EntryPointAuth.sol";

/// @title EntryPointManager contract
/// @author a42x
/// @notice You can use this contract for entry point manager
abstract contract EntryPointManager is EntryPointAuth {
    /// @notice Immutable entry point contract address
    IEntryPoint private immutable _ENTRY_POINT;

    /**
     * @notice EntryPointManager constructor
     * @param anEntryPoint entry point contract address
     */
    constructor(IEntryPoint anEntryPoint) {
        _ENTRY_POINT = anEntryPoint;
    }

    /**
     * @notice Get entryPoint contract address
     * @return IEntryPoint entryPoint contract address
     */
    function _entryPoint() internal view override returns (IEntryPoint) {
        return _ENTRY_POINT;
    }
}
