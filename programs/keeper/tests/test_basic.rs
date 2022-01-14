use core::convert::TryInto;
use std::mem::transmute;
use std::str::FromStr;

use anchor_spl::token::TokenAccount;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;
use solana_program_test::*;
use solana_sdk::signer::keypair;
use solana_sdk::{signature::Keypair, signer::Signer, transport::TransportError};

use keeper::instructions::perform_job;
use program_test::*;

mod program_test;

#[allow(unaligned_references)]
#[tokio::test]
async fn test_basic() -> Result<(), TransportError> {
    let context = TestContext::new().await;

    /// register job
    let context_argument = &context;
    let authority = Keypair::new();
    let payer = &context_argument.users[0].key;
    let mint = context_argument.mints[0].pubkey.unwrap();
    let token = context_argument.users[0].token_accounts[0];
    let ix_tag: [u8; 4] = 1u32.to_le_bytes();
    let (job, job_bump) = Pubkey::find_program_address(
        &[
            &authority.pubkey().as_ref(),
            &context_argument
                .keeper_requiring_program
                .program_id
                .to_bytes(),
            &ix_tag,
        ],
        &context_argument.keeper.program_id,
    );
    let vault = spl_associated_token_account::get_associated_token_address(&job, &mint);
    let instructions = vec![
        Instruction {
            program_id: context_argument.keeper.program_id,
            accounts: anchor_lang::ToAccountMetas::to_account_metas(
                &keeper::accounts::RegisterJob {
                    job,
                    vault,
                    program: context_argument.keeper_requiring_program.program_id,
                    authority: authority.pubkey(),
                    payer: payer.pubkey(),
                    credits_mint: mint,
                    system_program: solana_sdk::system_program::id(),
                    token_program: spl_token::id(),
                    associated_token_program: spl_associated_token_account::id(),
                    rent: solana_sdk::sysvar::rent::id(),
                },
                None,
            ),
            data: anchor_lang::InstructionData::data(&keeper::instruction::RegisterJob {
                job_bump,
                ix_tag: 1,
            }),
        },
        // Fund the newly created vault with 100 tokens
        spl_token::instruction::transfer(
            &spl_token::ID,
            &token,
            &vault,
            &payer.pubkey(),
            &[&payer.pubkey()],
            100,
        )
        .unwrap(),
    ];
    context_argument
        .solana
        .process_transaction(&instructions, Some(&[payer]))
        .await
        .unwrap();

    /// perform job
    let context_argument = &context;
    let mint = context_argument.mints[0].pubkey.unwrap();
    let instructions = vec![Instruction {
        program_id: context_argument.keeper.program_id,
        accounts: anchor_lang::ToAccountMetas::to_account_metas(
            &keeper::accounts::PerformJob {
                job,
                vault,
                program: context_argument.keeper_requiring_program.program_id,
                keeper_token: token,
                credits_mint: mint,
                token_program: spl_token::id(),
            },
            None,
        ),
        data: anchor_lang::InstructionData::data(&keeper::instruction::PerformJob {
            cpi_data: ix_tag.to_vec(),
        }),
    }];
    context_argument
        .solana
        .process_transaction(&instructions, Some(&[]))
        .await
        .unwrap();

    /// withdraw remaining job credits
    let balance_before = context_argument.solana.token_account_balance(token).await;
    let instructions = vec![Instruction {
        program_id: context_argument.keeper.program_id,
        accounts: anchor_lang::ToAccountMetas::to_account_metas(
            &keeper::accounts::WithdrawJobCredit {
                job,
                vault,
                credits_mint: mint,
                authority: authority.pubkey(),
                destination: token,
                token_program: spl_token::id(),
                associated_token_program: spl_associated_token_account::id(),
            },
            None,
        ),
        data: anchor_lang::InstructionData::data(&keeper::instruction::WithdrawJobCredit {
            amount: 99,
        }),
    }];
    context_argument
        .solana
        .process_transaction(&instructions, Some(&[&authority]))
        .await
        .unwrap();
    let balance_after = context_argument.solana.token_account_balance(token).await;
    assert_eq!(balance_after, balance_before + 99);

    Ok(())
}
