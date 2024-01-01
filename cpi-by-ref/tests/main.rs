use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};
use solana_program_test::{processor, ProgramTest};
use solana_sdk::{account::Account, signer::Signer, transaction::Transaction};

#[tokio::test]
async fn test_transfer_invoke_signed_no_alloc() {
    const RECEIVER_BALANCE_PRE: u64 = 1_000_000_000;

    let program_id = Pubkey::new_unique();
    let receiver = Pubkey::new_unique();
    let mut program_test = ProgramTest::default();
    program_test.add_program(
        "cpi_by_ref",
        program_id,
        processor!(cpi_by_ref::process_instruction),
    );
    program_test.add_account(
        receiver,
        Account {
            lamports: RECEIVER_BALANCE_PRE,
            data: Vec::new(),
            owner: system_program::ID,
            executable: false,
            rent_epoch: u64::MAX,
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    let sender_balance_pre = banks_client.get_balance(payer.pubkey()).await.unwrap();
    let mut tx = Transaction::new_with_payer(
        &[Instruction::new_with_bytes(
            program_id,
            &[],
            vec![
                AccountMeta {
                    pubkey: payer.pubkey(),
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: receiver,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: system_program::ID,
                    is_signer: false,
                    is_writable: false,
                },
            ],
        )],
        Some(&payer.pubkey()),
    );
    tx.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(tx).await.unwrap();

    let sender_balance_post = banks_client.get_balance(payer.pubkey()).await.unwrap();
    let receiver_balance_post = banks_client.get_balance(receiver).await.unwrap();

    assert_eq!(receiver_balance_post, RECEIVER_BALANCE_PRE + 69);
    assert_eq!(sender_balance_post, sender_balance_pre - 69 - 5000);
}
