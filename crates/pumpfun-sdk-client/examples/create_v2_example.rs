//! create_v2 example: build and optionally send a Token-2022 coin creation.
//!
//! Run on devnet (Pump recommends testing there first):
//!   cargo run -p pumpfun-sdk-client --example create_v2_example
//!
//! With dry-run (default): builds the instruction and simulates locally.
//! With SEND=1: sends and confirms the transaction (requires funded keypair).

use pumpfun_sdk_client::bonding_curve::BondingCurveClient;
use pumpfun_sdk_core::{build_create_v2_ix, CreateV2Params, Config};
use solana_sdk::{signature::{Keypair, Signer}, transaction::Transaction};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rpc_url = std::env::var("RPC_URL")
        .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());

    let cfg = Config::devnet_create_v2_defaults();
    let client = BondingCurveClient::new(&rpc_url);

    let mint = Keypair::new();
    let user = Keypair::new();

    let params = CreateV2Params {
        name: "Test Coin".to_string(),
        symbol: "TEST".to_string(),
        uri: "https://example.com/metadata.json".to_string(),
        is_mayhem_mode: true,
    };

    let (ix, accounts) = build_create_v2_ix(
        &cfg,
        params,
        mint.pubkey(),
        user.pubkey(),
    )?;

    println!("Mint: {}", mint.pubkey());
    println!("Bonding curve: {}", accounts.bonding_curve);
    println!("Global: {}", accounts.global);

    let send = std::env::var("SEND").ok().as_deref() == Some("1");
    if send {
        let bh = client.rpc().get_latest_blockhash()?;
        let signers: Vec<&dyn Signer> = vec![&user, &mint];
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&user.pubkey()),
            &signers,
            bh,
        );
        let sig = client.rpc().send_and_confirm_transaction_with_spinner_and_commitment(
            &tx,
            solana_commitment_config::CommitmentConfig::confirmed(),
        )?;
        println!("Sent: {}", sig);
    } else {
        println!("Dry run. Set SEND=1 to send. Simulating...");
        let bh = client.rpc().get_latest_blockhash()?;
        let signers: Vec<&dyn Signer> = vec![&user, &mint];
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&user.pubkey()),
            &signers,
            bh,
        );
        let result = client.rpc().simulate_transaction(&tx)?;
        println!("Simulation: {:?}", result.value);
    }

    Ok(())
}
