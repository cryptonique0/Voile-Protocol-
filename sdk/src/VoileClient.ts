/**
 * Voile Protocol Client
 * High-level interface for creating exit notes and managing liquidity
 */

import { MidenClient } from '@miden/client-sdk';
import { ExitNoteParams, SettlementParams, AdvanceLiquidityParams } from './types';

export class VoileClient {
  private midenClient: MidenClient;

  constructor(rpcEndpoint: string) {
    this.midenClient = new MidenClient(rpcEndpoint);
  }

  /**
   * Create a private exit note
   * @param params Exit note parameters
   * @returns Note commitment and transaction ID
   */
  async createExitNote(params: ExitNoteParams): Promise<{
    noteCommitment: string;
    transactionId: string;
  }> {
    // Load exit note script
    const exitNoteScript = await this.loadScript('exit_note.masm');
    
    // Create private note
    const note = await this.midenClient.createPrivateNote({
      script: exitNoteScript,
      data: {
        unstakeAmount: params.unstakeAmount,
        unlockTimestamp: params.unlockTimestamp,
        userAccountId: params.userAccountId,
        feeRate: params.feeRate,
        minAdvanceAmount: params.minAdvanceAmount,
      },
    });
    
    // Execute transaction locally
    const tx = await this.midenClient.executeTransaction({
      account: params.userAccountId,
      note: note,
      operation: 'create_exit_note',
    });
    
    // Generate proof
    const proof = await this.midenClient.generateProof(tx);
    
    // Submit to Miden operator
    const result = await this.midenClient.submitProof(proof);
    
    return {
      noteCommitment: note.commitment,
      transactionId: result.txId,
    };
  }

  /**
   * LP advances liquidity against exit note
   * @param params Advance liquidity parameters
   * @returns Transaction ID
   */
  async advanceLiquidity(params: AdvanceLiquidityParams): Promise<{
    transactionId: string;
    repaymentClaimId: string;
  }> {
    // Query exit note commitment from operator
    const noteCommitment = await this.midenClient.queryNoteCommitment(
      params.exitNoteCommitment
    );
    
    // Create transaction to consume exit note
    const tx = await this.midenClient.executeTransaction({
      account: params.lpAccountId,
      consumedNotes: [noteCommitment],
      operation: 'advance_liquidity',
    });
    
    // Generate proof
    const proof = await this.midenClient.generateProof(tx);
    
    // Submit proof
    const result = await this.midenClient.submitProof(proof);
    
    return {
      transactionId: result.txId,
      repaymentClaimId: result.outputNotes[0].id, // Settlement claim note
    };
  }

  /**
   * Settle exit after unstake unlocks
   * @param params Settlement parameters
   * @returns Transaction ID
   */
  async settleExit(params: SettlementParams): Promise<{
    transactionId: string;
  }> {
    // Load settlement note script
    const settlementScript = await this.loadScript('settlement_note.masm');
    
    // Create settlement note
    const note = await this.midenClient.createPrivateNote({
      script: settlementScript,
      data: {
        repaymentAmount: params.repaymentAmount,
        lpAccountId: params.lpAccountId,
      },
    });
    
    // Execute transaction
    const tx = await this.midenClient.executeTransaction({
      account: params.userAccountId,
      note: note,
      operation: 'settle_exit',
    });
    
    // Generate proof
    const proof = await this.midenClient.generateProof(tx);
    
    // Submit proof
    const result = await this.midenClient.submitProof(proof);
    
    return {
      transactionId: result.txId,
    };
  }

  private async loadScript(filename: string): Promise<string> {
    // Load MASM script from contracts directory
    // In production, these would be bundled or fetched from a CDN
    const fs = require('fs');
    const path = require('path');
    const scriptPath = path.join(__dirname, '../../contracts/note_scripts', filename);
    return fs.readFileSync(scriptPath, 'utf-8');
  }
}
