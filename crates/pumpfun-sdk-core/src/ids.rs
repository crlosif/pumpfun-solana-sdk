use solana_sdk::{pubkey, pubkey::Pubkey};

/// Pump.fun bonding curve program (mainnet).
/// Publicly referenced here: https://solscan.io/account/6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P
pub const DEFAULT_PUMP_PROGRAM_ID: Pubkey =
    pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");

/// Mayhem program id (from Pump public docs).
pub const DEFAULT_MAYHEM_PROGRAM_ID: Pubkey =
    pubkey!("MAyhSmzXzV1pTf7LsNkrNwkWKTo4ougAJ1PPg47MD4e");

/// Static Mayhem Global Params account (from Pump public docs).
pub const MAYHEM_GLOBAL_PARAMS: Pubkey =
    pubkey!("13ec7XdrjF3h3YcqBTFDSReRcUFwbCnJaAQspM4j6DDJ");

/// Static Mayhem SOL vault account (from Pump public docs).
pub const MAYHEM_SOL_VAULT: Pubkey =
    pubkey!("BwWK17cbHxwWBKZkUYvzxLcNQ1YVyaFezduWbtm2de6s");

/// SPL Token (legacy) program id.
pub const LEGACY_TOKEN_PROGRAM_ID: Pubkey =
    pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

/// SPL Token-2022 program id.
pub const TOKEN_2022_PROGRAM_ID: Pubkey =
    pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");
