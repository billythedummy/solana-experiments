use solana_program_test::{processor, ProgramTest};
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};

/// invokes prog_b with payer set to immutable
mod prog_a {
    use solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        instruction::{AccountMeta, Instruction},
        msg,
        program::invoke,
        pubkey::Pubkey,
        system_program,
    };

    solana_program::declare_id!("22222222222222222222222222222222222222222222");

    pub fn entrypoint(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        _program_data: &[u8],
    ) -> ProgramResult {
        let mut accounts_iter = accounts.iter();

        let payer = next_account_info(&mut accounts_iter)?;
        let dst = next_account_info(&mut accounts_iter)?;

        msg!("payer: {}", payer.key);
        msg!("dst: {}", dst.key);

        invoke(
            &crate::prog_b::instruction_payer_immut(payer.key, dst.key),
            accounts,
        )
    }

    pub fn instruction_payer_mut(payer: &Pubkey, dst: &Pubkey) -> Instruction {
        Instruction::new_with_bytes(
            self::id(),
            &[],
            vec![
                AccountMeta::new(*payer, true),
                AccountMeta::new(*dst, true),
                AccountMeta::new_readonly(crate::prog_b::id(), false),
                AccountMeta::new_readonly(crate::prog_c::id(), false),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
        )
    }
}

/// invokes prog_c with payer set to mutable
mod prog_b {
    use solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        instruction::{AccountMeta, Instruction},
        program::invoke,
        pubkey::Pubkey,
        system_program,
    };

    solana_program::declare_id!("33333333333333333333333333333333333333333333");

    pub fn entrypoint(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        _program_data: &[u8],
    ) -> ProgramResult {
        let mut accounts_iter = accounts.iter();

        let payer = next_account_info(&mut accounts_iter)?;
        let dst = next_account_info(&mut accounts_iter)?;

        invoke(
            &crate::prog_c::instruction_payer_mut(payer.key, dst.key),
            accounts,
        )
    }

    pub fn instruction_payer_immut(payer: &Pubkey, dst: &Pubkey) -> Instruction {
        Instruction::new_with_bytes(
            self::id(),
            &[],
            vec![
                AccountMeta::new_readonly(*payer, true),
                AccountMeta::new(*dst, true),
                AccountMeta::new_readonly(crate::prog_c::id(), false),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
        )
    }
}

/// mutates payer by creating dst account from payer
mod prog_c {
    use solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        instruction::{AccountMeta, Instruction},
        program::invoke,
        pubkey::Pubkey,
        rent::Rent,
        system_program,
        sysvar::Sysvar,
    };

    solana_program::declare_id!("44444444444444444444444444444444444444444444");

    pub fn entrypoint(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        _program_data: &[u8],
    ) -> ProgramResult {
        let mut accounts_iter = accounts.iter();

        let payer = next_account_info(&mut accounts_iter)?;
        let dst = next_account_info(&mut accounts_iter)?;

        let lamports = Rent::get().unwrap().minimum_balance(0);
        invoke(
            &solana_program::system_instruction::create_account(
                payer.key,
                dst.key,
                lamports,
                0,
                &self::id(),
            ),
            &[payer.to_owned(), dst.to_owned()],
        )
    }

    pub fn instruction_payer_mut(payer: &Pubkey, dst: &Pubkey) -> Instruction {
        Instruction::new_with_bytes(
            self::id(),
            &[],
            vec![
                AccountMeta::new(*payer, true),
                AccountMeta::new(*dst, true),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
        )
    }
}

#[tokio::test]
async fn main() {
    let mut program_test = ProgramTest::default();
    program_test.prefer_bpf(false);
    program_test.add_program("prog-a", prog_a::id(), processor!(prog_a::entrypoint));
    program_test.add_program("prog-b", prog_b::id(), processor!(prog_b::entrypoint));
    program_test.add_program("prog-c", prog_c::id(), processor!(prog_c::entrypoint));
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    let dst = Keypair::new();
    let mut tx = Transaction::new_with_payer(
        &[prog_a::instruction_payer_mut(
            &payer.pubkey(),
            &dst.pubkey(),
        )],
        Some(&payer.pubkey()),
    );
    tx.sign(&[&payer, &dst], recent_blockhash);
    banks_client.process_transaction(tx).await.unwrap_err();
}
