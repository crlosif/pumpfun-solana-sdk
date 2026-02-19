use solana_sdk::pubkey::Pubkey;

pub type Result<T> = std::result::Result<T, PumpSdkError>;

#[derive(Debug, thiserror::Error)]
pub enum PumpSdkError {
    #[error("create_v2 requires Token-2022 token program, got: {0:?}")]
    InvalidTokenProgramForCreateV2(crate::config::TokenProgram),

    #[error("borsh serialization failed: {0}")]
    BorshSerialize(#[from] std::io::Error),

    #[error("mayhem static account mismatch for {kind}: expected {expected}, derived {derived}")]
    MayhemStaticMismatch {
        kind: &'static str,
        expected: Pubkey,
        derived: Pubkey,
    },
}
