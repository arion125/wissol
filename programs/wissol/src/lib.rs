pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("64drnnxTSEe9fpeTEQ3MeVtm1Vztzf7bR4Cigs9X7S5j");

#[program]
pub mod wissol {
    use super::*;
    
    pub fn initialize_referral(ctx: Context<InitializeReferral>, percentage: u8) -> Result<()> {
        InitializeReferral::init(ctx, percentage)
    }

    pub fn transfer_ref(ctx: Context<TransferRef>, amount: u64) -> Result<()> {
        TransferRef::transfer_ref(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        Withdraw::withdraw(ctx)
    }
}
