# Time-Locked Vault Contract

A Soroban smart contract that implements a time-locked vault for Stellar assets.

## Overview

This contract allows:
- An owner to deposit tokens
- A designated beneficiary to withdraw tokens only after a specified unlock time
- The owner to perform emergency withdrawals before the unlock time

## Features

- **Time-based locking**: Funds are locked until a specified timestamp
- **Emergency withdrawal**: Owner can reclaim funds before unlock time
- **Secure access control**: Only authorized parties can deposit/withdraw
- **View functions**: Check contract state (owner, beneficiary, unlock time, etc.)

## Contract Functions

### Initialization
- `initialize(owner, beneficiary, unlock_time, token)` - Sets up the vault with initial parameters

### Core Functions
- `deposit(from, amount)` - Owner deposits tokens into the vault
- `withdraw()` - Beneficiary withdraws funds after unlock time
- `emergency_withdraw()` - Owner reclaims funds before unlock time

### View Functions
- `get_owner()` - Returns the owner address
- `get_beneficiary()` - Returns the beneficiary address
- `get_unlock_time()` - Returns the unlock timestamp
- `get_token()` - Returns the token contract address
- `get_deposited_amount()` - Returns the current deposited amount

## Usage

### Prerequisites
- Soroban SDK (v22.0.7 or later)
- Rust toolchain

### Building
```bash
cargo build --target wasm32-unknown-unknown --release
```

### Testing
```bash
cargo test
```

### Deployment
1. Deploy the contract to the Soroban network
2. Initialize with owner, beneficiary, unlock timestamp, and token address
3. Owner can deposit tokens
4. After unlock time, beneficiary can withdraw
5. Owner can emergency withdraw if needed before unlock time

## Error Cases
- Attempting to deposit negative amounts
- Unauthorized deposit/withdrawal attempts
- Early withdrawal attempts (before unlock time)
- Initializing with owner == beneficiary

## Security Considerations
- The contract uses Soroban's authentication system (`require_auth`)
- All storage operations use persistent storage
- Critical operations include proper authorization checks

## License

This project is licensed under the MIT License. 

## Acknowledgments
- Stellar Development Foundation
- Soroban team
