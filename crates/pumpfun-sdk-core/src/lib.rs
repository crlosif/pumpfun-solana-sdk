//! pumpfun-sdk-core
//!
//! Minimal, dependency-light building blocks for Pump.fun interaction:
//! - Program IDs/constants
//! - PDA / ATA derivations
//! - Anchor-style instruction builders (starting with `create_v2`)

pub mod ata;
pub mod config;
pub mod error;
pub mod ids;
pub mod pda;
pub mod util;

pub mod instructions;

pub use config::{Config, TokenProgram};
pub use error::{PumpSdkError, Result};
pub use instructions::bonding_curve_buy_sell::{
    build_buy_ix, build_sell_ix, BondingCurveBuyAccounts, BondingCurveSellAccounts,
};
pub use instructions::create_v2::{build_create_v2_ix, CreateV2Accounts, CreateV2Params};
