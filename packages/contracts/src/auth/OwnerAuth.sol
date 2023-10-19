// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.19;

import "@libraries/Errors.sol";

/// @title OwnerAuth contract
/// @author a42x
/// @notice You can use this contract for owner auth
abstract contract OwnerAuth {
    
    function _isOwner(bytes memory modulus) internal view virtual returns (bool);
}
