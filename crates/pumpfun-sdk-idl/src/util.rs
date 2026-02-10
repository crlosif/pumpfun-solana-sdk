use sha2::{Digest, Sha256};

/// Anchor global discriminator:
/// first 8 bytes of sha256("global:<ix_name>")
pub fn anchor_global_discriminator(ix_name: &str) -> [u8; 8] {
    let preimage = format!("global:{ix_name}");
    let hash = Sha256::digest(preimage.as_bytes());
    hash[..8].try_into().expect("slice length is 8")
}
