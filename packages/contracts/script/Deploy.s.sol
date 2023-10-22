// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

import "forge-std/Script.sol";
import "forge-std/console.sol";
import "@account-abstraction/contracts/core/EntryPoint.sol";
import "@account-abstraction/contracts/interfaces/IEntryPoint.sol";
import "../src/base/MynaWalletFactory.sol";
import "../src/base/ZKMynaWalletFactory.sol";
import "../src/base/MynaWalletPaymaster.sol";

contract DeployLocal is Script {
    function setUp() public {}

    function run() public {
        vm.startBroadcast(vm.envUint("PRIVATE_KEY"));
        EntryPoint entryPoint = new EntryPoint();
        MynaWalletFactory factory = new MynaWalletFactory(entryPoint);
        console.log("Deployed entryPoint at: ", address(entryPoint));
        console.log("Deployed factory at: ", address(factory));
        vm.stopBroadcast();
    }
}

contract DeployFactory is Script {
    function setUp() public {}

    function run() public {
        vm.startBroadcast(vm.envUint("PRIVATE_KEY"));

        address entryPointAddress = vm.envAddress("ENTRY_POINT_ADDRESS");

        MynaWalletFactory factory = new MynaWalletFactory(
            IEntryPoint(entryPointAddress)
        );
        console.log("Deployed factory at: ", address(factory));
        vm.stopBroadcast();
    }
}

contract DeployPaymaster is Script {
    function setUp() public {}

    function run() public {
        vm.startBroadcast(vm.envUint("PRIVATE_KEY"));
        MynaWalletPaymaster paymaster = new MynaWalletPaymaster(
            IEntryPoint(vm.envAddress("ENTRY_POINT_ADDRESS")),
            vm.envAddress("PAYMASTER_OWNER_ADDRESS")
        );
        console.log("Deployed paymaster at: ", address(paymaster));
        vm.stopBroadcast();
    }
}

contract DeployZKFactory is Script {
    function setUp() public {}

    function run() public {
        vm.startBroadcast(vm.envUint("PRIVATE_KEY"));

        address entryPointAddress = vm.envAddress("ENTRY_POINT_ADDRESS");
        address verifierAddress = vm.envAddress("VERIFIER_ADDRESS");

        // MynaWalletVerifier verifier = new MynaWalletVerifier();

        ZKMynaWalletFactory factory = new ZKMynaWalletFactory(
            IEntryPoint(entryPointAddress),
            IMynaWalletVerifier(verifierAddress)
        );
        console.log("Deployed factory at: ", address(factory));
        vm.stopBroadcast();
    }
}
