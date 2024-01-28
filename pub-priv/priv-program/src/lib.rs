use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::set_return_data,
    program_error::ProgramError, pubkey::Pubkey,
};

#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(etp);

fn f(data: &[u8]) -> Result<u64, ProgramError> {
    let d: &[u8; 8] = data
        .try_into()
        .map_err(|_e| ProgramError::InvalidInstructionData)?;
    let arg = u64::from_le_bytes(*d);
    let mut m = 69;
    let mut n = arg;
    while m != 0 {
        if m < n {
            core::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    Ok(n)
}

fn etp(_program_id: &Pubkey, _accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let ret = f(data)?;
    set_return_data(&ret.to_le_bytes());
    Ok(())
}
