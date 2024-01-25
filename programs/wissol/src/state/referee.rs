use anchor_lang::prelude::*;

#[account]
pub struct Referee {
    pub seed: u64,
    pub bump: u8,
    pub referral_code: Pubkey,
}

impl Space for Referee {
    const INIT_SPACE: usize = 8 + 8 + 1 + 32;
}