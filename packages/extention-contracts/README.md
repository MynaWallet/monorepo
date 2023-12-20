### Test

```bash
forge test
```

#### with printing execution traces for failing tests

```bash
forge test -vvv
```

#### with gas reporting

```bash
forge test --gas-report
```

## Deployment

By executing the script below, you can deploy the Factory Contract of MynaWallet. You can change the network to which you deploy by changing `rpc-url` and `etherscan-api-key`.

### Inclusion Verifier

```bash
forge script script/Deploy.s.sol:DeployMainMynaInclusionVerifier --broadcast --rpc-url ${SEPOLIA_RPC_URL} --verify --etherscan-api-key ${ETHER_SCAN_API_KEY}
```

### Registration Verifier

```bash
forge script script/Deploy.s.sol:DeployMainMynaRegistrationVerifier --broadcast --rpc-url ${SEPOLIA_RPC_URL} --verify --etherscan-api-key ${ETHER_SCAN_API_KEY}
```

### Main Tree

```bash
forge script script/Deploy.s.sol:DeployMainMynaTree --broadcast --rpc-url ${SEPOLIA_RPC_URL} --verify --etherscan-api-key ${ETHER_SCAN_API_KEY}
```