use solana_sdk::pubkey::Pubkey;

use crate::ids;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenProgram {
    Legacy,
    Token2022,
}

impl TokenProgram {
    pub fn id(self) -> Pubkey {
        match self {
            TokenProgram::Legacy => ids::LEGACY_TOKEN_PROGRAM_ID,
            TokenProgram::Token2022 => ids::TOKEN_2022_PROGRAM_ID,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub pump_program_id: Pubkey,
    pub mayhem_program_id: Pubkey,
    pub token_program: TokenProgram,
}

impl Config {
    /// Safe defaults for Pump `create_v2` (Token-2022).
    pub fn mainnet_create_v2_defaults() -> Self {
        Self {
            pump_program_id: ids::DEFAULT_PUMP_PROGRAM_ID,
            mayhem_program_id: ids::DEFAULT_MAYHEM_PROGRAM_ID,
            token_program: TokenProgram::Token2022,
        }
    }

    pub fn with_program_ids(pump_program_id: Pubkey, mayhem_program_id: Pubkey) -> Self {
        Self {
            pump_program_id,
            mayhem_program_id,
            token_program: TokenProgram::Token2022,
        }
    }
}
