use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;

use crate::*;

// take nft and create an account for referral and one for escrow
#[derive(Accounts)]
pub struct InitializeReferral<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub nft_mint: Account<'info, Mint>,
    
    #[account(
        init,
        seeds = [REFERRAL_SEED.as_bytes(), nft_mint.key().as_ref()],
        payer = payer,
        space = Referral::INIT_SPACE,
        bump,
    )]
    pub referral_account: Account<'info, Referral>,
    
    #[account(
        init,
        seeds = [NFT_ESCROW_SEED.as_bytes(), nft_mint.key().as_ref()],
        payer = payer,
        space = NftEscrow::INIT_SPACE,
        bump,
    )]
    pub nft_escrow: Account<'info, NftEscrow>,
    
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
}

impl<'info> InitializeReferral<'info> {
    pub fn init(ctx: Context<InitializeReferral>, percentage: u8) -> Result<()> {
        ctx.accounts.referral_account.set_inner(Referral {
            seed: ctx.accounts.referral_account.seed,
            bump: ctx.bumps.referral_account,
            percentage,
            referrer: ctx.accounts.payer.key(),
            nft_mint: ctx.accounts.nft_mint.key()
        });

        ctx.accounts.nft_escrow.set_inner(NftEscrow {
            seed: ctx.accounts.nft_escrow.seed,
            bump: ctx.bumps.nft_escrow,
            nft_mint: ctx.accounts.nft_mint.key()
        });
        Ok(())
    }
}