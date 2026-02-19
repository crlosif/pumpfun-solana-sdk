use sha2::{Digest, Sha256};

/// Anchor-style discriminator for global instructions:
/// `sha256("global:<name>")[0..8]`
pub fn anchor_global_discriminator(ix_name: &str) -> [u8; 8] {
    let preimage = format!("global:{ix_name}");
    let hash = Sha256::digest(preimage.as_bytes());
    hash[..8].try_into().expect("slice length is 8")
}
