use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::set_return_data, pubkey::Pubkey,
};

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let data = 69u32.to_le_bytes();
    set_return_data(&data);
    Ok(())
}
