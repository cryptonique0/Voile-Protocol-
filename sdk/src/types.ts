/**
 * Voile Protocol TypeScript Types
 */

export interface ExitNoteParams {
  unstakeAmount: bigint;
  unlockTimestamp: number;
  userAccountId: string;
  feeRate: number; // Basis points (e.g., 50 = 0.5%)
  minAdvanceAmount: bigint;
}

export interface AdvanceLiquidityParams {
  exitNoteCommitment: string;
  lpAccountId: string;
  advanceAmount: bigint;
}

export interface SettlementParams {
  userAccountId: string;
  lpAccountId: string;
  repaymentAmount: bigint;
  exitNoteId: string;
}

export interface ExitNote {
  id: string;
  commitment: string;
  unstakeAmount: bigint;
  unlockTimestamp: number;
  feeRate: number;
  status: 'pending' | 'advanced' | 'settled';
}

export interface RepaymentClaim {
  id: string;
  lpAccountId: string;
  exitNoteId: string;
  amount: bigint;
  createdAt: number;
  status: 'active' | 'claimed';
}
