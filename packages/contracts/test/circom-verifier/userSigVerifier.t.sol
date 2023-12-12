// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.19;

import {Test} from "forge-std/Test.sol";
import {MynaUserSigVerifier} from "../../src/circom-verifier/userSigVerifier.sol";

contract MynaUserSigVerifierTest is Test {
    MynaUserSigVerifier public verifier;

    function setUp() public {
        verifier = new MynaUserSigVerifier();
    }

    function test_Success() public {
        uint256[2] memory pA = [
            0x22e3a116a307c2894ebfd23f89de676040e96734fd89f7e69ae67872ec4aee85,
            0x20a4370b5d42e8763614323e2f3e1d1152b3a05b909d7b8c0055c0c7ff925b8f
        ];
        uint256[2][2] memory pB = [
            [
                0x173f99b0b67d21c3abd34d1aeb6416387f7442694266b17dd15f296388a0d013,
                0x185b6e18ec7739ce6f0d6478b033e5df39924e066088661b374abbc6329fc6ea
            ],
            [
                0x14ac5849929247bd7647e47c2220e4904147bbb7432c9ce20be93a525dca6202,
                0x04b25c4e967dcdbdc05ed1af5ff3e5fba91a899fdfffbad4797276a9c54e3b94
            ]
        ];
        uint256[2] memory pC = [
            0x088338b962fbb5c86d978609794a552bc638839e8dcc5f00a812d0e5f7672c88,
            0x2e490b96e823e4b368b1733bb65994c74d7f49629ea2f1011671213ac40a84e0
        ];
        uint256[4] memory pubSignals = [
            0x1bfdeb68d2cb034ee48ec29340ff2ed6f738f6d22e764e3ac003cd9938b02488,
            0x0000000000000000000000000000000001b14c48e21b856ddf9dd67b32dbe0df,
            0x0000000000000000000000000000000000a0cbc7c70e9f529524c9810eb8f6f0,
            0x00000000000000000000000000000000000000000000000000000000000008af
        ];

        bool ret = verifier.verifyProof(pA, pB, pC, pubSignals);
        assertTrue(ret == true, "fail!");
    }
}
