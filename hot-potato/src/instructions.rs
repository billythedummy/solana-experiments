use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{instruction::{Instruction, AccountMeta}, pubkey::Pubkey, system_program};

use crate::pda::find_hot_potato_pda;

#[derive(BorshDeserialize, BorshSerialize)]
pub enum HotPotatoInstruction {
    CreatePotato,
    ConsumePotato
}

pub fn create_potato_ix(payer: &Pubkey) -> Instruction {
    Instruction::new_with_borsh(
        crate::ID,
        &HotPotatoInstruction::CreatePotato,
        vec![
            AccountMeta::new(*payer, true),
            AccountMeta::new(find_hot_potato_pda().0, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ]
    )
}

pub fn consume_potato_ix() -> Instruction {
    Instruction::new_with_borsh(
        crate::ID,
        &HotPotatoInstruction::ConsumePotato,
        vec![
            AccountMeta::new(find_hot_potato_pda().0, false),
        ]
    )
}
