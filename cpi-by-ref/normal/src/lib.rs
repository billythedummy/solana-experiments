use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::invoke_signed_unchecked,
    pubkey::Pubkey,
    system_program,
};

#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    invoke_signed_unchecked(
        &Instruction {
            program_id: system_program::ID,
            accounts: vec![
                AccountMeta {
                    pubkey: *accounts[0].key,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: *accounts[1].key,
                    is_signer: false,
                    is_writable: true,
                },
            ],
            data: vec![2, 0, 0, 0, 69, 0, 0, 0, 0, 0, 0, 0],
        },
        &[accounts[0].clone(), accounts[1].clone()],
        &[],
    )
}
