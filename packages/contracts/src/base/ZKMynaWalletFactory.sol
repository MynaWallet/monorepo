// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/utils/Create2.sol";
import "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import "@account-abstraction/contracts/interfaces/IEntryPoint.sol";
import {ZKMynaWallet} from "./ZKMynaWallet.sol";
import {IMynaGovSigVerifier, IMynaUserSigVerifier} from "@interfaces/IMynaWalletVerifier.sol";
/**
 * Factory contract for MynaWallet.
 */

contract ZKMynaWalletFactory {
    ZKMynaWallet public immutable accountImplementation;

    /**
     * @notice Constructor of MynaWalletFactory
     * @dev Cusntuctor is only used when factory is deployed and the facotry holds wallet implementation address which is immutable
     * @param _entryPoint EntryPoint contract address that can operate this contract
     */
    constructor(
        IEntryPoint _entryPoint,
        IMynaGovSigVerifier newGovSigVerifier,
        IMynaUserSigVerifier newUserSigVerifier
    ) {
        accountImplementation = new ZKMynaWallet(_entryPoint, newGovSigVerifier, newUserSigVerifier);
    }

    /**
     * create an account, and return its address.
     * returns the address even if the account is already deployed.
     * Note that during UserOperation execution, this method is called only if the account is not deployed.
     * This method returns an existing account address so that entryPoint.getSenderAddress() would work even after account creation
     * @param identityCommitment identityCommitment
     * @param salt salt of the account
     * @return mynaWallet deployed account
     */
    function createAccount(bytes32 identityCommitment, uint256 salt) public returns (ZKMynaWallet mynaWallet) {
        address addr = getAddress(identityCommitment, salt);
        uint256 codeSize = addr.code.length;
        if (codeSize > 0) {
            return ZKMynaWallet(payable(addr));
        }
        mynaWallet = ZKMynaWallet(
            payable(
                new ERC1967Proxy{salt: bytes32(salt)}(
                    address(accountImplementation), abi.encodeCall(ZKMynaWallet.initialize, (identityCommitment))
                )
            )
        );
    }

    /**
     * @notice calculate the counterfactual address of this account as it would be returned by createAccount()
     * @param identityCommitment identityCommitment
     * @param salt salt of the account
     * @return the address of the account
     */
    function getAddress(bytes32 identityCommitment, uint256 salt) public view returns (address) {
        return Create2.computeAddress(
            bytes32(salt),
            keccak256(
                abi.encodePacked(
                    type(ERC1967Proxy).creationCode,
                    abi.encode(
                        address(accountImplementation), abi.encodeCall(ZKMynaWallet.initialize, (identityCommitment))
                    )
                )
            )
        );
    }
}
