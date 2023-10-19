# RSA Signature Verification Circuit

The Circuit which Verifies RSA signature.

## Overview

This is the circuit overview.
(The input will change in phase2.)

```mermaid
flowchart LR
    hashed-->verify_signature
    signature-->verify_signature
    modulus-->verify_signature
    subgraph circuit
    verify_signature
    end
    verify_signature-->ok
```

## Functions

- `verify_signature()`: verify RSA signature and return assigned set values if signature is valid

## How verify_signature() works

(add it later)