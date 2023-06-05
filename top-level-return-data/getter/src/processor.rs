use std::convert::TryInto;

use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program::get_return_data,
    program_error::ProgramError, pubkey::Pubkey,
};

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let (setter_prog, data) = get_return_data().unwrap();
    msg!("setter: {}", setter_prog);
    msg!("len: {}", data.len());
    let raw_u32: [u8; 4] = data.try_into().unwrap();
    let shld_be_69u32: u32 = u32::from_le_bytes(raw_u32);
    msg!("data: {}", shld_be_69u32);
    if shld_be_69u32 != 69 {
        return Err(ProgramError::Custom(1));
    }
    Ok(())
}
