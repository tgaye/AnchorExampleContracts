import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { DepositContract } from '../target/types/deposit_contract';
import { expect } from 'chai';

describe('deposit-contract', () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.DepositContract as Program<DepositContract>;

    let depositAccount;

    it('Initializes the contract', async () => {
        depositAccount = anchor.web3.Keypair.generate();

        const tx = await program.methods.initialize()
            .accounts({
                depositAccount: depositAccount.publicKey,
                user: provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([depositAccount])
            .rpc();
        console.log('Initialize transaction signature', tx);

        const account = await program.account.depositAccount.fetch(depositAccount.publicKey);
        console.log('Total Deposits:', account.totalDeposits.toString());
        expect(account.totalDeposits.toNumber()).equal(0);
    });

    it('Handles multiple deposits and withdrawal', async () => {
      const depositAmount = new anchor.BN(100000000); // 0.1 SOL

      // First Deposit
      await program.methods.deposit(depositAmount).accounts({
          depositAccount: depositAccount.publicKey,
          user: provider.wallet.publicKey,
      }).rpc();

      // Second Deposit
      await program.methods.deposit(depositAmount).accounts({
          depositAccount: depositAccount.publicKey,
          user: provider.wallet.publicKey,
      }).rpc();

      // Withdraw
      await program.methods.withdraw(depositAmount).accounts({
          depositAccount: depositAccount.publicKey,
          user: provider.wallet.publicKey,
      }).rpc();

      // Fetch the account and check deposits
      const account = await program.account.depositAccount.fetch(depositAccount.publicKey);
      console.log('Total Deposits after deposit and withdraw:', account.totalDeposits.toString());

      const expectedRemainingDeposit = new anchor.BN(100000000); // Expected remaining deposit after withdrawal
      expect(account.totalDeposits.eq(expectedRemainingDeposit)).to.be.true; // Using BN.eq for comparison
  });
});
