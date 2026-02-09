use borsh::BorshSerialize;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use solana_system_interface::program;

use crate::{
    ata,
    config::{Config, TokenProgram},
    ids,
    pda,
    util,
    PumpSdkError,
    Result,
};

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

#[derive(BorshSerialize)]
struct CreateV2IxData {
    name: String,
    symbol: String,
    uri: String,
    is_mayhem_mode: bool,
}

/// Build Pump.fun `create_v2` instruction (Token-2022).
///
/// Notes:
/// - `mint` must be a *new* keypair's pubkey (the tx must be signed by that keypair)
/// - `user` must sign and typically funds rent/fees
/// - This builder always includes the Mayhem-related accounts as listed in the docs.
pub fn build_create_v2_ix(cfg: &Config, params: CreateV2Params, mint: Pubkey, user: Pubkey) -> Result<(Instruction, CreateV2Accounts)> {
    if cfg.token_program != TokenProgram::Token2022 {
        return Err(PumpSdkError::InvalidTokenProgramForCreateV2(cfg.token_program));
    }

    // Optional safety check: ensure static mayhem accounts match derived PDAs.
    // If you intentionally use a different cluster/program, you can skip calling this.
    pda::validate_mayhem_static_accounts(&cfg.mayhem_program_id)?;

    let (mint_authority, _) = pda::pump_mint_authority(&cfg.pump_program_id);
    let (bonding_curve, _) = pda::pump_bonding_curve(&cfg.pump_program_id, &mint);
    let (global, _) = pda::pump_global(&cfg.pump_program_id);

    // Token-2022 ATAs (per docs)
    let associated_bonding_curve_token_account =
        ata::get_ata_with_token_program(&bonding_curve, &mint, &ids::TOKEN_2022_PROGRAM_ID);

    // Mayhem PDAs / vaults (per docs)
    let mayhem_program = cfg.mayhem_program_id;
    let global_params = ids::MAYHEM_GLOBAL_PARAMS;
    let sol_vault = ids::MAYHEM_SOL_VAULT;
    let (mayhem_state, _) = pda::mayhem_state(&mayhem_program, &mint);

    // "Mayhem Token Vault": Token-2022 ATA of sol_vault for this mint
    let mayhem_token_vault =
        ata::get_ata_with_token_program(&sol_vault, &mint, &ids::TOKEN_2022_PROGRAM_ID);

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

    // Account metas follow the index order from Pump docs for `create_v2`.
    let metas = vec![
        // 1. Mint (signer, writable)
        AccountMeta::new(mint, true),
        // 2. Mint Authority PDA (readonly)
        AccountMeta::new_readonly(mint_authority, false),
        // 3. Bonding Curve PDA (writable)
        AccountMeta::new(bonding_curve, false),
        // 4. Associated Bonding Curve Token Account (Token-2022 ATA) (writable)
        AccountMeta::new(associated_bonding_curve_token_account, false),
        // 5. Global PDA (writable to be safe)
        AccountMeta::new(global, false),
        // 6. User (signer, writable)
        AccountMeta::new(user, true),
        // 7. System program
        AccountMeta::new_readonly(program::id(), false),
        // 8. Token program (Token-2022)
        AccountMeta::new_readonly(ids::TOKEN_2022_PROGRAM_ID, false),
        // 9. Associated token program
        AccountMeta::new_readonly(spl_associated_token_account::id(), false),
        // 10. Mayhem program id
        AccountMeta::new_readonly(mayhem_program, false),
        // 11. Global Params (readonly)
        AccountMeta::new_readonly(global_params, false),
        // 12. SOL vault (writable - may receive lamports)
        AccountMeta::new(sol_vault, false),
        // 13. Mayhem State (writable - may be created/updated)
        AccountMeta::new(mayhem_state, false),
        // 14. Mayhem Token Vault (writable - may be created/updated)
        AccountMeta::new(mayhem_token_vault, false),
    ];

    // Instruction data = 8-byte discriminator + borsh args
    let disc = util::anchor_global_discriminator("create_v2");
    let ix_data = CreateV2IxData {
        name: params.name,
        symbol: params.symbol,
        uri: params.uri,
        is_mayhem_mode: params.is_mayhem_mode,
    };

    let mut data = Vec::with_capacity(8 + 128);
    data.extend_from_slice(&disc);
    data.extend_from_slice(&borsh::to_vec(&ix_data)?);

    let ix = Instruction {
        program_id: cfg.pump_program_id,
        accounts: metas,
        data,
    };

    Ok((ix, accounts_struct))
}
