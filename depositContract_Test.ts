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

  it('Handles deposit', async () => {
    // Assuming 0.1 SOL = 100000000 lamports for the deposit
    const depositAmount = 100000000;

    // Call the deposit function
    const tx = await program.methods.deposit()
        .accounts({
            depositAccount: depositAccount.publicKey,
            user: provider.wallet.publicKey, // CLI wallet as the user
        })
        // provider.wallet will automatically sign the transaction
        .rpc();
    console.log('Deposit transaction signature', tx);

    // Fetch the account and check if the deposit was successful
    const account = await program.account.depositAccount.fetch(depositAccount.publicKey);
    console.log('Total Deposits after deposit:', account.totalDeposits.toString());
    expect(account.totalDeposits.toNumber()).equal(depositAmount);
});
});
