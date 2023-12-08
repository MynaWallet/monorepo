# MynaWallet Halo2 Circuits

This repository aims to create proofs which verifies the RSA signatures signed by Myna Card(Japan's ID Card).

## Getting Started

For a brief introduction to zero-knowledge proofs (ZK), see this [doc](https://docs.axiom.xyz/zero-knowledge-proofs/introduction-to-zk).

Halo 2 is written in Rust, so you need to [install](https://www.rust-lang.org/tools/install) Rust to use this library:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone this repo and start off in the `halo2-circuits` directory.

```bash
git clone git@github.com:MynaWallet/halo2-circuits.git
cd halo2-circuits
```

## Run test

```bash
cargo test -- --nocapture
```

## Benchmarks

```bash
cargo bench
```

Result Not Yet

## Milestones

- ✅ RSA Verification Circuit Base
- RSA Verification Circuit (SHA2 Hash as input)
- Test & Benchmarks
- Verifier Contracts
- Example Codes which call Prover
- (Phase2) RSA Verification Circuit (DER-encoded certificate as input)
- (Phase3) Selective Disclosure

See more details on the [issues](https://github.com/MynaWallet/halo2-circuits/issues)

## Specs

- [Signature Verification](./spec/SignatureVerification.md)

## References

You can refer to these repos of RSA verification circuits.

- [halo2-rsa](https://github.com/zkCert/halo2-rsa)
- [zk-email-verify](https://github.com/zkemail/zk-email-verify)
- [zkCert](https://github.com/zkCert/halo2-zkcert)

## Example Usage
### Create the directory where proofs are stored
```bash
mkdir -p build/{app,agg}
```

### Generate the common reference string
```bash
cargo run trusted-setup
```

### Generate pk & vk
```bash
cargo run app keys
```

### Generate a proof
```bash
cargo run app prove
```

### Run the verification code written in Rust
```bash
cargo run app verify
```

### Run the verification code written in Solidity
```bash
cargo run app evm
```
