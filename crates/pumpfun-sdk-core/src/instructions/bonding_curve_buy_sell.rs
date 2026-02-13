//! Bonding curve buy/sell instruction builders.
//!
//! Uses the pump bonding curve program (same as create_v2). Fee recipient must be
//! fetched from the Global account before building; use client `fetch_global` to obtain it.

use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    sysvar,
};
use solana_system_interface::program;

use crate::{ata, config::Config, pda, Result};

/// All accounts needed for bonding curve buy, in IDL order.
#[derive(Debug, Clone, Copy)]
pub struct BondingCurveBuyAccounts {
    pub global: Pubkey,
    pub fee_recipient: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub associated_bonding_curve: Pubkey,
    pub associated_user: Pubkey,
    pub user: Pubkey,
}

/// Build Pump bonding curve `buy` instruction.
///
/// - `fee_recipient`: Read from Global account (e.g. via client `fetch_global`).
/// - `token_program`: Use `ids::TOKEN_2022_PROGRAM_ID` for create_v2 coins, or legacy for older mints.
pub fn build_buy_ix(
    cfg: &Config,
    mint: Pubkey,
    user: Pubkey,
    fee_recipient: Pubkey,
    amount: u64,
    max_sol_cost: u64,
    token_program: &Pubkey,
) -> Result<(Instruction, BondingCurveBuyAccounts)> {
    let (bonding_curve, _) = pda::pump_bonding_curve(&cfg.pump_program_id, &mint);
    let (global, _) = pda::pump_global(&cfg.pump_program_id);
    let (event_authority, _) = pda::pump_event_authority(&cfg.pump_program_id);

    let associated_bonding_curve =
        ata::get_ata_with_token_program(&bonding_curve, &mint, token_program);
    let associated_user = ata::get_ata_with_token_program(&user, &mint, token_program);

    let a = pumpfun_sdk_idl::generated::pump_bonding_curve_trade_min::buy::Accounts {
        global,
        fee_recipient,
        mint,
        bonding_curve,
        associated_bonding_curve,
        associated_user,
        user,
        system_program: program::id(),
        token_program: *token_program,
        rent: sysvar::rent::id(),
        event_authority,
        program: cfg.pump_program_id,
    };

    let args = pumpfun_sdk_idl::generated::pump_bonding_curve_trade_min::buy::Args {
        amount,
        max_sol_cost,
    };

    let ix = pumpfun_sdk_idl::generated::pump_bonding_curve_trade_min::buy::build_ix(
        cfg.pump_program_id,
        a,
        args,
    );

    let accounts = BondingCurveBuyAccounts {
        global,
        fee_recipient,
        mint,
        bonding_curve,
        associated_bonding_curve,
        associated_user,
        user,
    };

    Ok((ix, accounts))
}

/// All accounts needed for bonding curve sell, in IDL order.
#[derive(Debug, Clone, Copy)]
pub struct BondingCurveSellAccounts {
    pub global: Pubkey,
    pub fee_recipient: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub associated_bonding_curve: Pubkey,
    pub associated_user: Pubkey,
    pub user: Pubkey,
}

/// Build Pump bonding curve `sell` instruction.
///
/// - `fee_recipient`: Read from Global account (e.g. via client `fetch_global`).
/// - `token_program`: Use `ids::TOKEN_2022_PROGRAM_ID` for create_v2 coins, or legacy for older mints.
pub fn build_sell_ix(
    cfg: &Config,
    mint: Pubkey,
    user: Pubkey,
    fee_recipient: Pubkey,
    amount: u64,
    min_sol_output: u64,
    token_program: &Pubkey,
) -> Result<(Instruction, BondingCurveSellAccounts)> {
    let (bonding_curve, _) = pda::pump_bonding_curve(&cfg.pump_program_id, &mint);
    let (global, _) = pda::pump_global(&cfg.pump_program_id);
    let (event_authority, _) = pda::pump_event_authority(&cfg.pump_program_id);

    let associated_bonding_curve =
        ata::get_ata_with_token_program(&bonding_curve, &mint, token_program);
    let associated_user = ata::get_ata_with_token_program(&user, &mint, token_program);

    let a = pumpfun_sdk_idl::generated::pump_bonding_curve_trade_min::sell::Accounts {
        global,
        fee_recipient,
        mint,
        bonding_curve,
        associated_bonding_curve,
        associated_user,
        user,
        system_program: program::id(),
        associated_token_program: spl_associated_token_account::id(),
        token_program: *token_program,
        event_authority,
        program: cfg.pump_program_id,
    };

    let args = pumpfun_sdk_idl::generated::pump_bonding_curve_trade_min::sell::Args {
        amount,
        min_sol_output,
    };

    let ix = pumpfun_sdk_idl::generated::pump_bonding_curve_trade_min::sell::build_ix(
        cfg.pump_program_id,
        a,
        args,
    );

    let accounts = BondingCurveSellAccounts {
        global,
        fee_recipient,
        mint,
        bonding_curve,
        associated_bonding_curve,
        associated_user,
        user,
    };

    Ok((ix, accounts))
}
