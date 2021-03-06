use std::mem::size_of;

use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Job {
    pub program: Pubkey,
    pub credits_mint: Pubkey,
    pub authority: Pubkey,
    // todo: unsure how about bundling ix_tag, and later checking it, technically programs are free
    // to decode incoming ix data as they see fit, it doesnt need to be first 4 bytes, though
    // that is the convention
    pub ix_tag: u32,

    pub execution_payout: u64,
    pub is_configured: bool,

    pub bump: u8,

    // TODO: padding size
    padding: [u8; 2],
}
const_assert!(size_of::<Job>() == 3 * 32 + 4 + 8 + 1 + 1 + 2);

#[macro_export]
macro_rules! job_seeds {
    ( $job:expr ) => {
        &[
            $job.authority.as_ref(),
            $job.program.as_ref(),
            &$job.ix_tag.to_le_bytes(),
            &[$job.bump],
        ]
    };
}

pub use job_seeds;
