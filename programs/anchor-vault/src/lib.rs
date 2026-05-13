use anchor_lang::prelude::*;

mod state;
mod instructions;

use crate::instructions::*;

declare_id!("FKsP8SKH43ifF4zdmXcRj7arajzQYezUL94BgDruBtdE");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;
        Ok(())
    }
    pub fn deposit(ctx: Context<Deposit>,amount:u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn withdraw(ctx:Context<Withdraw>,amount:u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;
        Ok(())
    }

    pub fn close(ctx:Context<Close>) -> Result<()> {
        ctx.accounts.close_vault()?;
        Ok(())
    }
}











