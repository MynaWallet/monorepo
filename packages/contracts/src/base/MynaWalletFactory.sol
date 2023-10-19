// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/utils/Create2.sol";
import "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import "@account-abstraction/contracts/interfaces/IEntryPoint.sol";
import "./MynaWallet.sol";

/**
 * Factory contract for MynaWallet.
 */
contract MynaWalletFactory {
    MynaWallet public immutable accountImplementation;

    /**
     * @notice Constructor of MynaWalletFactory
     * @dev Cusntuctor is only used when factory is deployed and the facotry holds wallet implementation address which is immutable
     * @param _entryPoint EntryPoint contract address that can operate this contract
     */
    constructor(IEntryPoint _entryPoint) {
        accountImplementation = new MynaWallet(_entryPoint);
    }

    /**
     * create an account, and return its address.
     * returns the address even if the account is already deployed.
     * Note that during UserOperation execution, this method is called only if the account is not deployed.
     * This method returns an existing account address so that entryPoint.getSenderAddress() would work even after account creation
     * @param modulus modulus of the RSA public key which can operate this contract
     * @param salt salt of the account
     * @return mynaWallet deployed account
     */
    function createAccount(bytes memory modulus, uint256 salt) public returns (MynaWallet mynaWallet) {
        address addr = getAddress(modulus, salt);
        uint256 codeSize = addr.code.length;
        if (codeSize > 0) {
            return MynaWallet(payable(addr));
        }
        mynaWallet = MynaWallet(
            payable(
                new ERC1967Proxy{salt: bytes32(salt)}(
                    address(accountImplementation),
                    abi.encodeCall(MynaWallet.initialize, (modulus))
                )
            )
        );
    }

    /**
     * @notice calculate the counterfactual address of this account as it would be returned by createAccount()
     * @param modulus modulus of the RSA public key which can operate this contract
     * @param salt salt of the account
     * @return the address of the account
     */
    function getAddress(bytes memory modulus, uint256 salt) public view returns (address) {
        return Create2.computeAddress(
            bytes32(salt),
            keccak256(
                abi.encodePacked(
                    type(ERC1967Proxy).creationCode,
                    abi.encode(address(accountImplementation), abi.encodeCall(MynaWallet.initialize, (modulus)))
                )
            )
        );
    }
}
