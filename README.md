<p align="center">
  <img src="https://github.com/xJonathanLEI/starknet-rs/blob/master/images/starknet-rs-logo.png?raw=true" alt="Logo"/>
  <h1 align="center">starknet-rs</h1>
</p>

**Complete StarkNet library in Rust**

![starknet-version-v0.8.2](https://img.shields.io/badge/StarkNet_Version-v0.8.2-2ea44f?logo=ethereum)
[![linting-badge](https://github.com/xJonathanLEI/starknet-rs/actions/workflows/lint.yaml/badge.svg?branch=master)](https://github.com/xJonathanLEI/starknet-rs/actions/workflows/lint.yaml)
[![crates-badge](https://img.shields.io/crates/v/starknet.svg)](https://crates.io/crates/starknet)

> _Note that `starknet-rs` is still experimental. Breaking changes will be made before the first stable release. The library is also NOT audited or reviewed for security at the moment. Use at your own risk._

> _The underlying cryptography library [`starknet-crypto`](./starknet-crypto) does NOT provide constant-time guarantees._

## Adding starknet-rs to your project

Note that the [crates.io version](https://crates.io/crates/starknet) is currently outdated, and you're advised to use it from GitHub directly instead:

```toml
[dependencies]
starknet = { git = "https://github.com/xJonathanLEI/starknet-rs" }
```

## Features

- [x] Sequencer gateway / feeder gateway client
- [ ] Full node JSON-RPC API client
- [x] Smart contract deployment
- [x] Signer for using [IAccount](https://github.com/OpenZeppelin/cairo-contracts/blob/main/src/openzeppelin/account/IAccount.cairo) account contracts
- [ ] Strongly-typed smart contract binding code generation from ABI

## Crates

This workspace contains the following crates:

- `starknet`: Re-export of other sub-crates (recommended)
- `starknet-core`: Core data structures for interacting with StarkNet
- `starknet-providers`: Abstraction and implementation of clients for interacting with StarkNet nodes and sequencers
- `starknet-contract`: Types for deploying and interacting with StarkNet smart contracts
- `starknet-crypto`: **Low-level** cryptography utilities for StarkNet
- `starknet-signers`: StarkNet signer implementations
- `starknet-accounts`: Types for handling StarkNet account abstraction
- `starknet-ff`: StarkNet field element type

## Example

Examples can be found in the [examples folder](./examples):

1. [Get the latest block from `alpha-goerli` testnet](./examples/get_block.rs)

2. [Deploy contract to `alpha-goerli` testnet](./examples/deploy_contract.rs)

3. [Mint yourself 1,000 TST tokens on `alpha-goerli`](./examples/mint_tokens.rs)

   Make sure your account has some L2 Goerli ETH to pay for the transaction fee. You can use [this faucet](https://faucet.goerli.starknet.io/) to fund your account.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
