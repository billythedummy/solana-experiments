use lazy_static::lazy_static;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    system_program,
};
use solana_program_test::{processor, ProgramTest};
use solana_sdk::{account::Account, signature::Keypair, signer::Signer, transaction::Transaction};

// use consistent keys for fair test

mod receiver {
    solana_program::declare_id!("2VfYhT9tBzirPGi8aes1eSuEveeXEo3LWF7Dcv6S2J5n");
}

mod program {
    solana_program::declare_id!("9wQMfKs85gt4A6HDyxUetV9WvbMeC3ugHf1bAGdKy8AE");
}

lazy_static! {
    static ref PAYER: Keypair = Keypair::from_bytes(&[
        56, 123, 128, 182, 44, 132, 211, 52, 46, 244, 64, 129, 202, 227, 35, 177, 182, 168, 167,
        14, 240, 38, 237, 219, 4, 202, 106, 128, 8, 254, 162, 180, 181, 189, 103, 81, 6, 173, 60,
        86, 180, 84, 211, 185, 103, 242, 95, 196, 196, 144, 157, 51, 19, 209, 124, 94, 42, 158,
        135, 37, 68, 5, 11, 99
    ])
    .unwrap();
}

const RECEIVER_BALANCE_PRE: u64 = 1_000_000_000;
const PAYER_BALANCE_PRE: u64 = 1_000_000_000;

async fn run_test(mut program_test: ProgramTest) {
    program_test.add_account(
        PAYER.pubkey(),
        Account {
            lamports: PAYER_BALANCE_PRE,
            data: Vec::new(),
            owner: system_program::ID,
            executable: false,
            rent_epoch: u64::MAX,
        },
    );
    program_test.add_account(
        receiver::ID,
        Account {
            lamports: RECEIVER_BALANCE_PRE,
            data: Vec::new(),
            owner: system_program::ID,
            executable: false,
            rent_epoch: u64::MAX,
        },
    );

    let (mut banks_client, _payer, recent_blockhash) = program_test.start().await;
    let mut tx = Transaction::new_with_payer(
        &[Instruction::new_with_bytes(
            program::ID,
            &[],
            vec![
                AccountMeta {
                    pubkey: PAYER.pubkey(),
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: receiver::ID,
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
        Some(&PAYER.pubkey()),
    );
    tx.sign(&[&PAYER], recent_blockhash);
    banks_client.process_transaction(tx).await.unwrap();

    let sender_balance_post = banks_client.get_balance(PAYER.pubkey()).await.unwrap();
    let receiver_balance_post = banks_client.get_balance(receiver::ID).await.unwrap();

    assert_eq!(receiver_balance_post, RECEIVER_BALANCE_PRE + 69);
    assert_eq!(sender_balance_post, PAYER_BALANCE_PRE - 69 - 5000);
}

#[tokio::test]
async fn cpi_by_ref() {
    let mut program_test = ProgramTest::default();
    program_test.add_program(
        "cpi_by_ref",
        program::ID,
        processor!(cpi_by_ref::process_instruction),
    );
    run_test(program_test).await;
}

#[tokio::test]
async fn cpi_normal() {
    let mut program_test = ProgramTest::default();
    program_test.add_program(
        "cpi_normal",
        program::ID,
        processor!(cpi_normal::process_instruction),
    );
    run_test(program_test).await;
}
