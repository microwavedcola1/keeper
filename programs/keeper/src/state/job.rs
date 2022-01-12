use std::mem::size_of;

use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Job {
    pub program: Pubkey,
    pub credits_mint: Pubkey,
    pub ix_tag: u32,
    // todo padding
}
const_assert!(size_of::<Job>() == 32 + 32 + 4);
