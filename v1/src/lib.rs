#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

pub mod errors;
pub mod instructions;
pub mod states;
pub mod utils;

pinocchio_pubkey::declare_id!("F3djNpWTDPFvum35roNrrH1u7PtXCioD9N6KApWcgVi3");