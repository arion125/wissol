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
    // You might want to add a check that the nft_mint is a nft_mint by passing in the metadata account
    // and by probably checking the collection. Or just mint the NFT directly here in the intialize referral
    // instruction and add it to the Wissol Collection to make sure
    
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

    // You actually don't need neither the ATA program nor the token program since you're not using any token here! 
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
}

impl<'info> InitializeReferral<'info> {
    pub fn init(ctx: Context<InitializeReferral>, percentage: u8) -> Result<()> {

        /* 

            You don't need to save bumps because we can just get them by passing in bumps: InitializeReferralBumps
            Giving the possibility to give as input the percentage is really dangerous since i can actually access the 
            instruction and set that value to 100% and get all the royalties. You should probably remove that and just
            hardcode a value or the possibility of having 3 different range by passing in a u8 and mathcing the value: 

            For Example:
            
            let percentage: u8
            match randomU8 {
                0 => percentage = 5,
                1 => percentage = 10,
                2 => percentage = 20,
                _ => return Err(Err::InvalidRandomU8.into()), // write custom error 
            };

            Referral should be based on project. A referral account should be linked to your NFT and to a determined project. // TODO later it's not important now

        */

        ctx.accounts.referral_account.set_inner(Referral {
            seed: ctx.accounts.referral_account.seed,
            bump: ctx.bumps.referral_account,
            percentage,
            referrer: ctx.accounts.payer.key(),
            nft_mint: ctx.accounts.nft_mint.key()
        });

        /* 

            Probably instead of having an account here could be better to have a simple vault ATA
            (or system account if you're gonna send in SOL) and just send the refferral amount here
            Seed should be referral_account. If Ata, the auth should be the referral_accountm and you 
            need to pass the mint (USDC probably) and hardcode it with a constraint.

        */

        ctx.accounts.nft_escrow.set_inner(NftEscrow {
            seed: ctx.accounts.nft_escrow.seed,
            bump: ctx.bumps.nft_escrow,
            nft_mint: ctx.accounts.nft_mint.key()
        });
        Ok(())
    }
}