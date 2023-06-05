#![cfg(feature = "test-sbf")]

use solana_program_test::{processor, ProgramTest};
use solana_sdk::{instruction::Instruction, signature::Signer, transaction::Transaction};

#[tokio::test]
async fn main() {
    let mut program_test = ProgramTest::default();
    program_test.prefer_bpf(false);
    program_test.add_program(
        "setter",
        setter::id(),
        processor!(setter::processor::process_instruction),
    );
    program_test.add_program(
        "getter",
        getter::id(),
        processor!(getter::processor::process_instruction),
    );
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    let mut tx = Transaction::new_with_payer(
        &[
            Instruction::new_with_bytes(setter::id(), &[], vec![]),
            Instruction::new_with_bytes(getter::id(), &[], vec![]),
        ],
        Some(&payer.pubkey()),
    );
    tx.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(tx).await.unwrap();
}
