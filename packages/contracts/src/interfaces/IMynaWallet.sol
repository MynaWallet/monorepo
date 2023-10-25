// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.20;

import "@account-abstraction/contracts/core/BaseAccount.sol";
import "@account-abstraction/contracts/interfaces/IEntryPoint.sol";

interface IMynaWallet {
    function initialize(bytes memory newModulus) external;
    function execute(address dest, uint256 value, bytes calldata func) external;
    function executeBatch(address[] calldata dest, bytes[] calldata func) external;
    function executeBatch(address[] calldata dest, uint256[] calldata value, bytes[] calldata func) external;
    function addDeposit() external payable;
    function withdrawDepositTo(address payable withdrawAddress, uint256 amount) external;
    function entryPoint() external view returns (IEntryPoint);
    function getDeposit() external view returns (uint256);
}
