use anchor_lang::prelude::*;

#[account]
pub struct Referral {
    pub seed: u64,
    pub bump: u8,
    pub percentage: u8,
    pub referrer: Pubkey,
    pub nft_mint: Pubkey
}

impl Space for Referral {
    const INIT_SPACE: usize = 8 + 8 + 1 + 1 + 32 + 32;
}