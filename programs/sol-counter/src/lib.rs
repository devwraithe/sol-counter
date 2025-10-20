#![allow(deprecated)] // Suppress deprecated warnings
#![allow(unexpected_cfgs)] // Suppress unrecognized cfg flag warnings

use anchor_lang::prelude::*;

declare_id!("HAYtTjRugrXeNSJ2jTjPBCgncEbSteQojVZZQ9zQYuE7");

#[program]
pub mod sol_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Initializing...");

        let counter = &mut ctx.accounts.counter;
        counter.count = 0;

        msg!("Initialize complete!");
        msg!("Current count: {}", counter.count);

        Ok(())
    }

    pub fn increment_one(ctx: Context<Update>) -> Result<()> {
        msg!("Incrementing 1...");

        let counter = &mut ctx.accounts.counter;
        msg!("Previous count: {}", counter.count);

        // Used `checked_add` instead of `+=` for safety
        counter.count = counter
            .count
            .checked_add(1)
            .expect("Error adding 1 to count");

        msg!("Increment 1 complete!");
        msg!("Current count: {}", counter.count);

        Ok(())
    }

    pub fn decrement_one(ctx: Context<Update>) -> Result<()> {
        msg!("Decrementing 1...");

        let counter = &mut ctx.accounts.counter;
        msg!("Previous count: {}", counter.count);

        // Used `checked_sub`` instead of `-=` for safety
        counter.count = counter
            .count
            .checked_sub(1)
            .expect("Error removing 1 from count");

        msg!("Decrement 1 complete!");
        msg!("Current count: {}", counter.count);

        Ok(())
    }
    pub fn increment_any(ctx: Context<Update>, value: u64) -> Result<()> {
        msg!("Incrementing {}...", value);

        require!(value > 0, MyError::InvalidCountValue);

        let counter = &mut ctx.accounts.counter;
        msg!("Previous count: {}", counter.count);

        counter.count = counter
            .count
            .checked_add(value)
            .expect(&format!("Error adding {} to count", value));

        msg!("Increment {} complete!", value);
        msg!("Current count: {}", counter.count);

        Ok(())
    }

    pub fn decrement_any(ctx: Context<Update>, value: u64) -> Result<()> {
        msg!("Decrementing {}...", value);

        require!(value > 0, MyError::InvalidCountValue);

        let counter = &mut ctx.accounts.counter;
        msg!("Previous count: {}", counter.count);

        counter.count = counter
            .count
            .checked_sub(value)
            .expect(&format!("Error removing {} from count", value));

        msg!("Decrement {} complete!", value);
        msg!("Current count: {}", counter.count);

        Ok(())
    }
}

// Accounts
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = Counter::SIZE,
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

/// State
#[account]
pub struct Counter {
    pub count: u64,
}
impl Counter {
    pub const SIZE: usize = 8 + 8;
}

/// Errors
#[error_code]
pub enum MyError {
    #[msg("Invalid count value")]
    InvalidCountValue,
}
