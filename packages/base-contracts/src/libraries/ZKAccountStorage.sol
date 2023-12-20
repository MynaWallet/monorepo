// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.19;

/// @title AccountStorage(diamond storage contract)
/// @author a42x
/// @notice You can use this contract for account storage

library ZKAccountStorage {
    /// @notice AccountStorage slot
    bytes32 private constant _ACCOUNT_SLOT = keccak256("ZKMynaWallet.AccountStorage");

    /// @notice AccountStorage Struct
    struct Layout {
        /// @notice base account storage
        bytes32 zkOwner;
        /// @notice buffer(gap)
        uint256[50] gap0;
        bool verified;
    }

    // TODO session key storage

    // TODO recovery key storage

    /**
     * @notice Get AccountStorage
     * @dev return AccountStorage struct at the _ACCOUNT_SLOT
     * @return l AccountStorage
     */
    function layout() internal pure returns (Layout storage l) {
        bytes32 slot = _ACCOUNT_SLOT;
        assembly ("memory-safe") {
            l.slot := slot
        }
    }
}
