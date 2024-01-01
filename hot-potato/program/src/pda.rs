use solana_program::pubkey::Pubkey;

pub fn find_hot_potato_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[], &crate::ID)
}
