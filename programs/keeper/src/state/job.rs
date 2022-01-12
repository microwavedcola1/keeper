use std::mem::size_of;

use anchor_lang::prelude::*;

use crate::error::*;

#[account]
#[derive(Default)]
pub struct Job {
    pub program: Pubkey,
    // todo padding
}
const_assert!(size_of::<Job>() == 32);
