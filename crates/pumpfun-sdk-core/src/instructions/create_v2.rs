use solana_sdk::{instruction::Instruction, pubkey::Pubkey};
use solana_system_interface::program;

use crate::{
    ata,
    config::{Config, TokenProgram},
    ids,
    pda,
    PumpSdkError,
    Result,
};

// Re-export a nice param type for users of core
#[derive(Debug, Clone)]
pub struct CreateV2Params {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub is_mayhem_mode: bool,
}

#[derive(Debug, Clone)]
pub struct CreateV2Accounts {
    pub mint: Pubkey,
    pub mint_authority: Pubkey,
    pub bonding_curve: Pubkey,
    pub associated_bonding_curve_token_account: Pubkey,
    pub global: Pubkey,
    pub user: Pubkey,

    pub mayhem_program: Pubkey,
    pub global_params: Pubkey,
    pub sol_vault: Pubkey,
    pub mayhem_state: Pubkey,
    pub mayhem_token_vault: Pubkey,
}

/// Build Pump.fun `create_v2` instruction (Token-2022) using the generated IDL module.
///
/// - `mint` must be a *new* keypair's pubkey (the tx must be signed by that keypair)
/// - `user` must sign and typically funds rent/fees
pub fn build_create_v2_ix(
    cfg: &Config,
    params: CreateV2Params,
    mint: Pubkey,
    user: Pubkey,
) -> Result<(Instruction, CreateV2Accounts)> {
    if cfg.token_program != TokenProgram::Token2022 {
        return Err(PumpSdkError::InvalidTokenProgramForCreateV2(cfg.token_program));
    }

    // Safety: confirm published static accounts match derived PDAs for this mayhem program id.
    pda::validate_mayhem_static_accounts(&cfg.mayhem_program_id)?;

    let (mint_authority, _) = pda::pump_mint_authority(&cfg.pump_program_id);
    let (bonding_curve, _) = pda::pump_bonding_curve(&cfg.pump_program_id, &mint);
    let (global, _) = pda::pump_global(&cfg.pump_program_id);

    // Token-2022 ATA for bonding curve PDA
    let associated_bonding_curve_token_account =
        ata::get_ata_with_token_program(&bonding_curve, &mint, &ids::TOKEN_2022_PROGRAM_ID);

    // Mayhem accounts
    let mayhem_program = cfg.mayhem_program_id;
    let global_params = ids::MAYHEM_GLOBAL_PARAMS;
    let sol_vault = ids::MAYHEM_SOL_VAULT;
    let (mayhem_state, _) = pda::mayhem_state(&mayhem_program, &mint);

    // Mayhem token vault = Token-2022 ATA of Sol vault for this mint
    let mayhem_token_vault =
        ata::get_ata_with_token_program(&sol_vault, &mint, &ids::TOKEN_2022_PROGRAM_ID);

    // Use generated IDL builder (ensures account ordering matches IDL snapshot)
    let a = pumpfun_sdk_idl::generated::pump_bonding_curve_min::create_v2::Accounts {
        mint,
        mint_authority,
        bonding_curve,
        associated_bonding_curve_token_account,
        global,
        user,
        system_program: program::id(),
        token_program: ids::TOKEN_2022_PROGRAM_ID,
        associated_token_program: spl_associated_token_account::id(),
        mayhem_program,
        global_params,
        sol_vault,
        mayhem_state,
        mayhem_token_vault,
    };

    let args = pumpfun_sdk_idl::generated::pump_bonding_curve_min::create_v2::Args {
        name: params.name,
        symbol: params.symbol,
        uri: params.uri,
        is_mayhem_mode: params.is_mayhem_mode,
    };

    let ix = pumpfun_sdk_idl::generated::pump_bonding_curve_min::create_v2::build_ix(
        cfg.pump_program_id,
        a,
        args,
    );

    let accounts_struct = CreateV2Accounts {
        mint,
        mint_authority,
        bonding_curve,
        associated_bonding_curve_token_account,
        global,
        user,
        mayhem_program,
        global_params,
        sol_vault,
        mayhem_state,
        mayhem_token_vault,
    };

    Ok((ix, accounts_struct))
}
