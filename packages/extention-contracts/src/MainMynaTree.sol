//SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import {BinaryIMT, BinaryIMTData} from  "./utils/BinaryIMT.sol";
import "./interfaces/IMainMynaRegistrationVerifier.sol";
import "./interfaces/IMainMynaInclusionVerifier.sol";
import "./interfaces/IMainMynaTree.sol";

contract MainMynaTree is IMainMynaTree {
    BinaryIMTData internal mynaMainAccounts;
    IMainMynaRegistrationVerifier public registrationVerifier;
    IMainMynaInclusionVerifier public inclusionVerifier;

    struct MainAccount {
        uint256 hashedModulus;
        uint256 hashedUserSecret;
    }

    mapping(uint256 => uint256) internal isRegisteredModulus;
    mapping(uint256 => MainAccount) public mainAccounts;
    uint256 internal totalMainAccounts;
    
    error Myna__RegisteredModulus();
    error Myna__InvalidSignature();
    error Myna__NeedToRegisterTwoValue();

    event MainAccountAdded(uint256 index, uint256 hashedModulus, uint256 hashedUserSecret);

    constructor(
        IMainMynaRegistrationVerifier _registrationVerifier,
        IMainMynaInclusionVerifier _inclusionVerifier
    ) {
        uint256 zeroValue = uint256(keccak256(abi.encodePacked(uint(42)))) >> 8;
        BinaryIMT.init(mynaMainAccounts, 16, zeroValue);
        registrationVerifier = _registrationVerifier;
        inclusionVerifier = _inclusionVerifier;
    }

    function addMainAccount(
        uint hashedModulus, 
        uint hashedUserSecret,
        uint[17] calldata signature,
        uint[8] calldata proof
    ) public {
        if (isRegisteredModulus[hashedModulus] == 1) {
            revert Myna__RegisteredModulus();
        }

        bool res = registrationVerifier.verifyProof(
            [proof[0], proof[1]],
            [
                [proof[2], proof[3]], 
                [proof[4], proof[5]]
            ],
            [proof[6], proof[7]],
            [
                hashedModulus, hashedUserSecret, 
                signature[0], signature[1], signature[2],
                signature[3], signature[4], signature[5], 
                signature[6], signature[7], signature[8], 
                signature[9], signature[10], signature[11], 
                signature[12], signature[13], signature[14], 
                signature[15], signature[16]
            ]
        );
        if(!res) {
            revert Myna__InvalidSignature();
        }

        BinaryIMT.insert(mynaMainAccounts, hashedModulus);
        BinaryIMT.insert(mynaMainAccounts, hashedUserSecret);
        
        uint num = mynaMainAccounts.numberOfLeaves;
        if (num % 2 != 0) {
            revert Myna__NeedToRegisterTwoValue();
        }

        isRegisteredModulus[hashedModulus] = 1;
        mainAccounts[totalMainAccounts] = MainAccount(
            hashedModulus,
            hashedUserSecret
        );
        totalMainAccounts += 1;
        emit MainAccountAdded(totalMainAccounts, hashedModulus, hashedUserSecret);
    }

    function getMerkleTreeRoot() public view returns (uint256) {
        return mynaMainAccounts.root;
    }

    function getMerkleTreeDepth() public view returns (uint256) {
        return mynaMainAccounts.depth;
    }

    function getNumberOfMerkleTreeLeaves() public view returns (uint256) {
        return mynaMainAccounts.numberOfLeaves;
    }

    function verifyMainAccount(
        uint[8] calldata proof,
        uint identityCommitment
    ) public view returns (bool) {
        return inclusionVerifier.verifyProof(
            [proof[0], proof[1]],
            [
                [proof[2], proof[3]], 
                [proof[4], proof[5]]
            ],
            [proof[6], proof[7]],
            [identityCommitment, mynaMainAccounts.root]
        );
    }
}