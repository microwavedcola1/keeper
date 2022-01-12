use std::mem::size_of;

use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct Job {
    pub program: Pubkey,
    pub credits_mint: Pubkey,
    // todo: unsure how about bundling ix_tag, and later checking it, technically programs are free
    // to decode incoming ix data as they see fit, it doesnt need to be first 4 bytes, though
    // that is the convention
    pub ix_tag: u32,
    // todo padding
}
const_assert!(size_of::<Job>() == 32 + 32 + 4);
