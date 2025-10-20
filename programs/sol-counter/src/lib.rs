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

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        msg!("Incrementing...");

        let counter = &mut ctx.accounts.counter;
        msg!("Previous count: {}", counter.count);

        // Used `checked_add` instead of `+=` for safety
        counter.count = counter
            .count
            .checked_add(1)
            .expect("Error adding 1 to count");

        msg!("Increment complete!");
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
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

// State
#[account]
pub struct Counter {
    pub count: u64,
}
impl Counter {
    pub const SIZE: usize = 8 + 8;
}
