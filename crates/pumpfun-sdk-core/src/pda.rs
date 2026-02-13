use solana_sdk::pubkey::Pubkey;

use crate::{error::Result, ids, PumpSdkError};

/// Pump PDA: "mint-authority"
pub fn pump_mint_authority(pump_program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"mint-authority"], pump_program_id)
}

/// Pump PDA: "bonding-curve" + mint
pub fn pump_bonding_curve(pump_program_id: &Pubkey, mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"bonding-curve", mint.as_ref()], pump_program_id)
}

/// Pump PDA: "global"
pub fn pump_global(pump_program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"global"], pump_program_id)
}

/// Pump PDA: "__event_authority" (Anchor event emitter)
pub fn pump_event_authority(pump_program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"__event_authority"], pump_program_id)
}

/// Mayhem PDA: "global-params"
pub fn mayhem_global_params(mayhem_program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"global-params"], mayhem_program_id)
}

/// Mayhem PDA: "sol-vault"
pub fn mayhem_sol_vault(mayhem_program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"sol-vault"], mayhem_program_id)
}

/// Mayhem PDA: "mayhem-state" + mint
pub fn mayhem_state(mayhem_program_id: &Pubkey, mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"mayhem-state", mint.as_ref()], mayhem_program_id)
}

/// Validate that the derived PDAs match the static addresses published in docs.
/// (This helps catch wrong program ids / clusters.)
pub fn validate_mayhem_static_accounts(mayhem_program_id: &Pubkey) -> Result<()> {
    let (derived_gp, _) = mayhem_global_params(mayhem_program_id);
    if derived_gp != ids::MAYHEM_GLOBAL_PARAMS {
        return Err(PumpSdkError::MayhemStaticMismatch {
            kind: "global-params",
            expected: ids::MAYHEM_GLOBAL_PARAMS,
            derived: derived_gp,
        });
    }

    let (derived_vault, _) = mayhem_sol_vault(mayhem_program_id);
    if derived_vault != ids::MAYHEM_SOL_VAULT {
        return Err(PumpSdkError::MayhemStaticMismatch {
            kind: "sol-vault",
            expected: ids::MAYHEM_SOL_VAULT,
            derived: derived_vault,
        });
    }

    Ok(())
}
