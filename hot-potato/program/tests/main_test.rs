#![cfg(feature = "test-sbf")]

use solana_program_test::{processor, ProgramTest};
use solana_sdk::{signature::Signer, transaction::Transaction};

#[tokio::test]
async fn create_then_consume() {
    let mut program_test = ProgramTest::default();
    program_test.prefer_bpf(false);
    program_test.add_program(
        "hot-potato",
        hot_potato::id(),
        processor!(hot_potato::processor::process_instruction),
    );
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    let mut tx = Transaction::new_with_payer(
        &[
            hot_potato::instructions::create_potato_ix(&payer.pubkey()),
            hot_potato::instructions::consume_potato_ix(&payer.pubkey()),
        ],
        Some(&payer.pubkey()),
    );
    tx.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(tx).await.unwrap();
}

#[tokio::test]
async fn fail_create_without_consuming() {
    let mut program_test = ProgramTest::default();
    program_test.prefer_bpf(false);
    program_test.add_program(
        "hot_potato",
        hot_potato::id(),
        processor!(hot_potato::processor::process_instruction),
    );
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    let mut tx = Transaction::new_with_payer(
        &[hot_potato::instructions::create_potato_ix(&payer.pubkey())],
        Some(&payer.pubkey()),
    );
    tx.sign(&[&payer], recent_blockhash);
    assert!(banks_client.process_transaction(tx).await.is_err());
}
