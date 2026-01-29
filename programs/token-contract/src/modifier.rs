use anchor_lang::prelude::*;
use std::str::FromStr;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// --------------------------------------------------------
// 1. Constants (Hardcoded Admin)
// --------------------------------------------------------
// In production, we assume this address will never change.
// If you want a changeable owner, use a Config Account (State).
const ADMIN_PUBKEY: &str = "Hu5Vq145t4546454564564564564564564564564564";

#[program]
pub mod standard_access_control {
    use super::*;

    pub fn admin_only_function(ctx: Context<AdminOnly>) -> Result<()> {
        msg!("Success: Admin access granted.");
        Ok(())
    }
}

// --------------------------------------------------------
// 2. Validation Structs (The "Gatekeeper")
// --------------------------------------------------------
#[derive(Accounts)]
pub struct AdminOnly<'info> {
    #[account(
        mut,
        // ✅ STANDARD ACCESS CONTROL (Hardcoded)
        // This is the clean, declarative way to restrict access in modern Anchor.
        // It checks: Signer Key == ADMIN_PUBKEY
        address = Pubkey::from_str(ADMIN_PUBKEY).unwrap() @ AdminError::NotAuthorized
    )]
    pub admin: Signer<'info>,
}

// --------------------------------------------------------
// 3. Custom Errors
// --------------------------------------------------------
#[error_code]
pub enum AdminError {
    #[msg("You are not authorized to perform this action.")]
    NotAuthorized,
}
