import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { DepositContract } from '../target/types/deposit_contract';
import { expect } from 'chai';

describe('deposit-contract', () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.DepositContract as Program<DepositContract>;
    let depositAccount, player1Account, player2Account, gameAccount;

    before(async () => {
        depositAccount = anchor.web3.Keypair.generate();
        player1Account = anchor.web3.Keypair.generate();
        player2Account = anchor.web3.Keypair.generate();
        gameAccount = anchor.web3.Keypair.generate();

        // Log funding information
        console.log('Funding accounts...');
        for (let account of [depositAccount, player1Account, player2Account, gameAccount]) {
            const airdropTx = await provider.connection.requestAirdrop(account.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL); // Increased funding for transaction fees
            await provider.connection.confirmTransaction(airdropTx, "confirmed");
            const balance = await provider.connection.getBalance(account.publicKey);
            console.log(`Account ${account.publicKey.toString()} funded with ${balance / anchor.web3.LAMPORTS_PER_SOL} SOL`);
        }
    });

    it('Initializes the contract', async () => {
        await program.methods.initialize(provider.wallet.publicKey)
            .accounts({
                depositAccount: depositAccount.publicKey,
                user: provider.wallet.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([depositAccount])
            .rpc();

        const account = await program.account.depositAccount.fetch(depositAccount.publicKey);
        expect(account.totalDeposits.toNumber()).to.equal(0);
        expect(account.owner.toBase58()).to.equal(provider.wallet.publicKey.toBase58());
    });

    it('Handles deposits', async () => {
        const depositAmount = new anchor.BN(50000000); // Example amount
        await program.methods.deposit(depositAmount)
            .accounts({
                depositAccount: depositAccount.publicKey,
                user: player1Account.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([player1Account])
            .rpc();

        const updatedAccount = await program.account.depositAccount.fetch(depositAccount.publicKey);
        expect(updatedAccount.totalDeposits.toNumber()).to.equal(depositAmount.toNumber());
    });

    it('Handles withdrawals', async () => {
        const withdrawAmount = new anchor.BN(25000000); // Example amount
        console.log(`Attempting to withdraw ${withdrawAmount.toString()} lamports from player1Account`);
    
        // Log balance before withdrawal
        const balanceBeforeWithdrawal = await provider.connection.getBalance(player1Account.publicKey);
        console.log(`player1Account balance before withdrawal: ${balanceBeforeWithdrawal / anchor.web3.LAMPORTS_PER_SOL} SOL`);
    
        try {
            await program.methods.withdraw(withdrawAmount)
                .accounts({
                    depositAccount: depositAccount.publicKey,
                    user: player1Account.publicKey,
                    player: player1Account.publicKey,
                })
                .signers([player1Account])
                .rpc();
        } catch (error) {
            console.error("Error during withdrawal:", error);
        }
    
        // Log balance after withdrawal
        const balanceAfterWithdrawal = await provider.connection.getBalance(player1Account.publicKey);
        console.log(`player1Account balance after withdrawal: ${balanceAfterWithdrawal / anchor.web3.LAMPORTS_PER_SOL} SOL`);
    });

    it('Creates a game successfully', async () => {
        const wagerAmount = new anchor.BN(10000000); // Example wager
        console.log(`Attempting to create a game with a wager of ${wagerAmount.toString()} lamports`);
    
        try {
            await program.methods.createGame(wagerAmount)
                .accounts({
                    game: gameAccount.publicKey,
                    player1: player1Account.publicKey,
                    player2: player2Account.publicKey,
                    owner: provider.wallet.publicKey,
                    depositAccount: depositAccount.publicKey,
                })
                .signers([gameAccount, player1Account, player2Account])
                .rpc();
        } catch (error) {
            console.error("Error during game creation:", error);
        }
    });

    // Additional tests can be added as needed
});
