//! Program entrypoint

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

use crate::processor::process_instruction;

entrypoint!(etp);
fn etp(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    process_instruction(program_id, accounts, instruction_data)
}
