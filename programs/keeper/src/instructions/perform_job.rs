use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};
use solana_program::instruction::Instruction;

use crate::error::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(job_bump: u8)]
pub struct PerformJob<'info> {
    #[account(
        seeds = [program.key().as_ref()],
        bump = job_bump,
    )]
    pub job: Box<Account<'info, Job>>,

    #[account(
        associated_token::authority = job,
        associated_token::mint = credits_mint,
    )]
    pub vault: Box<Account<'info, TokenAccount>>,

    pub credits_mint: Box<Account<'info, Mint>>,

    pub program: UncheckedAccount<'info>,
}

pub fn perform_job<'key, 'accounts, 'remaining, 'info>(
    ctx: Context<'key, 'accounts, 'remaining, 'info, PerformJob<'info>>,
    job_bump: u8,
    cpi_data: Vec<u8>,
) -> Result<()> {
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

    Ok(())
}
