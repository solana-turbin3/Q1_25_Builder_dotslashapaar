use anchor_lang::prelude::*;

mod errors;
pub mod state;
pub mod context;

pub use state::*;
pub use context::*;

declare_id!("DeE6ezmZAt87b9bjfp63xwFZEypFCysn5nLebhSR9ZwX");

#[program]
pub mod amm_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed: u64, fee: u16, authority: Option<Pubkey>) -> Result<()> {
        ctx.accounts.init(seed, fee, authority, &ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        ctx.accounts.deposit(amount, max_x, max_y)?;
        Ok(())
    }

    pub fn swap(ctx: Context<Swap>, amount: u64, is_x: bool, min: u64) -> Result<()> {
        ctx.accounts.swap(is_x, amount, min)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>,amount: u64, min_x: u64, min_y: u64) -> Result<()> {
        ctx.accounts.withdraw(amount, min_x, min_y)?;
        Ok(())
    }
}


// 65s61y4ngkoLK2GRGqHUyed9WHoYKwqt6iMw6wQ3FS2v524S9qTU6MiNUh6dV5dUvudNPZx9BnLgh1ZKcYqqjVQB

