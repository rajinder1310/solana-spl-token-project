use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo, Transfer};

declare_id!("UY89vX8nRLbuy8LZCZy9ThMbNp1669Bi7Ue9uMwZC6P");

#[program]
pub mod token_contract {
    use super::*;

    pub fn initialize_mint(ctx: Context<InitializeMint>, decimals: u8) -> Result<()> {
        msg!("Token mint initialized: {:?}", ctx.accounts.mint.key());
        emit!(MintInitialized {
            mint: ctx.accounts.mint.key(),
            decimals,
            authority: ctx.accounts.payer.key(),
        });
        Ok(())
    }

    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, amount)?;
        msg!("Minted {} tokens to {:?}", amount, ctx.accounts.token_account.key());
        emit!(TokensMinted {
            mint: ctx.accounts.mint.key(),
            to: ctx.accounts.token_account.key(),
            amount,
        });
        Ok(())
    }

    pub fn transfer_token(ctx: Context<TransferToken>, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
        msg!("Transferred {} tokens from {:?} to {:?}", amount, ctx.accounts.from.key(), ctx.accounts.to.key());
        emit!(TokensTransferred {
            from: ctx.accounts.from.key(),
            to: ctx.accounts.to.key(),
            amount,
            authority: ctx.accounts.authority.key(),
        });
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(decimals: u8)]
pub struct InitializeMint<'info> {
    #[account(
        init,
        payer = payer,
        mint::decimals = decimals,
        mint::authority = payer,
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct TransferToken<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[event]
pub struct MintInitialized {
    pub mint: Pubkey,
    pub decimals: u8,
    pub authority: Pubkey,
}

#[event]
pub struct TokensMinted {
    pub mint: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
}

#[event]
pub struct TokensTransferred {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
    pub authority: Pubkey,
}
