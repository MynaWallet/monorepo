# `send_zk_uop.ts` Script Detailed Manual

## Overview

The `send_zk_uop.ts` script is designed for generating and sending user operations using zero-knowledge proofs on Ethereum-based blockchains. Specifically, it includes the following functionalities:

- Generation of sender addresses
- Generation of call data
- Creation and signing of user operations
- Gas estimation
- Sending user operations and checking the results

## Prerequisites

- **Network Connection**: Access to the Mumbai Test Network is required.
- **Deployment of ZKMynaWalletFactory**: The ZKMynaWalletFactory contract must be deployed.
- **Deployment of MynaWalletPaymaster**: The Paymaster contract must be deployed.
- **Deposit to MynaWalletPaymaster**: The Paymaster contract must be supplied with necessary funds.
- **Deployment of MynaWalletVerifier**: The verifier contract for zero-knowledge proofs must be deployed.
- **Building `packages/circom-circuits`**: Required circuits must be built.
- **Environment Variable Setup**: Necessary environment variables must be set in the `.env` file.
- **Node.js**: Must be installed.

## Setting Environment Variables

Set the following environment variables in the `.env` file at the project root.

```env
BUNDLER_PRC_URL=<Pimlico Bundler API URL>
PUBLIC_RPC_URL=<Mumbai Test Network RPC URL>
ENTRY_POINT_ADDRESS=<EntryPoint Contract Address>
FACTORY_ADDRESS=<ZKMynaWalletFactory Contract Address>
PAYMASTER_ADDRESS=<MynaWalletPaymaster Contract Address>
VERIFIER_ADDRESS=<MynaWalletVerifier Contract Address>
PAYMASTER_OWNER_ADDRESS=<MynaWalletPaymaster Owner's Address>
PAYMASTER_SIGNER_PRIVATE_KEY=<MynaWalletPaymaster Owner's Private Key>
```

Example

```env
BUNDLER_PRC_URL="https://api.pimlico.io/v1/mumbai/rpc?apikey=YOUR_PIMLICO_API_KEY"
PUBLIC_RPC_URL="https://polygon-mumbai-pokt.nodies.app"
ENTRY_POINT_ADDRESS=0x5FF137D4b0FDCD49DcA30c7CF57E578a026d2789
FACTORY_ADDRESS=0xZKMynaWalletFactoryAddress
PAYMASTER_ADDRESS=0xMynaWalletPaymasterAddress
VERIFIER_ADDRESS=0xMynaWalletVerifierAddress
PAYMASTER_OWNER_ADDRESS=0xPaymasterOwnerAddress
PAYMASTER_SIGNER_PRIVATE_KEY=0xPaymasterOwnerPrivateKey
```

## How to Obtain Pimlico's Bundler API Key

1. Visit [Pimlico](https://www.pimlico.io/) and create an account.
2. Issue an API Key and set this value in `BUNDLER_PRC_URL`.

## Building the Script

### Installing Dependencies

```sh
npm install
```

### Executing the Build

```sh
npm run build
```

## Troubleshooting

- **AA31 paymaster deposit too low**: The deposit in the Paymaster contract is insufficient. Deposit an adequate amount of gas funds into the Paymaster contract you are using.
- **AA41 too little verificationGas**: The gas estimation for UserOperation is insufficient. Update the verificationGasLimit in UserOperation to an appropriate gas limit.
- **Invalid UserOp signature or paymaster signature**: The signature on UserOperation or Paymaster is invalid. Check if you are signing correctly and modify the signature as needed.
- **ENOENT: no such file or directory**: The required file is missing. Check the correct directory structure and the presence of necessary files.
