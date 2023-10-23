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
            hex"1f546bf45c855723d8b92fd479bdb601e7816945f50f7f499a778498d79460b573a1e70763bbf5197a4be2d2ecdefe3fa2bf3eea8166757ebe77a509e57a59fe089ac0cd5c95613e895b94994583925473077b9357c146c476a58b5ee86a5166f827bb2c4f34a8955ee42eab104ad91703e1fbd8e3ad65506540276c81feace1db614ec8b4a09a17bf93043f75480004df161cd093c5f303fbc915371692f24fcf41bce1b96ef1497d8907d571097f26bb4c878e01a68bccd7fb1daf040b7cc5a964f3cf2e92209f4febcc63f20dc40f4a4d6dfda112f1200a9064396bed9732122633bc77e9467136d7082f1edc59a6197016fa8f2be686405a2dd26be00dae";
        userOperation.signature = signature;

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
