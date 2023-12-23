use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;

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
            Some(deposit) => {
                deposit.amount += amount;
            },
            None => {
                deposit_account.user_deposits.push(UserDeposit {
                    user: user_key,
                    amount,
                });
            },
        }

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let user_key = ctx.accounts.user.key();
        let mut deposit_found = false;
    
        // Check if the user has a deposit large enough to withdraw the requested amount
        for deposit in ctx.accounts.deposit_account.user_deposits.iter_mut() {
            if deposit.user == user_key {
                require!(deposit.amount >= amount, ErrorCode::InsufficientFunds);
                deposit_found = true;
    
                deposit.amount -= amount;
                ctx.accounts.deposit_account.total_deposits -= amount;
                break;
            }
        }
    
        if !deposit_found {
            return Err(ErrorCode::InsufficientFunds.into());
        }
    
        // Check if the deposit_account has enough lamports to cover the withdrawal
        let deposit_account_lamports = **ctx.accounts.deposit_account.to_account_info().lamports.borrow();
        require!(deposit_account_lamports >= amount, ErrorCode::InsufficientFunds);
    
        // Perform the lamport transfer
        **ctx.accounts.user.to_account_info().lamports.borrow_mut() += amount;
        **ctx.accounts.deposit_account.to_account_info().lamports.borrow_mut() -= amount;
    
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
    #[account(init, payer = user, space = 8 + 40 + 500)] 
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
    #[account(init, payer = user, space = 8 + 64)]
    pub player: Account<'info, Player>, // This account tracks the wins and losses
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub deposit_account: Account<'info, DepositAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub player: Account<'info, Player>, // Reference to update the Player account
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

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
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
}

#[error_code]
pub enum ErrorCode {
    #[msg("The deposited amount is not the correct value.")]
    InvalidAmount,
    #[msg("Insufficient funds for withdrawal.")]
    InsufficientFunds,
}
