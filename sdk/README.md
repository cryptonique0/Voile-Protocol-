# Voile Protocol SDK

TypeScript SDK for integrating with Voile Protocol on Miden.

## Installation

```bash
npm install @voile-protocol/sdk
```

**Note**: This SDK requires `@miden/client-sdk` which will be available when Miden mainnet launches in 2026. For now, this serves as the TypeScript interface specification.

## Quick Start

```typescript
import { VoileClient } from '@voile-protocol/sdk';

// Initialize client
const voile = new VoileClient('https://testnet-rpc.miden.io');

// Create an exit note
const result = await voile.createExitNote({
  unstakeAmount: BigInt('1000000000000000000'),
  unlockTimestamp: Date.now() + (7 * 24 * 60 * 60 * 1000),
  userAccountId: 'miden1...',
  feeRate: 50, // 0.5%
  minAdvanceAmount: BigInt('950000000000000000'),
});

console.log('Exit note created:', result.noteCommitment);
```

## API Reference

### VoileClient

#### `createExitNote(params: ExitNoteParams)`

Creates a private exit note for unstaking liquidity.

**Parameters:**
- `unstakeAmount`: Amount being unstaked (bigint)
- `unlockTimestamp`: When unstake unlocks (milliseconds)
- `userAccountId`: User's Miden account ID
- `feeRate`: Fee in basis points (50 = 0.5%)
- `minAdvanceAmount`: Minimum advance to accept

**Returns:**
```typescript
{
  noteCommitment: string;
  transactionId: string;
}
```

#### `advanceLiquidity(params: AdvanceLiquidityParams)`

LP advances liquidity against an exit note.

**Parameters:**
- `exitNoteCommitment`: Exit note commitment hash
- `lpAccountId`: LP's Miden account ID
- `advanceAmount`: Amount to advance

**Returns:**
```typescript
{
  transactionId: string;
  repaymentClaimId: string;
}
```

#### `settleExit(params: SettlementParams)`

Settles exit after unstake unlocks and repays LP.

**Parameters:**
- `userAccountId`: User's Miden account ID
- `lpAccountId`: LP's Miden account ID
- `repaymentAmount`: Amount to repay
- `exitNoteId`: Original exit note ID

**Returns:**
```typescript
{
  transactionId: string;
}
```

## Architecture

### Local Transaction Execution

All transactions are executed locally on the user's device:

1. User creates transaction with Miden client
2. Transaction executes locally using MASM contracts
3. ZK proof generated client-side
4. Only proof submitted to Miden operator

### Private Notes

Exit notes use Miden's private note system:

- Note commitment stored on-chain
- Note details communicated off-chain (side channels)
- LP queries commitment to validate exit note

### Account Components

Voile provides two account components:

- **User Wallet**: Creates exit notes, manages settlements
- **LP Wallet**: Advances liquidity, claims repayments

## Examples

See the `/examples` directory for complete usage examples:

- `create_exit_note.ts`: User creates exit note
- `advance_liquidity.ts`: LP advances against exit note

## Development

```bash
# Build SDK
npm run build

# Run tests
npm test

# Lint code
npm run lint
```

## License

MIT
