use bytemuck::{try_from_bytes, Pod, Zeroable};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

solana_program::declare_id!("2gt7fw22RH8g8Q12JrzrRvEMBNLKB4QnLwQiUXnhGPb6");

solana_program::entrypoint!(etp);

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Align16 {
    pub val: u128,
}

pub fn etp(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let u128_account = &accounts[2];
    let data = u128_account.try_borrow_data()?;
    let val: &Align16 = try_from_bytes(&data).map_err(|e| {
        msg!("bytemuck try_from_bytes failed: {:?}", e);
        ProgramError::Custom(69)
    })?;
    if val.val != 69 {
        msg!("Expected 69, Got: {}", val.val);
        return Err(ProgramError::Custom(70));
    }
    Ok(())
}
