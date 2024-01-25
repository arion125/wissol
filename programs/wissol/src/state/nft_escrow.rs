use anchor_lang::prelude::*;

#[account]
pub struct NftEscrow {
    pub seed: u64,
    pub bump: u8,
    pub nft_mint: Pubkey,
}

impl Space for NftEscrow {
    const INIT_SPACE: usize = 8 + 8 + 1 + 32;
}