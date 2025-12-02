/**
 * Example: Create an exit note with Voile Protocol
 */

import { VoileClient } from '@voile-protocol/sdk';

async function main() {
  // Initialize Voile client
  const voile = new VoileClient('https://testnet-rpc.miden.io');

  // Create exit note
  const result = await voile.createExitNote({
    unstakeAmount: BigInt('1000000000000000000'), // 1 token
    unlockTimestamp: Date.now() + (7 * 24 * 60 * 60 * 1000), // 7 days
    userAccountId: 'miden1...', // Your Miden account ID
    feeRate: 50, // 0.5% fee
    minAdvanceAmount: BigInt('950000000000000000'), // Min 0.95 tokens
  });

  console.log('Exit note created!');
  console.log('Note commitment:', result.noteCommitment);
  console.log('Transaction ID:', result.transactionId);
}

main().catch(console.error);
