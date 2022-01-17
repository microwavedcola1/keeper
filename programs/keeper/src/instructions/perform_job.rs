use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};
use solana_program::instruction::Instruction;
use std::convert::TryInto;

use crate::error::*;
use crate::state::*;

#[derive(Accounts)]
pub struct PerformJob<'info> {
    #[account(
        has_one = program,
        has_one = credits_mint
    )]
    pub job: Box<Account<'info, Job>>,

    #[account(
        mut,
        associated_token::authority = job,
        associated_token::mint = credits_mint,
    )]
    pub vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub keeper_token: Box<Account<'info, TokenAccount>>,

    pub credits_mint: Box<Account<'info, Mint>>,

    pub program: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

impl<'info> PerformJob<'info> {
    pub fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let program = self.token_program.to_account_info();
        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.keeper_token.to_account_info(),
            authority: self.job.to_account_info(),
        };
        CpiContext::new(program, accounts)
    }
}

pub fn perform_job<'key, 'accounts, 'remaining, 'info>(
    ctx: Context<'key, 'accounts, 'remaining, 'info, PerformJob<'info>>,
    cpi_data: Vec<u8>,
) -> Result<()> {
    require!(ctx.accounts.job.is_configured, JobNotConfigured);

    // verify job instruction tag
    let ix_tag = u32::from_le_bytes(cpi_data[0..4].try_into().unwrap());
    require!(ix_tag == ctx.accounts.job.ix_tag, InvalidIxTag);

    // perform job via cpi
    let mut accounts = vec![];
    let mut account_infos = vec![ctx.accounts.program.to_account_info()];
    for account in ctx.remaining_accounts.iter() {
        accounts.push(AccountMeta {
            pubkey: *account.key,
            is_signer: account.is_signer,
            is_writable: account.is_writable,
        });
        account_infos.push(account.clone());
    }
    solana_program::program::invoke(
        &Instruction {
            program_id: *ctx.accounts.program.key,
            accounts,
            data: cpi_data,
        },
        &account_infos[..],
    )?;

    // transfer credits
    let job = &ctx.accounts.job;
    let seeds = job_seeds!(job);
    transfer(
        ctx.accounts.transfer_ctx().with_signer(&[seeds]),
        job.execution_payout,
    )?;

    Ok(())
}
