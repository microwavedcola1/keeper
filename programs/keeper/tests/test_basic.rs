use std::str::FromStr;

use anchor_spl::token::TokenAccount;
use keeper::instructions::perform_job;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use solana_program_test::*;
use solana_sdk::signer::keypair;
use solana_sdk::{signature::Keypair, signer::Signer, transport::TransportError};

use program_test::*;

mod program_test;

#[allow(unaligned_references)]
#[tokio::test]
async fn test_basic() -> Result<(), TransportError> {
    let context = TestContext::new().await;

    /// register job
    let context_argument = &context;
    let payer = &context_argument.users[0].key;
    let mint = context_argument.mints[0].pubkey.unwrap();
    let token = context_argument.users[0].token_accounts[0];
    let (job, job_bump) = Pubkey::find_program_address(
        &[&context_argument
            .keeper_requiring_program
            .program_id
            .to_bytes()],
        &context_argument.keeper.program_id,
    );
    let instructions = vec![Instruction {
        program_id: context_argument.keeper.program_id,
        accounts: anchor_lang::ToAccountMetas::to_account_metas(
            &keeper::accounts::RegisterJob {
                job,
                vault: spl_associated_token_account::get_associated_token_address(&job, &mint),
                program: context_argument.keeper_requiring_program.program_id,
                deposit_authority: payer.pubkey(),
                credits_mint: mint,
                deposit_token: token,
                system_program: solana_sdk::system_program::id(),
                token_program: spl_token::id(),
                associated_token_program: spl_associated_token_account::id(),
                rent: solana_sdk::sysvar::rent::id(),
            },
            None,
        ),
        data: anchor_lang::InstructionData::data(&keeper::instruction::RegisterJob {
            job_bump,
            amount: 100,
        }),
    }];
    context_argument
        .solana
        .process_transaction(&instructions, Some(&[payer]))
        .await
        .unwrap();

    /// perform job
    let context_argument = &context;
    let payer = &context_argument.users[0].key;
    let mint = context_argument.mints[0].pubkey.unwrap();
    let (job, _) = Pubkey::find_program_address(
        &[&context_argument
            .keeper_requiring_program
            .program_id
            .to_bytes()],
        &context_argument.keeper.program_id,
    );
    let mut cpi_data = Vec::with_capacity(1);
    cpi_data.push(0u8);
    let instructions = vec![Instruction {
        program_id: context_argument.keeper.program_id,
        accounts: anchor_lang::ToAccountMetas::to_account_metas(
            &keeper::accounts::PerformJob {
                job,
                vault: spl_associated_token_account::get_associated_token_address(&job, &mint),
                program: context_argument.keeper_requiring_program.program_id,
                credits_mint: mint,
            },
            None,
        ),
        data: anchor_lang::InstructionData::data(&keeper::instruction::PerformJob {
            job_bump,
            cpi_data,
        }),
    }];
    context_argument
        .solana
        .process_transaction(&instructions, Some(&[]))
        .await
        .unwrap();

    Ok(())
}
