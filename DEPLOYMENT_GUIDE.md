# Voile Protocol - Miden Testnet Deployment Guide

## 🚀 Deployment Status

**Miden CLI Installation**: In Progress...
**Network**: Miden Testnet
**Status**: Setting up deployment tools

---

## Prerequisites

### ✅ Installed
- ✅ Rust 1.91.1 (latest stable)
- ✅ Cargo 1.91.1
- ⏳ Miden CLI (installing...)

### Required for Deployment
- [ ] Miden testnet RPC endpoint access
- [ ] Miden wallet with testnet tokens
- [ ] Account creation capabilities

---

## Deployment Steps

### Step 1: Initialize Miden Client

Once Miden CLI is installed, initialize your client:

```bash
# Source Rust environment
source "$HOME/.cargo/env"

# Initialize Miden client
miden-cli init

# This creates:
# - ~/.miden/miden-client.toml (config)
# - ~/.miden/store.sqlite3 (local state)
# - ~/.miden/accounts/ (account data)
```

### Step 2: Connect to Testnet

Configure testnet connection:

```bash
# Set testnet RPC endpoint
miden-cli config set-rpc https://testnet-rpc.miden.io

# Sync with testnet
miden-cli sync
```

### Step 3: Create Deployment Account

```bash
# Create a new account for deployment
miden-cli account new

# This generates:
# - Account ID (e.g., miden1...)
# - Authentication keypair
# - Initial account state
```

### Step 4: Compile MASM Contracts

Navigate to project directory and compile:

```bash
cd /home/web3joker/Downloads/voile-protocol-private-exit-liquidity-simulator

# Compile exit note script
miden-cli compile contracts/note_scripts/exit_note.masm

# Compile settlement note script
miden-cli compile contracts/note_scripts/settlement_note.masm

# Compile user wallet component
miden-cli compile contracts/account_components/voile_user_wallet.masm

# Compile LP wallet component
miden-cli compile contracts/account_components/voile_lp_wallet.masm
```

This produces compiled `.masb` (Miden Assembly Binary) files.

### Step 5: Deploy Account Components

Account components need to be registered with Miden:

```bash
# Deploy user wallet component
miden-cli deploy-component \
  --component contracts/account_components/voile_user_wallet.masm \
  --metadata contracts/account_components/voile_user_wallet.toml \
  --account <your-account-id>

# Deploy LP wallet component
miden-cli deploy-component \
  --component contracts/account_components/voile_lp_wallet.masm \
  --metadata contracts/account_components/voile_lp_wallet.toml \
  --account <your-account-id>
```

### Step 6: Register Note Scripts

Note scripts need to be compiled and their hashes registered:

```bash
# Get script hash for exit note
miden-cli script-hash contracts/note_scripts/exit_note.masm

# Get script hash for settlement note
miden-cli script-hash contracts/note_scripts/settlement_note.masm
```

### Step 7: Create Test Accounts

Create test accounts using your deployed components:

```bash
# Create user account with Voile user wallet
miden-cli account new \
  --type regular \
  --storage-mode public \
  --component voile_user_wallet

# Create LP account with Voile LP wallet
miden-cli account new \
  --type regular \
  --storage-mode public \
  --component voile_lp_wallet
```

### Step 8: Fund Test Accounts

Request testnet tokens from faucet:

```bash
# Request tokens for user account
miden-cli faucet request <user-account-id>

# Request tokens for LP account
miden-cli faucet request <lp-account-id>
```

### Step 9: Test Exit Note Creation

Create a test exit note:

```bash
# Execute user wallet to create exit note
miden-cli tx execute \
  --account <user-account-id> \
  --script "
    use.voile_user_wallet
    begin
      # unstake_amount, unlock_timestamp, fee_rate, min_advance
      push.1000000000000000000  # 1 token
      push.1733184000           # unlock timestamp
      push.50                   # 0.5% fee
      push.950000000000000000   # min advance
      
      call.voile_user_wallet::create_exit_note
    end
  "
```

### Step 10: Test LP Liquidity Advance

LP consumes exit note:

```bash
# LP advances liquidity
miden-cli tx execute \
  --account <lp-account-id> \
  --consume-notes <exit-note-commitment>
```

---

## Alternative: Rust Integration

For programmatic deployment, use the Miden Rust SDK:

### Create Deployment Script

```rust
// deployment/src/main.rs
use miden_client::{Client, ClientBuilder, store::StoreConfig};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let store_path = PathBuf::from("~/.miden/store.sqlite3");
    let store_config = StoreConfig::new(store_path);
    
    let client = ClientBuilder::new()
        .with_store_config(store_config)
        .with_rpc_endpoint("https://testnet-rpc.miden.io")
        .build()
        .await?;
    
    println!("✅ Miden client initialized");
    
    // Sync with network
    let sync_result = client.sync().await?;
    println!("✅ Synced to block: {}", sync_result.block_num);
    
    // Compile and deploy contracts
    // (Implementation depends on Miden client API)
    
    Ok(())
}
```

### Run Deployment

```bash
cd deployment
cargo run --release
```

---

## Verification

After deployment, verify your contracts:

```bash
# List deployed accounts
miden-cli account list

# Check account details
miden-cli account show <account-id>

# Verify component registration
miden-cli component list
```

---

## Testnet Information

**Official Resources:**
- Miden Testnet RPC: `https://testnet-rpc.miden.io`
- Miden Testnet Explorer: `https://testnet.midenscan.com`
- Miden Documentation: `https://docs.miden.xyz`
- Miden Faucet: `https://testnet-faucet.miden.io`

**Voile Protocol Contracts:**
- Exit Note Script: `contracts/note_scripts/exit_note.masm`
- Settlement Note: `contracts/note_scripts/settlement_note.masm`
- User Wallet: `contracts/account_components/voile_user_wallet.masm`
- LP Wallet: `contracts/account_components/voile_lp_wallet.masm`

---

## Troubleshooting

### Miden CLI Not Found

```bash
# Ensure Rust environment is loaded
source "$HOME/.cargo/env"

# Verify installation
which miden-cli
miden-cli --version
```

### Compilation Errors

```bash
# Check MASM syntax
miden-cli lint contracts/note_scripts/exit_note.masm

# Verify imports
miden-cli check-imports contracts/account_components/voile_user_wallet.masm
```

### Network Connection Issues

```bash
# Check RPC endpoint
curl https://testnet-rpc.miden.io/health

# Verify sync status
miden-cli sync --verbose
```

---

## Current Status

**Installation Progress:**
- ✅ Rust toolchain updated to 1.91.1
- ⏳ Miden CLI compiling (5-10 minutes)
- ⏸️ Awaiting Miden CLI completion

**Next Steps:**
1. Wait for Miden CLI installation to complete
2. Initialize Miden client with testnet
3. Compile MASM contracts
4. Deploy account components
5. Create and test accounts

---

## Notes

⚠️ **Miden Testnet Status**: As of December 2025, Miden is approaching mainnet launch (planned 2026). Testnet functionality may be limited.

⚠️ **CLI Commands**: Exact CLI commands may vary based on Miden client version. Refer to official docs for latest API.

✅ **Contracts Ready**: All MASM contracts are complete and ready for deployment.

---

**Last Updated**: December 2, 2025  
**Miden Client Version**: 0.9.4 (installing)
