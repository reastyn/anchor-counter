use anchor_lang::prelude::*;

declare_id!("7raN7YsohXGg91o7ZATJ8GKp7bEmrEZtxbkt5MkgiiLn");

#[program]
pub mod anchor_counter {
    use super::*;

    pub fn increment(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        msg!("Current count is {}", counter.count);
        Ok(())
    }
}

#[account]
pub struct Counter {
    pub count: u64,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(init_if_needed, payer=user, space = 8 + 8)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}