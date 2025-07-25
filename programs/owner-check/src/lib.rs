use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("2JzQxdnKh6RXwACK6desuPrsbk6Yd3ky4UHAPXQFFC9w");

const DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod owner_check {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        ctx.accounts.vault.token_account = ctx.accounts.token_account.key();
        ctx.accounts.vault.authority = ctx.accounts.authority.key();
        Ok(())
    }

    pub fn insecure_withdraw(ctx: Context<InsecureWithdraw>) -> Result<()> {
        if ctx.accounts.vault.authority != ctx.accounts.authority.key() {
            return Err(ProgramError::IllegalOwner.into());
        }

        let amount = ctx.accounts.token_account.amount;

        let seeds = &[
            b"token",
            ctx.accounts.vault.key.as_ref(),
            &[ctx.bumps.token_account],
        ];
        let signer = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.token_account.to_account_info(),
                to: ctx.accounts.withdraw_destination.to_account_info(),
            },
            signer,
        );

        token::transfer(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn secure_withdraw(ctx: Context<SecureWithdraw>) -> Result<()> {
        let amount = ctx.accounts.token_account.amount;

        let seeds = &[
            b"token",
            ctx.accounts.vault.key.as_ref(),
            &[ctx.bumps.token_account],
        ];
        let signer = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.token_account.to_account_info(),
                to: ctx.accounts.withdraw_destination.to_account_info(),
            },
            signer,
        );

        token::transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(
        init,
        payer = authority,
        space = DISCRIMINATOR_SIZE + Vault::INIT_SPACE,
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        init,
        payer = authority,
        token::mint = mint,
        token::authority = token_account,
        seeds = [b"token", vault.key().as_ref()],
        bump,
    )]
    pub token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct InsecureWithdraw<'info> {
    #[account()]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        seeds = [b"token", vault.key().as_ref()],
        bump,
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub withdraw_destination: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct SecureWithdraw<'info> {
    #[account(has_one = token_account, has_one = authority)]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        seeds = [b"token", vault.key().as_ref()],
        bump,
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub withdraw_destination: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub authority: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub token_account: Pubkey,
    pub authority: Pubkey,
}
