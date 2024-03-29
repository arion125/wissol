use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::transfer;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;

use crate::*;

// create ref unbreakable link and transfer funds from payer to: escrow, project, wissol

/* 

    The idea is good, you need to make the fee_vault an ATA if you're doing token. SystemAccount work only to
    contain SOL. Probably you can just hardcode the keypair [adderss = ... ]
    and send it as UncheckedAccount or TokenAccount.

    To make the code more clean i'm gonna suggest some name changing

    What do you need Referee for?

*/
#[derive(Accounts)]
pub struct TransferRef<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = payer,
    )]
    pub payer_token_ata: Account<'info, TokenAccount>, // PayerAta
 
    #[account(
        init_if_needed,
        seeds = [REFEREE_SEED.as_bytes(), payer.key().as_ref()],
        payer = payer,
        space = Referee::INIT_SPACE,
        bump,
    )]
    pub payer_referee_account: Account<'info, Referee>, 

    pub referral_account: Account<'info, Referral>, 

    pub mint: Account<'info, Mint>,

    // This is not needed because accomplish the same thing as the escrow_mint_ata go into intialize_referral.rs and see the comment
    pub nft_escrow: Account<'info, NftEscrow>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = nft_escrow,
    )]
    pub escrow_mint_ata: Account<'info, TokenAccount>,

    // This needs to be checked in some way tbh or i can just put whatever i want as an account and pocket the amount.
    pub project: SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = project,
    )]
    pub project_mint_ata: Account<'info, TokenAccount>,

    pub fee_account: SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = fee_account,
    )]
    pub fee_mint_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
}

impl<'info> TransferRef<'info> {
    // Super exploitable put the amount here as input !!!!! I can just say that the amount is 10x the real amount and with a 10% royalties i drain the signer
    pub fn transfer_ref(ctx: Context<TransferRef>, amount: u64) -> Result<()> {
        let decimals = 10u64.pow(ctx.accounts.mint.decimals as u32);
        let total_amount = amount * decimals;
        let referrer_amount = total_amount * ctx.accounts.referral_account.percentage as u64 / 100;
        let main_amount = total_amount - referrer_amount;

        let transfer_to_project_accounts = Transfer {
            from: ctx.accounts.payer_token_ata.to_account_info(),
            to: ctx.accounts.project_mint_ata.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
       
        let transfer_to_escrow_accounts = Transfer {
            from: ctx.accounts.payer_token_ata.to_account_info(),
            to: ctx.accounts.escrow_mint_ata.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };

        let cpi_context_project = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_to_project_accounts,
        );
        let cpi_context_escrow = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_to_escrow_accounts,
        );

        transfer(cpi_context_project, main_amount)?;
        transfer(cpi_context_escrow, referrer_amount)
    }
}
