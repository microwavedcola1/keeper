use std::cmp::min;
use std::convert::TryInto;

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};
use spl_token::{
    solana_program::{
        account_info::next_account_info, program::invoke_signed, program_error::ProgramError,
        program_pack::Pack,
    },
    state::Account,
};

use thiserror::Error;

use crate::program_test::helpers::keeper_requiring_program::KeeperRequiringProgramError::InvalidInstruction;

pub enum KeeperRequiringProgramInstruction {
    /// Perform an update
    ///
    /// Accounts expected:
    ///
    ///   0. `[writable]` a.
    ///   1. `[writable]` b.
    ///   2. `[writable]` c.
    Update {},
}

entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    Processor::process(program_id, accounts, instruction_data)
}

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = KeeperRequiringProgramInstruction::unpack(instruction_data)?;

        match instruction {
            KeeperRequiringProgramInstruction::Update {} => {
                msg!("Instruction: Update");
                Self::process_update(program_id, accounts)
            }
        }
    }

    fn process_update(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        Ok(())
    }
}

impl KeeperRequiringProgramInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::Update {},
            _ => return Err(InvalidInstruction.into()),
        })
    }
}

#[derive(Error, Debug, Copy, Clone)]
pub enum KeeperRequiringProgramError {
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("The account is not currently owned by the program")]
    IncorrectProgramId,
}

impl From<KeeperRequiringProgramError> for ProgramError {
    fn from(e: KeeperRequiringProgramError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
