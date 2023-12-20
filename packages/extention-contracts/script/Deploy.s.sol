// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

import "forge-std/Script.sol";
import "forge-std/console.sol";
import "../src/MainMynaTree.sol";
import "../src/circom-verifier/MainMynaInclusionVerifier.sol";
import "../src/circom-verifier/MainMynaRegistraionVerifier.sol";

contract DeployMainMynaInclusionVerifier is Script {
    function setUp() public {}

    function run() public {
        vm.startBroadcast(vm.envUint("PRIVATE_KEY"));
        MainMynaInclusionVerifier inclusionVerifier = new MainMynaInclusionVerifier();
        console.log("Deployed inclusion verifier at: ", address(inclusionVerifier));
        vm.stopBroadcast();
    }
}

contract DeployMainMynaRegistrationVerifier is Script {
    function setUp() public {}

    function run() public {
        vm.startBroadcast(vm.envUint("PRIVATE_KEY"));
        MainMynaRegistrationVerifier registrationVerifier = new MainMynaRegistrationVerifier();
        console.log("Deployed registration verifier at: ", address(registrationVerifier));
        vm.stopBroadcast();
    }
}

contract DeployMainMynaTree is Script {
    function setUp() public {}

    function run() public {
        vm.startBroadcast(vm.envUint("PRIVATE_KEY"));
        address registrationVerifierAddress = vm.envAddress("REGISTRATION_VERIFIER_ADDRESS");
        address inclusionVerifierAddress = vm.envAddress("INCLUSION_VERIFIER_ADDRESS");
        MainMynaTree mainTree = new MainMynaTree(
            IMainMynaRegistrationVerifier(registrationVerifierAddress),
            IMainMynaInclusionVerifier(inclusionVerifierAddress)
        );
        console.log("Deployed main tree at: ", address(mainTree));
        vm.stopBroadcast();
    }
}

// contract DeployMainMynaTree is Script {
//     function setUp() public {}

//     function run() public {
//         vm.startBroadcast(vm.envUint("PRIVATE_KEY"));
//         // address registrationVerifierAddress = vm.envAddress("REGISTRATION_VERIFIER_ADDRESS");
//         address inclusionVerifierAddress = vm.envAddress("INCLUSION_VERIFIER_ADDRESS");
//         MainMynaTree mainTree = new MainMynaTree(
//             IMainMynaInclusionVerifier(inclusionVerifierAddress)
//         );
//         console.log("Deployed main tree at: ", address(mainTree));
//         vm.stopBroadcast();
//     }
// }