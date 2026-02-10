//! pumpfun-sdk-idl
//!
//! - Vendors an IDL snapshot (minimal at first)
//! - Provides generated, typed instruction builders (Anchor-style)
//! - Includes helpers for discriminator computation

pub mod embedded;
pub mod idl;
pub mod util;

// Generated code lives here (populated in later commits)
pub mod generated;

pub use embedded::pump_bonding_curve_min_idl_json;
pub use idl::{Idl, IdlAccount, IdlArg, IdlInstruction};
