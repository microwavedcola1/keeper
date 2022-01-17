use anchor_lang::prelude::*;

#[error]
pub enum ErrorCode {
    #[msg("Invalid instruction tag")]
    InvalidIxTag,

    #[msg("Job not configured")]
    JobNotConfigured,
}
