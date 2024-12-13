# C3Caller - Soroban Implementation

## Overview
C3Caller  Protocol is a cross-chain communication protocol implemented on the Soroban smart contract platform for the Stellar network. 

## Contracts
The protocol consists of several core contracts:

- `C3Caller`: Manages cross-chain calls and message passing
- `C3Governor`: Handles protocol governance and parameter updates
- `C3UUIDKeeper`: Maintains unique identifiers for cross-chain transactions
- `C3GovClient`: Base contract for governance functionality

## Features
- Cross-chain message passing
- Unique transaction identifier management
- Governance system with operator management
- Pausable functionality for emergency situations
- Event logging for transaction tracking
- Fallback mechanism for failed transactions

## Prerequisites
- Rust toolchain
- Soroban CLI
- Stellar network access (testnet/mainnet)

## Installation

1. Install Rust and Cargo
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install Soroban CLI
```bash
cargo install --locked soroban-cli
```

3. Clone the repository
```bash
git clone https://github.com/ContinuumDAO/c3caller-stellar
cd c3caller-stellar
```

4. Build the contracts
```bash
cargo build --target wasm32-unknown-unknown --release
```

## Testing
Run the test suite:
```bash
cargo test
```

## Contract Deployment

1. Build the contracts:
```bash
stellar contract build
```

2. Deploy to the Stellar testnet:
```bash
stellar contract deploy \
    --wasm target/wasm32-unknown-unknown/release/c3caller.wasm \
    --source <your-secret-key> \
    --network testnet
```



