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
