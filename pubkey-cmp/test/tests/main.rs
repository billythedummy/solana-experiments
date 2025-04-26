use lazy_static::lazy_static;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};
use solana_program_test::{processor, ProgramTest};
use solana_sdk::{account::Account, signature::Keypair, signer::Signer, transaction::Transaction};

// use consistent keys for fair test

mod rand_pubkey {
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

const PAYER_BALANCE: u64 = 1_000_000_000;

async fn run_test(mut program_test: ProgramTest, pubkey: Pubkey) {
    program_test.add_account(
        PAYER.pubkey(),
        Account {
            lamports: PAYER_BALANCE,
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
            vec![AccountMeta {
                pubkey,
                is_signer: false,
                is_writable: false,
            }],
        )],
        Some(&PAYER.pubkey()),
    );
    tx.sign(&[&PAYER], recent_blockhash);
    let tx_res = banks_client.process_transaction(tx).await;
    match pubkey <= program::ID {
        true => {
            tx_res.unwrap_err();
        }
        false => tx_res.unwrap(),
    };
}

fn pubkey_cmp_syscall_programtest() -> ProgramTest {
    let mut program_test = ProgramTest::default();
    program_test.add_program(
        "pubkey_cmp_syscall",
        program::ID,
        processor!(pubkey_cmp_syscall::process_instruction),
    );
    program_test
}

#[tokio::test]
async fn pubkey_cmp_syscall_payer() {
    let program_test = pubkey_cmp_syscall_programtest();
    run_test(program_test, PAYER.pubkey()).await;
}

#[tokio::test]
async fn pubkey_cmp_syscall_rand() {
    let program_test = pubkey_cmp_syscall_programtest();
    run_test(program_test, rand_pubkey::ID).await;
}

#[tokio::test]
async fn pubkey_cmp_syscall_zeroes() {
    let program_test = pubkey_cmp_syscall_programtest();
    run_test(program_test, Pubkey::default()).await;
}

// Normal

fn pubkey_cmp_normal_programtest() -> ProgramTest {
    let mut program_test = ProgramTest::default();
    program_test.add_program(
        "pubkey_cmp_normal",
        program::ID,
        processor!(pubkey_cmp_normal::process_instruction),
    );
    program_test
}

#[tokio::test]
async fn pubkey_cmp_normal_payer() {
    let program_test = pubkey_cmp_normal_programtest();
    run_test(program_test, PAYER.pubkey()).await;
}

#[tokio::test]
async fn pubkey_cmp_normal_rand() {
    let program_test = pubkey_cmp_normal_programtest();
    run_test(program_test, rand_pubkey::ID).await;
}

#[tokio::test]
async fn pubkey_cmp_normal_zeroes() {
    let program_test = pubkey_cmp_normal_programtest();
    run_test(program_test, Pubkey::default()).await;
}
