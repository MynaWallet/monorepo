use crate::config::Config;
use ethers::abi::Abi;
use ethers::contract::{Contract, ContractInstance};
use ethers::providers::{Http, Provider};
use ethers::signers::LocalWallet;
use ethers_core::types::Address;
use std::sync::Arc;

pub fn load_provider(config: &Config, network: &str) -> Provider<Http> {
    let endpoint = match network {
        "mumbai" => &config.MUMBAI_ENDPOINT,
        "sepolia" => &config.SEPOLIA_ENDPOINT,
        _ => &config.GOERLI_ENDPOINT,
    };

    Provider::<Http>::try_from(endpoint).unwrap()
}

pub fn load_signer(config: &Config) -> LocalWallet {
    config
        .SIGNER_SECRET_KEY
        .as_str()
        .parse::<LocalWallet>()
        .unwrap()
}

pub fn load_paymaster_contract(
    config: &Config,
    provider: Arc<Provider<Http>>,
) -> ContractInstance<Arc<Provider<ethers_providers::Http>>, Provider<ethers_providers::Http>> {
    let address = config
        .PAYMATER_CONTRACT_ADDRESS
        .as_str()
        .parse::<Address>()
        .unwrap();

    let abi = serde_json::from_str::<Abi>(
        r#"
    [
        {
            "inputs": [
            {
                "internalType": "contract IEntryPoint",
                "name": "newEntryPoint",
                "type": "address"
            },
            { "internalType": "address", "name": "_owner", "type": "address" }
            ],
            "stateMutability": "nonpayable",
            "type": "constructor"
        },
        {
            "anonymous": false,
            "inputs": [
            {
                "indexed": true,
                "internalType": "address",
                "name": "previousOwner",
                "type": "address"
            },
            {
                "indexed": true,
                "internalType": "address",
                "name": "newOwner",
                "type": "address"
            }
            ],
            "name": "OwnershipTransferred",
            "type": "event"
        },
        {
            "inputs": [],
            "name": "_POST_OP_GAS",
            "outputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
            "stateMutability": "view",
            "type": "function"
        },
        {
            "inputs": [
            { "internalType": "uint32", "name": "unstakeDelaySec", "type": "uint32" }
            ],
            "name": "addStake",
            "outputs": [],
            "stateMutability": "payable",
            "type": "function"
        },
        {
            "inputs": [
            { "internalType": "address", "name": "sender", "type": "address" }
            ],
            "name": "allowedAddresses",
            "outputs": [
            { "internalType": "bool", "name": "isAllowed", "type": "bool" }
            ],
            "stateMutability": "view",
            "type": "function"
        },
        {
            "inputs": [
            { "internalType": "address[]", "name": "addr", "type": "address[]" },
            { "internalType": "bool[]", "name": "allowed", "type": "bool[]" }
            ],
            "name": "batchSetAllowedAddresses",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
        },
        {
            "inputs": [],
            "name": "deposit",
            "outputs": [],
            "stateMutability": "payable",
            "type": "function"
        },
        {
            "inputs": [],
            "name": "entryPoint",
            "outputs": [
            { "internalType": "contract IEntryPoint", "name": "", "type": "address" }
            ],
            "stateMutability": "view",
            "type": "function"
        },
        {
            "inputs": [],
            "name": "getDeposit",
            "outputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
            "stateMutability": "view",
            "type": "function"
        },
        {
            "inputs": [
            {
                "components": [
                { "internalType": "address", "name": "sender", "type": "address" },
                { "internalType": "uint256", "name": "nonce", "type": "uint256" },
                { "internalType": "bytes", "name": "initCode", "type": "bytes" },
                { "internalType": "bytes", "name": "callData", "type": "bytes" },
                {
                    "internalType": "uint256",
                    "name": "callGasLimit",
                    "type": "uint256"
                },
                {
                    "internalType": "uint256",
                    "name": "verificationGasLimit",
                    "type": "uint256"
                },
                {
                    "internalType": "uint256",
                    "name": "preVerificationGas",
                    "type": "uint256"
                },
                {
                    "internalType": "uint256",
                    "name": "maxFeePerGas",
                    "type": "uint256"
                },
                {
                    "internalType": "uint256",
                    "name": "maxPriorityFeePerGas",
                    "type": "uint256"
                },
                {
                    "internalType": "bytes",
                    "name": "paymasterAndData",
                    "type": "bytes"
                },
                { "internalType": "bytes", "name": "signature", "type": "bytes" }
                ],
                "internalType": "struct UserOperation",
                "name": "userOp",
                "type": "tuple"
            },
            { "internalType": "uint48", "name": "validUntil", "type": "uint48" },
            { "internalType": "uint48", "name": "validAfter", "type": "uint48" },
            { "internalType": "address", "name": "erc20Token", "type": "address" },
            { "internalType": "uint256", "name": "exchangeRate", "type": "uint256" }
            ],
            "name": "getHash",
            "outputs": [{ "internalType": "bytes32", "name": "", "type": "bytes32" }],
            "stateMutability": "view",
            "type": "function"
        },
        {
            "inputs": [],
            "name": "isAllowlistEnabled",
            "outputs": [{ "internalType": "bool", "name": "", "type": "bool" }],
            "stateMutability": "view",
            "type": "function"
        },
        {
            "inputs": [],
            "name": "owner",
            "outputs": [{ "internalType": "address", "name": "", "type": "address" }],
            "stateMutability": "view",
            "type": "function"
        },
        {
            "inputs": [
            { "internalType": "bytes", "name": "paymasterAndData", "type": "bytes" }
            ],
            "name": "parsePaymasterAndData",
            "outputs": [
            { "internalType": "uint48", "name": "validUntil", "type": "uint48" },
            { "internalType": "uint48", "name": "validAfter", "type": "uint48" },
            { "internalType": "address", "name": "erc20Token", "type": "address" },
            { "internalType": "uint256", "name": "exchangeRate", "type": "uint256" },
            { "internalType": "bytes", "name": "signature", "type": "bytes" }
            ],
            "stateMutability": "pure",
            "type": "function"
        },
        {
            "inputs": [
            {
                "internalType": "enum IPaymaster.PostOpMode",
                "name": "mode",
                "type": "uint8"
            },
            { "internalType": "bytes", "name": "context", "type": "bytes" },
            { "internalType": "uint256", "name": "actualGasCost", "type": "uint256" }
            ],
            "name": "postOp",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
        },
        {
            "inputs": [],
            "name": "renounceOwnership",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
        },
        {
            "inputs": [
            { "internalType": "address", "name": "sender", "type": "address" }
            ],
            "name": "senderNonce",
            "outputs": [
            { "internalType": "uint256", "name": "nonce", "type": "uint256" }
            ],
            "stateMutability": "view",
            "type": "function"
        },
        {
            "inputs": [
            { "internalType": "address", "name": "addr", "type": "address" },
            { "internalType": "bool", "name": "allowed", "type": "bool" }
            ],
            "name": "setAllowedAddress",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
        },
        {
            "inputs": [{ "internalType": "bool", "name": "enabled", "type": "bool" }],
            "name": "setAllowlistEnabled",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
        },
        {
            "inputs": [
            { "internalType": "address", "name": "newOwner", "type": "address" }
            ],
            "name": "transferOwnership",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
        },
        {
            "inputs": [],
            "name": "unlockStake",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
        },
        {
            "inputs": [
            {
                "components": [
                { "internalType": "address", "name": "sender", "type": "address" },
                { "internalType": "uint256", "name": "nonce", "type": "uint256" },
                { "internalType": "bytes", "name": "initCode", "type": "bytes" },
                { "internalType": "bytes", "name": "callData", "type": "bytes" },
                {
                    "internalType": "uint256",
                    "name": "callGasLimit",
                    "type": "uint256"
                },
                {
                    "internalType": "uint256",
                    "name": "verificationGasLimit",
                    "type": "uint256"
                },
                {
                    "internalType": "uint256",
                    "name": "preVerificationGas",
                    "type": "uint256"
                },
                {
                    "internalType": "uint256",
                    "name": "maxFeePerGas",
                    "type": "uint256"
                },
                {
                    "internalType": "uint256",
                    "name": "maxPriorityFeePerGas",
                    "type": "uint256"
                },
                {
                    "internalType": "bytes",
                    "name": "paymasterAndData",
                    "type": "bytes"
                },
                { "internalType": "bytes", "name": "signature", "type": "bytes" }
                ],
                "internalType": "struct UserOperation",
                "name": "userOp",
                "type": "tuple"
            },
            { "internalType": "bytes32", "name": "userOpHash", "type": "bytes32" },
            { "internalType": "uint256", "name": "maxCost", "type": "uint256" }
            ],
            "name": "validatePaymasterUserOp",
            "outputs": [
            { "internalType": "bytes", "name": "context", "type": "bytes" },
            { "internalType": "uint256", "name": "validationData", "type": "uint256" }
            ],
            "stateMutability": "nonpayable",
            "type": "function"
        },
        {
            "inputs": [
            {
                "internalType": "address payable",
                "name": "withdrawAddress",
                "type": "address"
            }
            ],
            "name": "withdrawStake",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
        },
        {
            "inputs": [
            {
                "internalType": "address payable",
                "name": "withdrawAddress",
                "type": "address"
            },
            { "internalType": "uint256", "name": "amount", "type": "uint256" }
            ],
            "name": "withdrawTo",
            "outputs": [],
            "stateMutability": "nonpayable",
            "type": "function"
        }
    ]"#,
    )
    .unwrap();

    Contract::new(address, abi, provider)
}
