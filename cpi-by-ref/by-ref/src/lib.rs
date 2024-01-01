use std::marker::PhantomData;

use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, instruction::AccountMeta, pubkey::Pubkey,
    system_program,
};

// Adapted from:
// https://github.com/solana-labs/solana/blob/26d058185e4f9ed2982f1fb2527b146fdd9e3bed/sdk/program/src/stable_layout/stable_vec.rs#L25C1-L31C2
#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct StableVec<T> {
    pub ptr: *const T, // instead of NonNull, NonNull is repr(transparent)
    pub cap: usize,
    pub len: usize,
    _marker: PhantomData<T>,
}

impl<T> From<&[T]> for StableVec<T> {
    fn from(value: &[T]) -> Self {
        Self {
            ptr: value.as_ptr(),
            cap: value.len(),
            len: value.len(),
            _marker: PhantomData,
        }
    }
}

// Copied from:
// https://github.com/solana-labs/solana/blob/26d058185e4f9ed2982f1fb2527b146fdd9e3bed/sdk/program/src/stable_layout/stable_instruction.rs#L31-L37
#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct StableInstruction {
    pub accounts: StableVec<AccountMeta>,
    pub data: StableVec<u8>,
    pub program_id: Pubkey,
}

#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let metas = [
        AccountMeta {
            pubkey: *accounts[0].key,
            is_signer: true,
            is_writable: true,
        },
        AccountMeta {
            pubkey: *accounts[1].key,
            is_signer: false,
            is_writable: true,
        },
    ];
    let trf_accounts = [accounts[0].clone(), accounts[1].clone()];
    // transfer 69 lamports bincode-encoded
    let ix = [2, 0, 0, 0, 69, 0, 0, 0, 0, 0, 0, 0];

    no_alloc_invoke_signed_unchecked(
        &StableInstruction {
            accounts: metas.as_ref().into(),
            data: ix.as_ref().into(),
            program_id: system_program::ID,
        },
        &trf_accounts,
        &[],
    )
}

// Adapted from:
// https://github.com/solana-labs/solana/blob/26d058185e4f9ed2982f1fb2527b146fdd9e3bed/sdk/program/src/program.rs#L289
fn no_alloc_invoke_signed_unchecked(
    ix: &StableInstruction,
    account_infos: &[AccountInfo],
    signers_seeds: &[&[&[u8]]],
) -> ProgramResult {
    #[cfg(target_os = "solana")]
    {
        let result = unsafe {
            solana_program::syscalls::sol_invoke_signed_rust(
                ix as *const _ as *const u8,
                account_infos as *const _ as *const u8,
                account_infos.len() as u64,
                signers_seeds as *const _ as *const u8,
                signers_seeds.len() as u64,
            )
        };
        match result {
            solana_program::entrypoint::SUCCESS => Ok(()),
            _ => Err(result.into()),
        }
    }

    #[cfg(not(target_os = "solana"))]
    {
        // silence clippy
        let _ = ix;
        let _ = account_infos;
        let _ = signers_seeds;
        solana_program::log::sol_log("SyscallStubs: sol_invoke_signed() not available");
        Ok(())
    }
}
