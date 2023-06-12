use std::convert::TryInto;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult, pubkey::Pubkey, program::invoke_signed, program_error::ProgramError,
};

use crate::{instructions::HotPotatoInstruction, pda::find_hot_potato_pda};

const POTATO_SIZE: usize = 4;

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

    // only needed for rent-exempt case
    // let payer_original = **payer.try_borrow_lamports().unwrap();
    let potato_lamports = 1; // Rent::get().unwrap().minimum_balance(POTATO_SIZE) 1 0
    let create_ix = solana_program::system_instruction::create_account(
        payer.key,
        potato.key,
        potato_lamports,
        POTATO_SIZE.try_into().unwrap(),
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
    {
        let mut d = &mut **potato.try_borrow_mut_data().unwrap();
        69u32.serialize(&mut d).unwrap();
    }
    
    // comment out for non rent-exempt case
    //**potato.try_borrow_mut_lamports().unwrap() = 0;
    //**payer.try_borrow_mut_lamports().unwrap() = payer_original;
    solana_program::msg!("create: addr {}, len {}, data {:?}", potato.key, potato.try_borrow_data().unwrap().len(), potato.try_borrow_data().unwrap());
    Ok(())
}

pub fn process_consume_potato(accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let refund_to = next_account_info(account_info_iter)?;
    let potato = next_account_info(account_info_iter)?;
    {
        let d = &**potato.try_borrow_data().unwrap();
        solana_program::msg!("consume: addr {}, len {}", potato.key, d.len());
        let shld_be_69 = u32::deserialize(&mut d.as_ref()).unwrap();
        if shld_be_69 != 69 {
            return Err(ProgramError::Custom(69));
        }
        // drop d so realloc can run
    }

    potato.realloc(0, false)?;
    let refund_to_lamports = **refund_to.try_borrow_lamports().unwrap();
    let potato_lamports = **potato.try_borrow_lamports().unwrap();
    **potato.try_borrow_mut_lamports().unwrap() = 0;
    **refund_to.try_borrow_mut_lamports().unwrap() = refund_to_lamports + potato_lamports;
    Ok(())
}
