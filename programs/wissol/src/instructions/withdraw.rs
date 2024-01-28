use anchor_lang::prelude::*;
use anchor_spl::{token::{TokenAccount, Mint, Token, Transfer, transfer}, associated_token::AssociatedToken};

use crate::*;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = payer,
    )]
    pub payer_mint_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        // seeds = [NFT_ESCROW_SEED.as_bytes(), nft_escrow.nft_mint.key().as_ref()],
        // bump,
    )]
    pub nft_escrow: Account<'info, NftEscrow>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = nft_escrow,
    )]
    pub escrow_mint_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        let amount = ctx.accounts.escrow_mint_ata.amount;

        let nft_mint: Pubkey = ctx.accounts.nft_escrow.nft_mint.key();

        let seeds = &[
            NFT_ESCROW_SEED.as_bytes(),
            nft_mint.as_ref(),
            &[ctx.accounts.nft_escrow.bump],        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_withdraw_accounts = Transfer {
            from: ctx.accounts.escrow_mint_ata.to_account_info(),
            to: ctx.accounts.payer_mint_ata.to_account_info(),
            authority: ctx.accounts.nft_escrow.to_account_info(),
        };

        let cpi_withdraw_context = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_withdraw_accounts,
            signer_seeds
        );

        transfer(cpi_withdraw_context, amount)
    }
}