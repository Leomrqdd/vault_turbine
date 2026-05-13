pub use anchor_lang::prelude::*;
pub use crate::state::VaultState;
pub use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        close = user,
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
    pub system_program: Program<'info, System>,
}

impl<'info> Close<'info> {
    pub fn close_vault(&mut self) -> Result<()> {
        // Drain vault PDA (system account): transfer all lamports to user.
        // SystemAccount does not implement AccountsClose; we use a CPI transfer with PDA signer.
        let amount = self.vault.to_account_info().lamports();
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };
        let seeds: &[&[&[u8]]] = &[&[
            b"vault",
            &self.vault_state.key().to_bytes(),
            &[self.vault_state.vault_bump],
        ]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, seeds);
        transfer(cpi_ctx, amount)?;
        

        // Close vault_state PDA automatically by the close = user constraint: return rent exemption to user.
        Ok(())
    }
}
