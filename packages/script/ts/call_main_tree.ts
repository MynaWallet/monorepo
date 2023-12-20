import fs from "fs";
import { createPublicClient, createWalletClient, http } from 'viem';
import { sepolia } from 'viem/chains';
import { privateKeyToAccount } from 'viem/accounts';

async function main() {
    const pA = [
        "0x0944bf8476ec0e2fb5fc5c2c3d89bc24327b24cdae68d90f43f0f470265530ea",
        "0x0744432fdaf9cb8bbd8167c34459ff635ac6095cfe40f16f4f1d86f479931d6d"
    ];
    const pB = [
        [
            "0x0889b8f19acb81a04d0aedb4f0753953f28f96197902b6bab96da8c78852d903",
            "0x04cb432b23cd835a01e2f9ae918713ea294007df17fcc29b7d6d146d6d694903"
        ],
        [
            "0x0b5f1a1b9de34e6541b11091196951df62099344c1970a50dc8c6ca5ef7e29ad",
            "0x0e190a8c79602164f50742d521a60bc249708f4efeaeaa025cd0aad4d487676d"
        ]
    ];
    const pC = [
        "0x0966104e73b94b19532893c136d99467327dd75b398cd49dda71522249e6c10e",
        "0x057675b17b471acd6eb4cc67e1c1ce3b3c03333fbd72d696ed30886c244cc37a"
    ];
    const hashedModulus = "0x2072591f673451919d3fa8156fb6c9df2d80cc3cb62608bdf82561f2e54d386c";
    const hashedUserSecrets = "0x1b408dafebeddf0871388399b1e53bd065fd70f18580be5cdde15d7eb2c52743";
    const signature = [
        "0x000000000000000000000000000000000083311788a4e1d06dc95e250e478766",
        "0x000000000000000000000000000000000093e38ad902da3297380f79b12233cd",
        "0x000000000000000000000000000000000046a924089f34dee57e0a9639731c87",
        "0x000000000000000000000000000000000003a0e529e13da9cdc5efeaf109728d",
        "0x00000000000000000000000000000000008c1efe3e88fb7bb4d9396b0546487d",
        "0x00000000000000000000000000000000016a39afa35f13f2c6f84d545b95d685",
        "0x000000000000000000000000000000000109eb2a1f1ce80a155e8c3fe6785787",
        "0x000000000000000000000000000000000076fa188240c667d79bcfdf845907c4",
        "0x00000000000000000000000000000000009a23684e5d7cd82e16390ab6684760",
        "0x0000000000000000000000000000000001d83db4ee4146d2e57c722d5f5f8bfc",
        "0x0000000000000000000000000000000001aab3cd19cf8c4daf948162e8848569",
        "0x000000000000000000000000000000000144ac614e342576d57026dbbfe60779",
        "0x000000000000000000000000000000000005a11d7772956b0ffe8c3c6ba9c7b9",
        "0x0000000000000000000000000000000000be43c7c54166eb89655c378f8b6502",
        "0x00000000000000000000000000000000011159115c8d7fb043ad18a5856050f2",
        "0x000000000000000000000000000000000079b47140da069c6fb503c61dbe03f9",
        "0x0000000000000000000000000000000000000e59f28ba37e188c6b9c9915fd9d"
    ];

    const account = privateKeyToAccount("");
    const rpcUrl = "";
    const walletClient = createWalletClient({
        account,
        chain: sepolia,
        transport: http(rpcUrl),
    });
    const publicClient = createPublicClient({
        chain: sepolia,
        transport: http(rpcUrl),
    });
    const abi = [
        {
            "type": "constructor",
            "inputs": [
                {
                    "name": "_registrationVerifier",
                    "type": "address",
                    "internalType": "contract IMainMynaRegistrationVerifier"
                },
                {
                    "name": "_inclusionVerifier",
                    "type": "address",
                    "internalType": "contract IMainMynaInclusionVerifier"
                }
            ],
            "stateMutability": "nonpayable"
        },
        {
            "type": "function",
            "name": "addMainAccount",
            "inputs": [
                {
                    "name": "hashedModulus",
                    "type": "uint256",
                    "internalType": "uint256"
                },
                {
                    "name": "hashedUserSecret",
                    "type": "uint256",
                    "internalType": "uint256"
                },
                {
                    "name": "signature",
                    "type": "uint256[17]",
                    "internalType": "uint256[17]"
                },
                {
                    "name": "_pA",
                    "type": "uint256[2]",
                    "internalType": "uint256[2]"
                },
                {
                    "name": "_pB",
                    "type": "uint256[2][2]",
                    "internalType": "uint256[2][2]"
                },
                {
                    "name": "_pC",
                    "type": "uint256[2]",
                    "internalType": "uint256[2]"
                }
            ],
            "outputs": [],
            "stateMutability": "nonpayable"
        },
        {
            "type": "function",
            "name": "getMerkleTreeDepth",
            "inputs": [],
            "outputs": [
                {
                    "name": "",
                    "type": "uint256",
                    "internalType": "uint256"
                }
            ],
            "stateMutability": "view"
        },
        {
            "type": "function",
            "name": "getMerkleTreeRoot",
            "inputs": [],
            "outputs": [
                {
                    "name": "",
                    "type": "uint256",
                    "internalType": "uint256"
                }
            ],
            "stateMutability": "view"
        },
        {
            "type": "function",
            "name": "getNumberOfMerkleTreeLeaves",
            "inputs": [],
            "outputs": [
                {
                    "name": "",
                    "type": "uint256",
                    "internalType": "uint256"
                }
            ],
            "stateMutability": "view"
        },
        {
            "type": "function",
            "name": "inclusionVerifier",
            "inputs": [],
            "outputs": [
                {
                    "name": "",
                    "type": "address",
                    "internalType": "contract IMainMynaInclusionVerifier"
                }
            ],
            "stateMutability": "view"
        },
        {
            "type": "function",
            "name": "mainAccounts",
            "inputs": [
                {
                    "name": "",
                    "type": "uint256",
                    "internalType": "uint256"
                }
            ],
            "outputs": [
                {
                    "name": "hashedModulus",
                    "type": "uint256",
                    "internalType": "uint256"
                },
                {
                    "name": "hashedUserSecret",
                    "type": "uint256",
                    "internalType": "uint256"
                }
            ],
            "stateMutability": "view"
        },
        {
            "type": "function",
            "name": "registrationVerifier",
            "inputs": [],
            "outputs": [
                {
                    "name": "",
                    "type": "address",
                    "internalType": "contract IMainMynaRegistrationVerifier"
                }
            ],
            "stateMutability": "view"
        },
        {
            "type": "function",
            "name": "verifyMainAccount",
            "inputs": [
                {
                    "name": "_pA",
                    "type": "uint256[2]",
                    "internalType": "uint256[2]"
                },
                {
                    "name": "_pB",
                    "type": "uint256[2][2]",
                    "internalType": "uint256[2][2]"
                },
                {
                    "name": "_pC",
                    "type": "uint256[2]",
                    "internalType": "uint256[2]"
                },
                {
                    "name": "identityCommitment",
                    "type": "uint256",
                    "internalType": "uint256"
                }
            ],
            "outputs": [
                {
                    "name": "",
                    "type": "bool",
                    "internalType": "bool"
                }
            ],
            "stateMutability": "view"
        },
        {
            "type": "event",
            "name": "MainAccountAdded",
            "inputs": [
                {
                    "name": "index",
                    "type": "uint256",
                    "indexed": false,
                    "internalType": "uint256"
                },
                {
                    "name": "hashedModulus",
                    "type": "uint256",
                    "indexed": false,
                    "internalType": "uint256"
                },
                {
                    "name": "hashedUserSecret",
                    "type": "uint256",
                    "indexed": false,
                    "internalType": "uint256"
                }
            ],
            "anonymous": false
        },
        {
            "type": "error",
            "name": "Myna__InvalidSignature",
            "inputs": []
        },
        {
            "type": "error",
            "name": "Myna__NeedToRegisterTwoValue",
            "inputs": []
        },
        {
            "type": "error",
            "name": "Myna__RegisteredModulus",
            "inputs": []
        }
    ];
    const input = [hashedModulus, hashedUserSecrets, signature, pA, pB, pC];
    const hash = await walletClient.writeContract({
        abi,
        functionName: 'addMainAccount',
        address: "0x4060CC431aF6cbC6D02e9061b02a3118D5a2BAc8",
        args: input,
    });
    await publicClient.waitForTransactionReceipt({
        hash,
    });
    console.log(`${hash}`);
}

main();