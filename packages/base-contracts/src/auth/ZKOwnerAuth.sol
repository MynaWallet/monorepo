// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.19;

import "@libraries/Errors.sol";

/// @title ZKOwnerAuth contract
/// @author a42x
/// @notice You can use this contract for owner auth
abstract contract ZKOwnerAuth {
    function _isOwner(bytes32 identityCommitment) internal view virtual returns (bool);
}
