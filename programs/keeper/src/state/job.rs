use std::mem::size_of;

use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Job {
    pub program: Pubkey,
    pub ix_tag: u32,
}
const_assert!(size_of::<Job>() == 32 + 4);
