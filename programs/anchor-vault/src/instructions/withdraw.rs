use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::state::VaultState;


#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"vault_state", user.key().as_ref()],
        bump = vault_state.state_bump
    )]
    pub vault_state: Account<'info, VaultState>,
    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>

}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self,amount:u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let seeds: &[&[&[u8]]] = &[&[b"vault", &self.vault_state.key().to_bytes(), &[self.vault_state.vault_bump]]];
        let cpi_context = CpiContext::new_with_signer(
            cpi_program,
            cpi_accounts,
            seeds);
        transfer(cpi_context, amount)?;
        Ok(())
    }
}

