use std::cmp::Ordering;

use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(process_instruction);

#[derive(PartialOrd, Eq, PartialEq)]
struct SyscallCmpPubkey<'a>(pub &'a Pubkey);

impl<'a> Ord for SyscallCmpPubkey<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        #[cfg(target_os = "solana")]
        {
            let mut result: i32 = 0;
            let r: *mut i32 = &mut result;
            let s1 = self.0.as_ref().as_ptr();
            let s2 = other.0.as_ref().as_ptr();
            unsafe {
                solana_program::syscalls::sol_memcmp_(s1, s2, 32, r);
            }
            if result < 0 {
                Ordering::Less
            } else if result > 0 {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
        #[cfg(not(target_os = "solana"))]
        {
            self.0.cmp(other.0)
        }
    }
}

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    if SyscallCmpPubkey(accounts[0].key) <= SyscallCmpPubkey(&Pubkey::default()) {
        return Err(ProgramError::InvalidArgument);
    }
    Ok(())
}
