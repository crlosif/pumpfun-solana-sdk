//! pumpfun-sdk-idl
//!
//! - Vendors IDL snapshots
//! - Provides generated, typed instruction builders (Anchor-style)
//! - Includes helpers for discriminator computation

pub mod embedded;
pub mod idl;
pub mod util;

pub mod generated;

pub use embedded::{pump_amm_min_idl_json, pump_bonding_curve_min_idl_json};
pub use idl::{Idl, IdlAccount, IdlArg, IdlInstruction};
