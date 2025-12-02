/**
 * Example: LP advances liquidity against an exit note
 */

import { VoileClient } from '@voile-protocol/sdk';

async function main() {
  // Initialize Voile client
  const voile = new VoileClient('https://testnet-rpc.miden.io');

  // Advance liquidity
  const result = await voile.advanceLiquidity({
    exitNoteCommitment: '0x...', // Exit note commitment from user
    lpAccountId: 'miden1...', // LP account ID
    advanceAmount: BigInt('950000000000000000'), // 0.95 tokens
  });

  console.log('Liquidity advanced!');
  console.log('Transaction ID:', result.transactionId);
  console.log('Repayment claim ID:', result.repaymentClaimId);
}

main().catch(console.error);
