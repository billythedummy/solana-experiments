#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

pub mod instructions;
pub mod pda;
pub mod processor;

solana_program::declare_id!("2gt7fw22RH8g8Q12JrzrRvEMBNLKB4QnLwQiUXnhGPb6");
