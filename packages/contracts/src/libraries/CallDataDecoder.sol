// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Errors} from "./Errors.sol";

library CallDataDecoder {
    bytes4 private constant _EXECUTE_SELECTOR = bytes4(keccak256("execute(address,uint256,bytes)"));
    bytes4 private constant _EXECUTE_BATCH_SELECTOR = bytes4(keccak256("executeBatch(address[],bytes[])"));
    bytes4 private constant _EXECUTE_BATCH_WITH_VALUE_SELECTOR =
        bytes4(keccak256("executeBatch(address[],uint256[],bytes[])"));

    /**
     * @dev Decodes the call data for a MynaWallet transaction.
     * @param callData The call data to decode.
     * @return An array of destination addresses, an array of values, and an array of function call data.
     * If the call data is for a single transaction, the arrays will have length 1.
     * If the call data is for a batch transaction, the arrays will have length equal to the number of transactions in the batch.
     * If the call data is invalid, reverts with an error message.
     */
    function decodeCallData(bytes calldata callData)
        public
        pure
        returns (address[] memory, uint256[] memory, bytes[] memory)
    {
        if (_isExecuteOrExecuteBatch(callData)) revert Errors.INVALID_CALLDATA();
        bytes4 selector = bytes4(callData[0:4]);

        if (selector == _EXECUTE_SELECTOR) {
            (address dest, uint256 value, bytes memory func) = _decodeExecute(callData);
            address[] memory addresses = new address[](1);
            uint256[] memory values = new uint256[](1);
            bytes[] memory funcs = new bytes[](1);
            addresses[0] = dest;
            values[0] = value;
            funcs[0] = func;
            return (addresses, values, funcs);
        } else if (selector == _EXECUTE_BATCH_SELECTOR) {
            (address[] memory dest, bytes[] memory func) = _decodeExecuteBatch(callData);
            return (dest, new uint256[](dest.length), func);
        } else if (selector == _EXECUTE_BATCH_WITH_VALUE_SELECTOR) {
            (address[] memory dest, uint256[] memory value, bytes[] memory func) =
                _decodeExecuteBatchWithValue(callData);
            return (dest, value, func);
        } else {
            revert Errors.ENTRYPOINT_SELECTOR_MUST_BE_EXECUTE_OR_EXECUTE_BATCH(selector);
        }
    }

    /**
     * @dev Checks if the given call data represents an `execute` or `executeBatch` function call.
     * @param callData The call data to check.
     * @return A boolean indicating whether the call data represents an `execute` or `executeBatch` function call.
     */
    function _isExecuteOrExecuteBatch(bytes calldata callData) private pure returns (bool) {
        if (callData.length < 4) return false;

        bytes4 selector = bytes4(callData[0:4]);
        return selector == _EXECUTE_SELECTOR || selector == _EXECUTE_BATCH_SELECTOR
            || selector == _EXECUTE_BATCH_WITH_VALUE_SELECTOR;
    }

    /**
     * @dev Decodes the execute function call data.
     * @param callData The call data to decode.
     * @return The target address, ETH value, and data of the execute function call.
     */
    function _decodeExecute(bytes calldata callData) private pure returns (address, uint256, bytes memory) {
        return abi.decode(callData[4:], (address, uint256, bytes));
    }

    /**
     * @dev Decodes the call data for the `executeBatch` function in the `BatchExecutor` contract.
     * @param callData The encoded call data.
     * @return A tuple containing an array of target addresses and an array of corresponding call data.
     */
    function _decodeExecuteBatch(bytes calldata callData) private pure returns (address[] memory, bytes[] memory) {
        return abi.decode(callData[4:], (address[], bytes[]));
    }

    /**
     * @dev Decodes the call data for the `executeBatchWithValue` function in the `MynaWallet` contract.
     * @param callData The call data to decode.
     * @return A tuple containing the addresses, values, and data of the batched transactions.
     */
    function _decodeExecuteBatchWithValue(bytes calldata callData)
        private
        pure
        returns (address[] memory, uint256[] memory, bytes[] memory)
    {
        return abi.decode(callData[4:], (address[], uint256[], bytes[]));
    }
}
