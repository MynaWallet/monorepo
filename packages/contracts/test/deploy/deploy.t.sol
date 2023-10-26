// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "@account-abstraction/contracts/core/EntryPoint.sol";
import "@account-abstraction/contracts/interfaces/IEntryPoint.sol";
import "@account-abstraction/contracts/interfaces/UserOperation.sol";

import "@factories/MynaWalletFactory.sol";

contract DeployTest is Test {
    EntryPoint public entryPoint;
    MynaWalletFactory public mynaWalletFactory;

    address bundler;
    address alice;

    function setUp() public {
        bundler = vm.addr(0x1);
        alice = vm.addr(0x2);

        entryPoint = new EntryPoint();
        mynaWalletFactory = new MynaWalletFactory(entryPoint);
    }

    function testDeploy() public {
        // fix block number
        vm.warp(1641070800);
        // fix chainId
        vm.chainId(31337);

        address sender;
        uint256 nonce;
        bytes memory initCode;
        bytes memory callData;
        uint256 callGasLimit;
        uint256 verificationGasLimit;
        uint256 preVerificationGas;
        uint256 maxFeePerGas;
        uint256 maxPriorityFeePerGas;
        bytes memory paymasterAndData;
        bytes memory signature;

        bytes memory modulus =
            hex"8f6047064f400fd2ff80ad6569c2cffc238079e2cb18648305a59b9f1f389730f9bf9b5e3e436f88065c06241c7189ba43b6adbe5ec7a979d4b42f2a450cd19e8075e5a817b04328a0d16ebfcb6bc09a96020217af6218f3765dbc129131edd004472ab45908bf02ec35b7c044e1c900f7df179fc19c94835802e58c432bc73cee54148a6f24d7316cca195791c87e07e85b07f80b71ddc15b9b053e6f0265a8e81c27c7546dea38cbb951ca71c384892b81df12c8cb0444f9e04d24d0d3323fa857075be26746f4b731a186a51cec24151597b9d31c9ef78db83f27ef0d973d4d2a2d8a9093c7118bf86322603a17d7814a05f6150963b72a275f645a099319";
        uint256 salt = 0;

        // check if the contract is not deployed
        sender = mynaWalletFactory.getAddress(modulus, salt);
        assertTrue(sender.code.length == 0, "address is not zero");

        // generate userOperation
        verificationGasLimit = 1000000;
        preVerificationGas = 150000;
        callGasLimit = 1000000;
        maxFeePerGas = 10 gwei;
        maxPriorityFeePerGas = 10 gwei;
        bytes memory mynaWalletFactoryCall = abi.encodeWithSignature("createAccount(bytes,uint256)", modulus, salt);
        initCode = abi.encodePacked(address(mynaWalletFactory), mynaWalletFactoryCall);

        // generate transfer calldata
        callData = hex"";
        bytes memory executeCall =
            abi.encodeWithSignature("execute(address,uint256,bytes)", address(alice), 1 ether, callData);

        // construct UserOperation
        UserOperation memory userOperation = UserOperation(
            sender,
            nonce,
            initCode,
            executeCall,
            callGasLimit,
            verificationGasLimit,
            preVerificationGas,
            maxFeePerGas,
            maxPriorityFeePerGas,
            paymasterAndData,
            signature
        );

        bytes32 userOpHash = entryPoint.getUserOpHash(userOperation);
        console.logBytes32(userOpHash);

        // set actual signature
        signature =
            hex"3b277e317a1e535dbf3d40bdbddf8d9e3303abe5ec8224468e2f3ab268e83df079420815e930b2b9525935aaf78510e06f69d33f72da939cc0cf04177ee3f651424ef425a824c0e4a8c7992b05d8bd64c6fa705e3545c124b42f9fe1ef65c7ed859652e22dce03d6ba6ceb06792b16290125482fa117a8af3531ce981abbcc3dfc21b7f9fdbd670bec282850a1b5fb7e09d11c6d48cde58d15931a619f1219ccddd0e13f6e652ae2478c7e9e905593f30a297e11a4a85ebbcd9eff49ade085a3e1698113b28d1eb8a52115719dee92e457d147f7d095c6cad9b728f333e12ad22740101e10d28c4e093bb3c4fcb45a4775490aa957b3531f515b99eba3bf5e6c";

        // validationType + actual signature
        bytes memory actualSig = abi.encode(1, signature);

        // set signature as address(sender) + (0x01 + signature)
        userOperation.signature = abi.encode(sender, actualSig);

        UserOperation[] memory userOperations = new UserOperation[](1);
        userOperations[0] = userOperation;

        // deposit ether
        vm.deal(sender, 42 ether);
        vm.deal(bundler, 1 ether);

        // execute as bundler
        vm.startPrank(bundler);
        entryPoint.handleOps(userOperations, payable(bundler));
        vm.stopPrank();

        // check if the contract is deployed
        assertTrue(sender.code.length != 0, "A2:sender.code.length == 0");
        // check if alice has 1 ether
        assertTrue(alice.balance == 1 ether, "A3:alice.balance != 1 ether");
    }
}
