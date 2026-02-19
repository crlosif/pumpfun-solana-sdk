use solana_sdk::pubkey::Pubkey;

/// Associated token address using a specified token program id (legacy or token-2022).
pub fn get_ata_with_token_program(owner: &Pubkey, mint: &Pubkey, token_program_id: &Pubkey) -> Pubkey {
    spl_associated_token_account::get_associated_token_address_with_program_id(owner, mint, token_program_id)
}
