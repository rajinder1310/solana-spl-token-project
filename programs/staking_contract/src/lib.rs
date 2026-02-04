use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use anchor_lang::solana_program::pubkey;

declare_id!("9vF8iR37L3nKtBR4x6mhy8dE8eMLUzcuCNbSCGCpnYHG");

// Admin Key (Deployer): Sirf ye wallet hi 'initialize' call kar sakta hai.
const ADMIN_PUBKEY: Pubkey = pubkey!("HfLwDVax4RaftkctDGGw5a84jheVZtSint919Xy9D3dD");

#[program]
pub mod staking_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // 1. Verify ki call karne wala ADMIN hi hai
        require_keys_eq!(ctx.accounts.payer.key(), ADMIN_PUBKEY, ErrorCode::Unauthorized);
        msg!("Staking Vault Initialized Successfully!");
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);
        let staker = &mut ctx.accounts.staker;
        let stake_info = &mut ctx.accounts.stake_info;

        // 1. Token Transfer Logic (User -> Vault)
        let transfer_instruction = token::Transfer {
            from: ctx.accounts.staker_token_account.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: staker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
        );

        token::transfer(cpi_ctx, amount)?;

        // 2. Update User Record
        //    Agar pehli baar hai to init hoga, agar doosri baar hai to add hoga.
        stake_info.amount += amount;
        stake_info.deposit_ts = Clock::get()?.unix_timestamp; // Current Time Note kiya

        // 3. Emit Event (Log)
        emit!(TokensStaked {
            staker: staker.key(),
            amount,
            total_staked: stake_info.amount,
        });

        msg!("Staked {} tokens successfully. Total: {}", amount, stake_info.amount);
        Ok(())
    }

    pub fn withdraw(_ctx: Context<Withdraw>) -> Result<()> {

        Ok(())
    }
}

// ----------------- ERRORS -----------------
#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("Amount must be greater than zero.")]
    InvalidAmount,
}

// ----------------- STRUCTS -----------------

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,                        // 1. Solana ko bata rahe hain ki ye NAYA account banaana hai.
        payer = payer,               // 2. Is account ko banane ki fees (Storage Rent) 'payer' dega.

        // 3. SEEDS: Ye sabse important hai using PDA.
        //    "vault" (ek fixed word) + mint (token ka address) milakar ek unique address banega.
        //    Iska fayda ye hai ki hum hamesha same "Vault" dhoondh sakte hain bina address save kiye.
        seeds = [b"vault", mint.key().as_ref()],

        bump,                        // 4. Collision avoid karne ke liye system ek 'bump' number find karega.

        token::mint = mint,          // 5. Ye Vault kis type ka token store karega? (Only USDC, or Only SOL etc.)
        token::authority = vault,    // 6. AUTHORITY: Is Vault ka malik kaun?
                                     //    Humne kaha 'vault' khud. (PDA). Koi insaan isse paise nahi nikal sakta.
    )]
    pub vault: Account<'info, TokenAccount>, // -> Ye asli Tijori (Vault) banegi.

    pub mint: Account<'info, Mint>,   // -> Token ka blueprint (e.g. USDC ka main address).

    #[account(mut)]
    pub payer: Signer<'info>,                // -> Wo user jo transaction sign kar raha hai aur fees bharega.

    pub system_program: Program<'info, System>,      // -> Naya account create karne wala department.
    pub token_program: Program<'info, Token>,        // -> Token account setup karne wala department.
    pub rent: Sysvar<'info, Rent>,                   // -> Rent calculate karne wala department.
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub staker: Signer<'info>,       // -> Jo user apne paise stake karna chahta hai (Sign zaroori hai).

    #[account(
        mut,                         // -> Vault change hoga (paise aayenge), isliye 'mut'.
        seeds = [b"vault", mint.key().as_ref()], // -> Hum confirm kar rahe hain ki ye WAHI sahi vault hai.
        bump,                                    //    Agar seeds match nahi hue, to transaction fail hogi. (Fake vault prevention)
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,             // -> Agar account nahi hai to banao, hai to use karo
        payer = staker,             // -> Fees staker dega
        space = 8 + 8 + 8,          // -> Space: Discriminator(8) + Amount(8) + Timestamp(8)
        seeds = [b"user", staker.key().as_ref()], // -> Unique ID for User
        bump
    )]    pub stake_info: Account<'info, UserStakeInfo>, // -> User ka naya ledger accounts


    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub staker_token_account: Account<'info, TokenAccount>, // -> User ka wo wallet jahan abhi tokens pade hain.

    pub token_program: Program<'info, Token>, // -> Transfer karne ke liye Token Program chahiye.
    pub system_program: Program<'info, System>, // System Program chahiye account banane ke liye
}

#[derive(Accounts)]
pub struct Withdraw {}

// ----------------- DATA ACCOUNTS -----------------

#[account]
pub struct UserStakeInfo {
    pub amount: u64,        // Kitna paisa jama hai
    pub deposit_ts: i64,    // Kab jama kiya (Timestamp)
}

// ----------------- EVENTS -----------------
#[event]
pub struct TokensStaked {
    pub staker: Pubkey,
    pub amount: u64,
    pub total_staked: u64,
}
