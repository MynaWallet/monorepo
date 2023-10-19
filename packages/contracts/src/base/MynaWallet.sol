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
import {SolRsaVerify} from "@libraries/RsaVerify.sol";
import {Errors} from "@libraries/Errors.sol";
import { IRsaVerifier } from "../interfaces/IRsaVerifier.sol";

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
    using SolRsaVerify for bytes32;

    address constant RSA_VERIFIER_ADDRESS = 0xE042A2524DF80121dD25B40D1749D39dC34B27FF;

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
     * @notice Check if the givin signature is valid
     * @param hashed hashed data
     * @param sig signature
     * @param exp exponent of the RSA public key
     * @param mod modulus of the RSA public key
     * @return 0 if valid
     */
    function verifyPkcs1Sha256(bytes32 hashed, bytes memory sig, bytes memory exp, bytes memory mod)
        public
        view
        returns (uint256)
    {
        return hashed.pkcs1Sha256Verify(sig, exp, mod);
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
        bytes32 hashed = sha256(abi.encode(userOpHash));
        uint256 hashedInU256 = uint256(hashed);
        (uint[2] memory _pA, uint[2][2] memory _pB, uint[2] memory _pC, uint[21] memory _pubSignals) = parseBytesToVerifierInput(userOp.signature);
        require(_pubSignals[20] == hashedInU256, "Invalid user");
        bool ret = IRsaVerifier(RSA_VERIFIER_ADDRESS).verifyProof(
            _pA,
            _pB,
            _pC,
            _pubSignals
        );
        if (!ret) return SIG_VALIDATION_FAILED;
    }

    function parseBytesToVerifierInput(bytes memory bytesData) 
        internal 
        pure 
        returns (uint[2] memory _pA, uint[2][2] memory _pB, uint[2] memory _pC, uint[21] memory _pubSignals) 
    {
        require(bytesData.length == 928, "Invalid input!");
        uint cnt = 0;
        for (uint i = 0; i < 2; i++) {
            _pA[i] = sliceUint(bytesData, cnt * 32);
            cnt ++;
        }

        for (uint i = 0; i < 2; i++) {
            for (uint j = 0; j < 2; j++) {
                _pB[i][j] = sliceUint(bytesData, cnt * 32);
                cnt ++;
            }
        }

        for (uint i = 0; i < 2; i++) {
            _pC[i] = sliceUint(bytesData, cnt * 32);
            cnt ++;
        }

        for (uint i = 0; i < 21; i++) {
            _pubSignals[i] = sliceUint(bytesData, cnt * 32);
            cnt ++;
        }
    }

    function sliceUint(bytes memory bs, uint start)
        internal pure
        returns (uint)
    {
        require(bs.length >= start + 32, "slicing out of range");
        uint x;
        assembly {
            x := mload(add(bs, add(0x20, start)))
        }
        return x;
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
