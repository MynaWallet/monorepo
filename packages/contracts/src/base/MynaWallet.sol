// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts/proxy/utils/UUPSUpgradeable.sol";
import "@account-abstraction/contracts/core/BaseAccount.sol";
import "@auth/Auth.sol";
import "@account-abstraction/contracts/samples/callback/TokenCallbackHandler.sol";
import "@managers/OwnerManager.sol";
import "@managers/EntryPointManager.sol";
import "@managers/EIP1271Manager.sol";
import {Errors} from "@libraries/Errors.sol";
import {SignatureValidator, SignatureType} from "@utils/SignatureValidator.sol";

/// @title MynaWallet
/// @author a42x
/// @notice You can use this contract for ERC-4337 compiant wallet which works with My Number Card
contract MynaWallet is
    BaseAccount,
    Auth,
    EntryPointManager,
    OwnerManager,
    EIP1271Manager,
    TokenCallbackHandler,
    UUPSUpgradeable,
    Initializable
{
    using SignatureValidator for bytes;

    // @notice Event which will be emitted when this contract is initalized
    event MynaWalletInitialized(IEntryPoint indexed entryPoint, bytes modulus);

    /// @notice recieve fallback function
    receive() external payable {}

    /**
     * @notice Constructor of MynaWallet (Only used by factory contract)
     * @dev Cusntuctor is only used when factory is deployed and the facotry holds wallet implementation address
     * @param newEntryPoint EntryPoint contract address that can operate this contract
     */
    constructor(IEntryPoint newEntryPoint) EntryPointManager(newEntryPoint) {
        _disableInitializers();
    }

    /**
     * @dev The _entryPoint member is immutable, to reduce gas consumption.  To upgrade EntryPoint,
     * a new implementation of MynaWallet must be deployed with the new EntryPoint address, then upgrading
     * the implementation by calling `upgradeTo()`
     * @param newModulus modulus of the RSA public key which can operate this contract
     */
    function initialize(bytes memory newModulus) external initializer {
        _setOwner(newModulus);
        emit MynaWalletInitialized(entryPoint(), newModulus);
    }

    /**
     * @notice Execute a transaction (called directly from entryPoint)
     * @param dest target address
     * @param value value to send
     * @param func function call data
     */
    function execute(address dest, uint256 value, bytes calldata func) external onlyEntryPoint {
        _call(dest, value, func);
    }

    /**
     * @notice Execute a sequence of transactions (called directory from by entryPoint)
     * @param dest target addresses
     * @param func function call data
     */
    function executeBatch(address[] calldata dest, bytes[] calldata func) external onlyEntryPoint {
        if (dest.length != func.length) revert Errors.INVALID_ARRAY_LENGTH(dest.length, 0, func.length);
        for (uint256 i = 0; i < dest.length;) {
            _call(dest[i], 0, func[i]);
            unchecked {
                i++;
            }
        }
    }

    /**
     * execute a sequence of transactions
     * @param dest target addreses
     * @param value value to send
     * @param func function call data
     */
    function executeBatch(address[] calldata dest, uint256[] calldata value, bytes[] calldata func)
        external
        onlyEntryPoint
    {
        if (dest.length != value.length || value.length != func.length) {
            revert Errors.INVALID_ARRAY_LENGTH(dest.length, value.length, func.length);
        }
        for (uint256 i = 0; i < dest.length;) {
            _call(dest[i], value[i], func[i]);
            unchecked {
                i++;
            }
        }
    }

    /**
     * @notice Deposit more funds for this account in the entryPoint
     * @dev This function is payable
     */
    function addDeposit() public payable {
        entryPoint().depositTo{value: msg.value}(address(this));
    }

    /**
     * @notice Withdraw value from the account's deposit
     * @param withdrawAddress target to send to
     * @param amount to withdraw
     */
    function withdrawDepositTo(address payable withdrawAddress, uint256 amount) public onlySelf {
        entryPoint().withdrawTo(withdrawAddress, amount);
    }

    /**
     * @notice Get the entryPoint contract address
     * @return entryPoint contract address
     */
    function entryPoint() public view override(BaseAccount) returns (IEntryPoint) {
        return EntryPointManager._entryPoint();
    }

    /**
     * @notice Check current account deposit in the entryPoint
     * @return deposit amount
     */
    function getDeposit() public view returns (uint256) {
        return entryPoint().balanceOf(address(this));
    }

    /**
     * @notice Validate UserOperation and its signature, currently only supports RSA signature
     * @dev Internal function
     * @param userOp user operation
     * @param userOpHash hash of the user operation
     * @return validationData 0 if valid
     */
    function _validateSignature(UserOperation calldata userOp, bytes32 userOpHash)
        internal
        virtual
        override
        returns (uint256 validationData)
    {
        if (userOp.signature.length == 0) return SIG_VALIDATION_FAILED;

        // Decode signature
        (address validator, bytes memory signature) = userOp.signature.decodeUserOoerationSignature();
        // Currently only supports self signature validator
        if (validator != address(this)) {
            return SIG_VALIDATION_FAILED;
        }

        // Decompose signature
        (SignatureType signatureType, bytes memory sig) = signature.decompose();

        if (signatureType == SignatureType.RSA) {
            (bytes memory modulus, bytes memory exponent) = getOwner();
            uint256 ret = SignatureValidator.verifyPkcs1Sha256(userOpHash, sig, exponent, modulus);
            if (ret != 0) return SIG_VALIDATION_FAILED;
        } else if (signatureType == SignatureType.ECDSA) {
            // TODO
            return SIG_VALIDATION_FAILED;
        } else {
            return SIG_VALIDATION_FAILED;
        }
    }

    /**
     * @notice Call a contract with arbitrary data and value
     * @dev Internal function
     * @param target target address
     * @param value value to send
     * @param data function call data
     */
    function _call(address target, uint256 value, bytes memory data) internal {
        (bool success, bytes memory result) = target.call{value: value}(data);
        if (!success) {
            assembly {
                revert(add(result, 32), mload(result))
            }
        }
    }

    /**
     * @notice Upgrade this contract to a new implementation
     * @dev Internal function
     * @param newImplementation new implementation address
     */
    function _authorizeUpgrade(address newImplementation) internal view override onlySelf {
        // TODO add time lock?
        (newImplementation);
    }
}
