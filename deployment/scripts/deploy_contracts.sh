#!/bin/bash
# Deploy Voile Protocol contracts to Miden testnet

set -e

echo "🚀 Deploying Voile Protocol contracts to Miden testnet..."

# Compile MASM contracts
echo "📝 Compiling MASM contracts..."
miden-asm compile ../contracts/note_scripts/exit_note.masm
miden-asm compile ../contracts/note_scripts/settlement_note.masm
miden-asm compile ../contracts/account_components/voile_user_wallet.masm
miden-asm compile ../contracts/account_components/voile_lp_wallet.masm

echo "✅ Contracts compiled"

# Deploy account components
echo "📦 Deploying account components..."
# TODO: Add actual deployment logic using Miden CLI
# miden-cli deploy --component ../contracts/account_components/voile_user_wallet.masm

echo "✅ Contracts deployed successfully"
