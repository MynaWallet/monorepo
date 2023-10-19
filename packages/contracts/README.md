# MynaWallet

## Overview

MynaWallet smart contract is a contract account that complies with [ERC-4337](https://eips.ethereum.org/EIPS/eip-4337). It approves operations described in `UserOperation` by verifying RSA signatures.

## Preparation

MynaWallet smart contract is developed using [Foundry](https://book.getfoundry.sh/getting-started/installation). Please build the [Foundry](https://book.getfoundry.sh/getting-started/installation) development environment according to the procedures in the linked page. After that, clone this repository with submodules.

 Make a copy of `.env.sample` and name it `.env`. Then, please set the environment variables according to the comments in the file. Please do not edit `.env.sample` directly as it is a sample file.

```bash
cp .env.sample .env
# Then edit .env with your favorite editor.
```

## Development

### Local Development

#### Start a local node

You can start a local node with the following command. The node will provide a JSON-RPC endpoint at `http://localhost:8545` and test accounts.

```bash
anvil
```

Edit .env file to set `PRIVATE_KEY` from the given test account and reload .env file.
```bash
source .env
```

#### Deploy to local node

```bash
forge script script/Deploy.s.sol:DeployLocal --broadcast --rpc-url ${LOCAL_RPC_URL}
```

#### Debugging

For example, you can call `accountImplementation()` function to get the address of the account implementation address. Replace `YOUR_DEPLOYED_FACTORY_CONTRACT_ADDRESS` with the address of the deployed contract. For more information, please refer to [Cast](https://book.getfoundry.sh/cast/).

```bash
cast call YOUR_DEPLOYED_FACTORY_CONTRACT_ADDRESS "accountImplementation()(address)" --rpc-url ${LOCAL_RPC_URL}
```

### Compile

```bash
forge build --sizes
```

### Test

#### with printing execution traces for failing tests

```bash
forge test --vvv
```

#### with gas reporting

```bash
forge test --gas-report
```

## Deployment

By executing the script below, you can deploy the Factory Contract of MynaWallet. You can change the network to which you deploy by changing `rpc-url` and `etherscan-api-key`.

### Factory

```bash
forge script script/Deploy.s.sol:DeployFactory --broadcast --rpc-url ${SEPOLIA_RPC_URL} --verify --etherscan-api-key ${SEPOLIA_SCAN_API_KEY}
```

### Paymaster

```bash
forge script script/Deploy.s.sol:DeployPaymaster --broadcast --rpc-url ${SEPOLIA_RPC_URL} --verify --etherscan-api-key ${SEPOLIA_SCAN_API_KEY}
```
