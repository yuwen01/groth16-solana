# `sp1-solana`

This crate verifies Groth16 proofs generated with SP1, leveraging Solana's BN254 precompiles for cheap on-chain verification.

> [!CAUTION]
>
> This repository is under active development and is not yet ready for production use.

## Overview

- **Groth16 Proof Verification**: Implements the Groth16 protocol for zero-knowledge proof verification.
- **Solana BN254 Precompiles**: Leverages Solana's native BN254 precompiles for optimized performance.
- **Easy Integration**: Seamlessly integrates with existing Solana programs and infrastructure.
- **Extensible**: Built with modularity in mind, allowing for future enhancements and integrations.

## Installation

Add `sp1-solana` to your `Cargo.toml`:

```toml
[dependencies]
sp1-solana = { git = "https://github.com/succinctlabs/sp1-solana" }
```

If your crate isn't running as a solana contract, you can optionally enable the `sp1-serialize` feature. 
This allows an [`SP1ProofWithPublicValues`](https://docs.rs/sp1-sdk/2.0.0/sp1_sdk/proof/struct.SP1ProofWithPublicValues.html)
to be serialized into a `SP1ProofFixture`. 

## Usage

See the [example](./example/README.md) for a full example.

## Acknowledgements
This crate uses the [`groth16-solana`](https://github.com/Lightprotocol/groth16-solana/) crate from Light Protocol Labs for the actual Groth16 proof verification, and the [`ark-bn254`](https://github.com/arkworks-rs/algebra) crate for the elliptic curve operations.