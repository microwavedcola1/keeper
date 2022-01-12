use std::str::FromStr;

use anchor_spl::token::TokenAccount;
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

    let payer = &context.users[0].key;
    let mint = context.mints[0].pubkey.unwrap();
    let token = context.users[0].token_accounts[0];

    let (job, job_bump) = Pubkey::find_program_address(
        &[&context.keeper_requiring_program.program_id.to_bytes()],
        &context.keeper.program_id,
    );

    let instructions = vec![Instruction {
        program_id: context.keeper.program_id,
        accounts: anchor_lang::ToAccountMetas::to_account_metas(
            &keeper::accounts::RegisterJob {
                job,
                vault: spl_associated_token_account::get_associated_token_address(&job, &mint),
                program: context.keeper_requiring_program.program_id,
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
    context
        .solana
        .process_transaction(&instructions, Some(&[payer]))
        .await
        .unwrap();

    Ok(())
}
