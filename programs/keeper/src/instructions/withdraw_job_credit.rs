use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

use crate::error::*;
use crate::state::*;

#[derive(Accounts)]
pub struct WithdrawJobCredit<'info> {
    #[account(
        has_one = credits_mint,
        has_one = authority,
    )]
    pub job: Box<Account<'info, Job>>,

    pub credits_mint: Box<Account<'info, Mint>>,

    pub authority: Signer<'info>,

    #[account(
        mut,
        associated_token::authority = job,
        associated_token::mint = credits_mint,
    )]
    pub vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = destination.mint == credits_mint.key(),
    )]
    pub destination: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> WithdrawJobCredit<'info> {
    pub fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let program = self.token_program.to_account_info();
        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.destination.to_account_info(),
            authority: self.job.to_account_info(),
        };
        CpiContext::new(program, accounts)
    }
}

pub fn withdraw_job_credit(ctx: Context<WithdrawJobCredit>, amount: u64) -> Result<()> {
    let seeds = job_seeds!(&ctx.accounts.job);
    transfer(ctx.accounts.transfer_ctx().with_signer(&[seeds]), amount)?;

    Ok(())
}
