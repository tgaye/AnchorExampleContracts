use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
use std::mem::size_of;

declare_id!("47dheiy7CSRJF1mGP1DSiJsx83Bd1gtTLvs9SUNNvomt");

#[program]
pub mod deposit_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let deposit_account = &mut ctx.accounts.deposit_account;
        deposit_account.total_deposits = 0;
        deposit_account.user_deposits = Vec::new();
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        // Transfer funds from user to deposit account
        anchor_lang::solana_program::program::invoke(
            &system_instruction::transfer(
                &ctx.accounts.user.key(),
                &ctx.accounts.deposit_account.key(),
                amount,
            ),
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.deposit_account.to_account_info(),
            ],
        )?;
    
        let deposit_account = &mut ctx.accounts.deposit_account;
        deposit_account.total_deposits += amount;
    
        let user_key = ctx.accounts.user.key();
        match deposit_account.user_deposits.iter_mut().find(|deposit| deposit.user == user_key) {
            Some(deposit) => deposit.amount += amount,
            None => deposit_account.user_deposits.push(UserDeposit {
                user: user_key,
                amount,
            }),
        }
    
        // Initialize or update the player account
        let player = &mut ctx.accounts.player;
            // Inside your deposit function
            if player.balance == 0 {
                // No need to manually set the key or lamports; Anchor handles initialization
                player.balance = amount;
            } else {
                // Update player's balance
                player.balance += amount;
            }
    
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let deposit_account = &mut ctx.accounts.deposit_account;
        let user_key = ctx.accounts.user.key();
        
        if let Some(deposit) = deposit_account.user_deposits.iter_mut().find(|deposit| deposit.user == user_key) {
            
            if deposit.amount < amount {
                return Err(ErrorCode::InsufficientFunds.into());
            }

            deposit.amount -= amount;
            deposit_account.total_deposits -= amount;

            **ctx.accounts.user.to_account_info().lamports.borrow_mut() += amount;
        } else {
            return Err(ErrorCode::InsufficientFunds.into());
        }

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
    #[account(init, payer = user, space = 8 + 40 + 500)] // Adjusted space for vector
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
    #[account(init, payer = user, space = 8 + size_of::<Player>(), seeds = [user.key().as_ref()], bump)]
    pub player: Account<'info, Player>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub deposit_account: Account<'info, DepositAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetTotalDeposits<'info> {
    pub deposit_account: Account<'info, DepositAccount>,
}

#[account]
pub struct DepositAccount {
    pub total_deposits: u64,
    pub user_deposits: Vec<UserDeposit>,
}

#[account]
pub struct UserDeposit {
    pub user: Pubkey,
    pub amount: u64,
}

#[account]
pub struct Player {
    pub balance: u64,
    pub wager: u64,
    pub wins: u64,
    pub losses: u64,
    pub total_bet: u64,
    pub total_won: u64,
    pub total_lost: u64,
    pub bump: u8, 
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid amount")]
    InvalidAmount,

    #[msg("Insufficient funds")]
    InsufficientFunds,

    #[msg("Player account mismatch")]
    PlayerAccountMismatch,

    #[msg("Bump mismatch")]
    BumpMismatch,
}