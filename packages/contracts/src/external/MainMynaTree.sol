//SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@zk-kit/imt.sol/contracts/internal/InternalBinaryIMT.sol";
import "../interfaces/IMainMynaRegistrationVerifier.sol";
import "../interfaces/IMainMynaInclusionVerifier.sol";

contract MainMynaTree {
    using InternalBinaryIMT for IMT;

    IMT internal mynaMainAccounts;
    IMainMynaRegistrationVerifier public registrationVerifier;
    IMainMynaInclusionVerifier public inclusionVerifier;

    struct MainAccount {
        uint256 hashedModulus;
        uint256 hashedUserSecret;
    }

    mapping(uint256 => uint256) internal isRegisteredModulus;
    mapping(uint256 => MainAccount) public mainAccounts;
    uint256 totalMainAccounts;

    constructor(
        IMainMynaRegistrationVerifier _registrationVerifier,
        IMainMynaInclusionVerifier _inclusionVerifier
    ) {
        uint256 zeroValue = uint256(keccak256(abi.encodePacked(42))) >> 8;
        mynaMainAccounts.init(16, zeroValue);
        registrationVerifier = _registrationVerifier;
        inclusionVerifier = _inclusionVerifier;
    }

    function addMainAccount(
        uint hashedModulus, 
        uint hashedUserSecret,
        uint[17] calldata signature,
        uint[2] calldata _pA,
        uint[2][2] calldata _pB,
        uint[2] calldata _pC
    ) public {
        if (isRegisteredModulus[hashedModulus] == 1) {
            revert Myna__RegisteredModulus();
        }

        uint[19] calldata _publicInputs;
        _publicInputs.push(hashedModulus);
        _publicInputs.push(hashedUserSecret);
        for (uint i = 0; i < 17; i++) {
            _publicInputs.push(signature[i]);
        }
        if (!registrationVerifier.verifyProof(_pA, _pB, _pC, _publicInputs)) {
            revert Myna__InvalidSignature();
        }

        mynaMainAccounts.insert(hashedModulus);
        mynaMainAccounts.insert(hashedUserSecret);
        
        uint256 merkleTreeRoot = getMerkleTreeRoot();
        uint256 index = getNumberOfMerkleTreeLeaves() - 1;
        if (index % 2 == 1) {
            revert Myna__NeedToRegisterTwoValue();
        }

        isRegisteredModulus[hashedModulus] = 1;
        mainAccounts[mainAccountsNum] = MainAccount(
            hashedModulus,
            hashedUserSecret
        );
        mainAccountsNum += 1;
        emit MainAccountAdded(index - 1, hashedModulus, hashedUserSecret);
    }

    function getMerkleTreeRoot() public view virtual override returns (uint256) {
        return mynaMainAccounts.root;
    }

    function getMerkleTreeDepth(uint256 groupId) public view virtual override returns (uint256) {
        return mynaMainAccounts.depth;
    }

    function getNumberOfMerkleTreeLeaves() public view virtual override returns (uint256) {
        return mynaMainAccounts.numberOfLeaves;
    }

    function verifyMainAccount(
        uint[2] calldata _pA,
        uint[2][2] calldata _pB,
        uint[2] calldata _pC,
        uint identityCommitment
    ) public view returns (bool) {
        return inclusionVerifier.verifyProof(_pA, _pB, _pC, [identityCommitment, mynaMainAccounts.root]);
    }
}