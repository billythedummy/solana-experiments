use solana_program::instruction::{AccountMeta, Instruction, InstructionError};
use solana_program_test::{processor, BanksClientError, ProgramTest};
use solana_sdk::{
    account::Account,
    signer::Signer,
    transaction::{Transaction, TransactionError},
};

mod empty_account {
    solana_program::declare_id!("pwhmTGsBTjtZZCvHzmxyQHekm12UqReQc2zv4mt8bZ7");
}

mod u128_account {
    solana_program::declare_id!("kHx2zwxpSmvVLDDvaK9DTh9WQfmG1qBDN6URR3K34qS");
}

fn create_program_test() -> ProgramTest {
    let mut program_test = ProgramTest::new(
        "account_data_alignment",
        account_data_alignment::id(),
        processor!(account_data_alignment::etp),
    );
    let u128_rent_exempt = 1002240;
    program_test.add_account(
        u128_account::ID,
        Account::new_data(u128_rent_exempt, &69u128, &account_data_alignment::ID).unwrap(),
    );
    program_test
}

#[tokio::test]
async fn test_succeed() {
    let program_test = create_program_test();
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let mut tx = Transaction::new_with_payer(
        &[Instruction::new_with_bytes(
            account_data_alignment::ID,
            &[],
            vec![
                AccountMeta::new_readonly(empty_account::ID, false),
                AccountMeta::new_readonly(u128_account::ID, false),
                AccountMeta::new_readonly(u128_account::ID, false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    tx.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(tx).await.unwrap();
}

#[tokio::test]
async fn test_fail_align() {
    let program_test = create_program_test();
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    let mut tx = Transaction::new_with_payer(
        &[Instruction::new_with_bytes(
            account_data_alignment::ID,
            &[],
            vec![
                AccountMeta::new_readonly(empty_account::ID, false),
                AccountMeta::new_readonly(empty_account::ID, false),
                AccountMeta::new_readonly(u128_account::ID, false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    tx.sign(&[&payer], recent_blockhash);
    let tx_res = banks_client.process_transaction(tx).await;
    let err = tx_res.err().unwrap();
    match err {
        BanksClientError::TransactionError(e) => assert_eq!(
            e,
            TransactionError::InstructionError(0, InstructionError::Custom(69))
        ),
        _ => panic!("Unexpected error {}", err),
    }
}
