use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
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

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        seeds = [b"vault_state", user.key().as_ref()],
        bump,
        space = VaultState::DISCRIMINATOR.len() + VaultState::INIT_SPACE
    )]
    pub vault_state: Account<'info, VaultState>,
    #[account(
        mut,
        seeds = [b"vault", vault_state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>

}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bump:&InitializeBumps) -> Result<()> {
        let rent_exempt: u64 = Rent::get()?.minimum_balance(self.vault.to_account_info().data_len());
        self.vault_state.vault_bump = bump.vault;
        self.vault_state.state_bump = bump.vault_state;
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_context = CpiContext::new(cpi_program,cpi_accounts);

        transfer(cpi_context,rent_exempt)?;
        Ok(())
    }
}



#[derive(InitSpace)]
#[account]
pub struct VaultState {
    pub vault_bump:u8,
    pub state_bump:u8,
}




#[derive(Accounts)]
pub struct Deposit<'info> {
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

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self,amount:u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_context = CpiContext::new(cpi_program,cpi_accounts);

        transfer(cpi_context,amount)?;
        Ok(())
    }
}



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

