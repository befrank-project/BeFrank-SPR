
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer, MintTo};

declare_id!("BeFrank11111111111111111111111111111111111");

#[program]
pub mod befrank_token {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        total_supply: u64,
    ) -> Result<()> {
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.initial_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::mint_to(cpi_ctx, total_supply)?;
        Ok(())
    }

    pub fn transfer_with_fee(
        ctx: Context<TransferWithFee>,
        amount: u64,
    ) -> Result<()> {
        let fee = amount / 100;
        let welfare_fee = fee / 2;
        let staking_fee = fee / 4;
        let ecosystem_fee = fee - welfare_fee - staking_fee;

        let transfer_main = Transfer {
            from: ctx.accounts.sender.to_account_info(),
            to: ctx.accounts.receiver.to_account_info(),
            authority: ctx.accounts.sender_authority.to_account_info(),
        };
        token::transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), transfer_main), amount)?;

        for (dest, portion) in [
            (&ctx.accounts.welfare_account, welfare_fee),
            (&ctx.accounts.staking_account, staking_fee),
            (&ctx.accounts.ecosystem_account, ecosystem_fee),
        ] {
            let transfer_fee = Transfer {
                from: ctx.accounts.sender.to_account_info(),
                to: dest.to_account_info(),
                authority: ctx.accounts.sender_authority.to_account_info(),
            };
            token::transfer(CpiContext::new(ctx.accounts.token_program.to_account_info(), transfer_fee), portion)?;
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = payer, mint::decimals = 6, mint::authority = mint_authority)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub initial_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: Mint authority only
    pub mint_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct TransferWithFee<'info> {
    #[account(mut)]
    pub sender: Account<'info, TokenAccount>,
    #[account(mut)]
    pub receiver: Account<'info, TokenAccount>,
    #[account(mut)]
    pub welfare_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub staking_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub ecosystem_account: Account<'info, TokenAccount>,
    pub sender_authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}
