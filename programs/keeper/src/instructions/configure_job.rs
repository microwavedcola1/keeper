use anchor_lang::prelude::*;

use crate::error::*;
use crate::state::*;

#[derive(Accounts)]
pub struct ConfigureJob<'info> {
    #[account(
        mut,
        has_one = authority
    )]
    pub job: Box<Account<'info, Job>>,
    pub authority: Signer<'info>,
}

pub fn configure_job(ctx: Context<ConfigureJob>, execution_payout: u64) -> Result<()> {
    let job = &mut ctx.accounts.job;

    job.is_configured = true;
    job.execution_payout = execution_payout;

    Ok(())
}
