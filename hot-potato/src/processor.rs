use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult, pubkey::Pubkey, program::invoke_signed, program_error::ProgramError,
};

use crate::{instructions::HotPotatoInstruction, pda::find_hot_potato_pda};

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = HotPotatoInstruction::try_from_slice(instruction_data)?;
    match instruction {
        HotPotatoInstruction::CreatePotato => process_create_potato(accounts),
        HotPotatoInstruction::ConsumePotato => process_consume_potato(accounts),
    }
}

pub fn process_create_potato(accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let payer = next_account_info(account_info_iter)?;
    let potato = next_account_info(account_info_iter)?;

    let create_ix = solana_program::system_instruction::create_account(
        payer.key,
        potato.key,
        0,
        4,
        &crate::id(),
    );
    let (_potato, bump) = find_hot_potato_pda();
    invoke_signed(
        &create_ix,
        &[
            payer.to_owned(),
            potato.to_owned(),
        ],
        &[&[&[bump]]],
    )?;
    let mut d = potato.try_borrow_mut_data().unwrap();
    69u32.serialize(&mut *d).unwrap();
    Ok(())
}

pub fn process_consume_potato(accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let potato = next_account_info(account_info_iter)?;
    let d = potato.try_borrow_data().unwrap();
    solana_program::msg!("addr {}, len {}", potato.key, d.len());
    let shld_be_69 = u32::deserialize(&mut d.as_ref()).unwrap();
    if shld_be_69 != 69 {
        return Err(ProgramError::Custom(69));
    }
    potato.realloc(0, false)
}
