// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/utils/cryptography/MerkleProof.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {SolRsaVerify} from "@libraries/RsaVerify.sol";

enum ValidationType {
    NONE, // 0x00 - no type
    OWNER, // 0x01 - validates with owner's RSA PKCS1.5 signature
    SESSION_KEY // 0x02 - validates with session keys' ECDSA signature
}

library SignatureValidator {
    using SolRsaVerify for bytes32;

    // Signature Invalid
    uint256 constant SIG_INVALID = 1;

    /**
     * @dev Decomposes a signature into its type and the actual validation bytes.
     * @param signature The signature to decompose.
     * @return A tuple containing the signature type and the signature bytes.
     */
    function decompose(bytes memory signature) internal pure returns (ValidationType, bytes memory) {
        if (signature.length == 0) {
            return (ValidationType.NONE, signature);
        }

        // Decode the signature type and actual signature
        (uint8 validationTypeRaw, bytes memory sig) = abi.decode(signature, (uint8, bytes));

        ValidationType validationType;
        if (validationTypeRaw == 0x01) {
            validationType = ValidationType.OWNER;
        } else if (validationTypeRaw == 0x02) {
            validationType = ValidationType.SESSION_KEY;
        } else {
            // unknown signature type is not supported
            return (ValidationType.NONE, signature);
        }

        return (validationType, sig);
    }

    /**
     * @dev Decodes a userOperation signature into its components.
     * @param signature The signature to decode.
     * @return The address and data payload of the signature.
     */
    function decodeUserOperationSignature(bytes memory signature) internal pure returns (address, bytes memory) {
        return abi.decode(signature, (address, bytes));
    }

    /**
     * @dev Check if the givin signature is valid
     * @param userOpHash hashed data
     * @param sig signature
     * @param exp exponent of the RSA public key
     * @param mod modulus of the RSA public key
     * @return 0 if valid
     */
    function verifyPkcs1Sha256(bytes32 userOpHash, bytes memory sig, bytes memory exp, bytes memory mod)
        internal
        view
        returns (uint256)
    {
        return sha256(abi.encode(userOpHash)).pkcs1Sha256Verify(sig, exp, mod);
    }

    /**
     * @dev Verifies the session key signature and checks if it is expired and in the merkle tree.
     * @param userOpHash The hash of the user operation.
     * @param signature The signature.
     * @param merkleRoot The root of the merkle tree.
     * @return validationData 0 if the session key is valid, 1 otherwise.
     */
    function verifySessionKey(bytes32 userOpHash, bytes memory signature, bytes32 merkleRoot)
        internal
        view
        returns (uint256 validationData)
    {
        (
            uint48 validUntil,
            uint48 validAfter,
            bytes memory sessionkeyData,
            bytes32[] memory merkleProof,
            bytes memory sessionKeySignature
        ) = _decodeSessionKey(signature);

        // Check if the session key is not expired
        if (block.timestamp < validAfter || block.timestamp > validUntil) {
            return SIG_INVALID;
        }

        bytes32 leaf = keccak256(abi.encodePacked(validUntil, validAfter, sessionkeyData));
        // Check if the session key is in the merkle tree
        if (!_verifySessionkeyMerkeProof(merkleProof, leaf, merkleRoot)) {
            return SIG_INVALID;
        }

        // Check if the session key signature is valid
        // Currently sessionkeyData only contains session key address, but it may be extended in the future
        address sessionKey = abi.decode(sessionkeyData, (address));
        (address signer, ECDSA.RecoverError err) =
            ECDSA.tryRecover(ECDSA.toEthSignedMessageHash(userOpHash), sessionKeySignature);
        if (err != ECDSA.RecoverError.NoError || signer != sessionKey) {
            return SIG_INVALID;
        }
    }

    /**
     * @dev Decodes a signature into its constituent session key parts.
     * @param signature The signature to decompose.
     * @return validUntil The timestamp until which the session key is valid.
     * @return validAfter The timestamp after which the session key becomes valid.
     * @return sessionkeyData The session key data.
     * @return merkleProof The merkle proof of the session key.
     * @return sessionKeySignature The signature of the session key.
     */
    function _decodeSessionKey(bytes memory signature)
        private
        pure
        returns (
            uint48 validUntil,
            uint48 validAfter,
            bytes memory sessionkeyData,
            bytes32[] memory merkleProof,
            bytes memory sessionKeySignature
        )
    {
        (validUntil, validAfter, sessionkeyData, merkleProof, sessionKeySignature) =
            abi.decode(signature, (uint48, uint48, bytes, bytes32[], bytes));
    }

    /**
     * @notice Verifies a session key Merkle proof.
     * @param merkleProof The Merkle proof to verify.
     * @param leaf The leaf node of the Merkle tree.
     * @return A boolean indicating whether the Merkle proof is valid or not.
     */
    function _verifySessionkeyMerkeProof(bytes32[] memory merkleProof, bytes32 leaf, bytes32 merkleRoot)
        private
        pure
        returns (bool)
    {
        return MerkleProof.verify(merkleProof, merkleRoot, leaf);
    }
}
