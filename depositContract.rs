use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

declare_id!("47dheiy7CSRJF1mGP1DSiJsx83Bd1gtTLvs9SUNNvomt");

#[program]
pub mod deposit_contract {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let deposit_account = &mut ctx.accounts.deposit_account;
        deposit_account.total_deposits = 0;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>) -> Result<()> {
        const DEPOSIT_AMOUNT: u64 = 100000000; // 0.1 SOL in lamports
    
        // Transfer 0.1 SOL from the user to the deposit_account
        anchor_lang::solana_program::program::invoke(
            &system_instruction::transfer(
                &ctx.accounts.user.key(),
                &ctx.accounts.deposit_account.key(),
                DEPOSIT_AMOUNT,
            ),
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.deposit_account.to_account_info(),
            ],
        )?;
    
        // Update total deposits
        let deposit_account = &mut ctx.accounts.deposit_account;
        deposit_account.total_deposits += DEPOSIT_AMOUNT;
    
        Ok(())
    }

    pub fn get_total_deposits(ctx: Context<GetTotalDeposits>) -> Result<()> {
        let deposit_account = &ctx.accounts.deposit_account;
        msg!("Total Deposits: {}", deposit_account.total_deposits);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 40)]
    pub deposit_account: Account<'info, DepositAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub deposit_account: Account<'info, DepositAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetTotalDeposits<'info> {
    pub deposit_account: Account<'info, DepositAccount>,
}

#[account]
pub struct DepositAccount {
    pub total_deposits: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("The deposited amount is not the correct value.")]
    InvalidAmount,
}
