# MynaWallet monorepo

This is a mono-repo to manage MynaWallet development.

## Overview of MynaWallet

MynaWallet is an ERC-4337-compliant contract wallet service, which aims to revolutionize the way Japanese citizens interact with Ethereum by leveraging the widespread use of My Number Card, a government-issued personal identification card. This solution will provide a user-friendly platform for secure cryptocurrency transactions, fostering the growth of the Ethereum ecosystem in Japan and encouraging further innovation.

As of February 28, 2023, My Number Cards have been issued to over 80 million people, representing 64% of the Japanese population. This means that anyone with a My Number Card essentially has a contract wallet, positioning MynaWallet to potentially serve the entire Japanese population as its customer base.

We are conducting R&D on how to accomplish wallet generation and operations with My Number Card without revealing nor storing the modulus (a part of the public key held by My Number Card) in the smart contract and EIP-4337-compliant contract wallet compatible with My Number Card signatures (RSA).

## Contribution rules

- Please make sure to comment before working on an issue.
- Please fork this repo, cut your own branch from `develop`, and name your branch `<your github name>/<something meaningful>`.

## Directory structure

Pls see the respective packages for information needed to build.

```:text
/packages
├── /circom-circuits
├──── /src              # Circom circuits.
├──── /build            # Compiled circuits.
├──── /test             # Circom tests for circuit
├──── Makefile          # CLI scripts
├
├── /contracts
├──── /src              # Solidity contracts.
├──── /test             # Solidity tests for contracts.
├──── /script           # Scripts to deploy wallet.
├──── /types            # Genereted types for contracts.
├──── Makefile          # CLI scripts
├
├── /halo2-circuits
├──── /src              # Cricuits code.
├──── /spec             # Documents for circuit explanations.
├──── /certs            # Example MynaCard certificate.
├
├── /paymaster
├──── /src              # API source code.
├──── Dockerfile        # Dockerfile for API.
├──── Makefile          # CLI scripts
├
├── /script
├──── /ts               # Typescript scripts.
├──── /js               # Javascript scripts.

```
