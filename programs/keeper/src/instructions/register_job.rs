use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::error::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(job_bump: u8, ix_tag: u32)]
pub struct RegisterJob<'info> {
    #[account(
        init,
        // TODO: Could there be two job registrations with the same seed for different credits_mint?
        seeds = [authority.key().as_ref(), program.key().as_ref(), &ix_tag.to_le_bytes()],
        bump = job_bump,
        payer = payer,
    )]
    pub job: Box<Account<'info, Job>>,

    pub program: UncheckedAccount<'info>,

    pub authority: UncheckedAccount<'info>,

    pub credits_mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        associated_token::authority = job,
        associated_token::mint = credits_mint,
        payer = payer
    )]
    pub vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn register_job(ctx: Context<RegisterJob>, job_bump: u8, ix_tag: u32) -> Result<()> {
    let job = &mut ctx.accounts.job;
    job.program = ctx.accounts.program.key();
    job.credits_mint = ctx.accounts.credits_mint.key();
    job.authority = ctx.accounts.authority.key();
    job.ix_tag = ix_tag;
    job.bump = job_bump;

    Ok(())
}
