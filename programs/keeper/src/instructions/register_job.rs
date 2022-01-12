use std::mem::size_of;

use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::error::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(job_bump: u8)]
pub struct RegisterJob<'info> {
    #[account(
        init,
        seeds = [program.key().as_ref()],
        bump = job_bump,
        payer = deposit_authority,
        space = 8 + size_of::<Job>()
    )]
    pub job: Box<Account<'info, Job>>,

    #[account(
        init,
        associated_token::authority = job,
        associated_token::mint = credits_mint,
        payer = deposit_authority
    )]
    pub vault: Box<Account<'info, TokenAccount>>,

    pub program: UncheckedAccount<'info>,

    #[account(mut)]
    pub deposit_authority: Signer<'info>,

    pub credits_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        constraint = deposit_token.owner == deposit_authority.key(),
    )]
    pub deposit_token: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> RegisterJob<'info> {
    pub fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, token::Transfer<'info>> {
        let program = self.token_program.to_account_info();
        let accounts = token::Transfer {
            from: self.deposit_token.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.deposit_authority.to_account_info(),
        };
        CpiContext::new(program, accounts)
    }
}

pub fn register_job(ctx: Context<RegisterJob>, job_bump: u8, amount: u64) -> Result<()> {
    let job = &mut ctx.accounts.job;
    job.program = ctx.accounts.program.key();
    token::transfer(ctx.accounts.transfer_ctx(), amount)?;
    Ok(())
}
