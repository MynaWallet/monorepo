// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/utils/cryptography/MerkleProof.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {SolRsaVerify} from "@libraries/RsaVerify.sol";

enum SignatureType {
    NONE, // 0x00 - no signature
    RSA, // 0x01 - RSA PKCS1.5 signature
    ECDSA // 0x02 - ECDSA signature
}

library SignatureValidator {
    using SolRsaVerify for bytes32;

    /**
     * @dev Decodes a userOperation signature into its components.
     * @param signature The signature to decode.
     * @return The address and data payload of the signature.
     */
    function decodeUserOoerationSignature(bytes memory signature) internal pure returns (address, bytes memory) {
        return abi.decode(signature, (address, bytes));
    }

    // decompose signature to (signatureType, signature)
    function decompose(bytes memory signature) internal pure returns (SignatureType, bytes memory) {
        if (signature.length == 0) {
            return (SignatureType.NONE, signature);
        }
        uint8 signatureTypeRaw = uint8(signature[0]);
        SignatureType signatureType;

        if (signatureTypeRaw == 0x01) {
            signatureType = SignatureType.RSA;
        } else if (signatureTypeRaw == 0x02) {
            signatureType = SignatureType.ECDSA;
        } else {
            // unknown signature type is not supported
            return (SignatureType.NONE, signature);
        }

        bytes memory sig = new bytes(signature.length - 1);
        for (uint256 i = 0; i < sig.length; i++) {
            sig[i] = signature[i + 1];
        }
        return (signatureType, sig);
    }

    /**
     * @notice Check if the givin signature is valid
     * @param userOpHash hashed data
     * @param sig signature
     * @param exp exponent of the RSA public key
     * @param mod modulus of the RSA public key
     * @return 0 if valid
     */
    function verifyPkcs1Sha256(bytes32 userOpHash, bytes memory sig, bytes memory exp, bytes memory mod)
        public
        view
        returns (uint256)
    {
        return sha256(abi.encode(userOpHash)).pkcs1Sha256Verify(sig, exp, mod);
    }
}
