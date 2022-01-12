#[macro_use]
extern crate static_assertions;

use anchor_lang::prelude::*;

use error::*;
use instructions::*;
use state::*;

use crate::instructions::*;

pub mod error;
pub mod instructions;
pub mod state;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod keeper {
    use super::*;

    pub fn register_job(ctx: Context<RegisterJob>, job_bump: u8, amount: u64) -> Result<()> {
        instructions::register_job(ctx, job_bump, amount)
    }

    pub fn perform_job<'key, 'accounts, 'remaining, 'info>(
        ctx: Context<'key, 'accounts, 'remaining, 'info, PerformJob<'info>>,
        cpi_data: Vec<u8>,
    ) -> Result<()> {
        instructions::perform_job(ctx, cpi_data)
    }
}
