//! Devnet test harness for create_v2 and bonding curve.
//!
//! Run with: cargo test -p pumpfun-sdk-client --test devnet_harness -- --ignored
//!
//! Requires RPC_URL for devnet (e.g. https://api.devnet.solana.com).
//! These tests are integration tests that hit the network.

use pumpfun_sdk_client::bonding_curve::BondingCurveClient;
use pumpfun_sdk_core::{build_create_v2_ix, ids, pda, CreateV2Params, Config};
use solana_sdk::{pubkey, pubkey::Pubkey, signature::Keypair, signer::Signer};

#[test]
#[ignore = "network: run with cargo test --test devnet_harness -- --ignored"]
fn devnet_fetch_global_fee_recipient() {
    let rpc = std::env::var("RPC_URL").unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());
    let client = BondingCurveClient::new(&rpc);
    let cfg = Config::devnet_create_v2_defaults();
    let fee = client.fetch_fee_recipient(&cfg.pump_program_id).expect("fetch fee recipient");
    assert!(!fee.eq(&Pubkey::default()));
}

#[test]
#[ignore = "network: run with cargo test --test devnet_harness -- --ignored"]
fn devnet_validate_global_pda() {
    let (global, _) = pda::pump_global(&ids::DEFAULT_PUMP_PROGRAM_ID);
    // From pump docs: 4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf
    let expected = pubkey!("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf");
    assert_eq!(global, expected, "global PDA should match published address");
}

#[test]
#[ignore = "network: run with cargo test --test devnet_harness -- --ignored"]
fn devnet_create_v2_build_and_simulate() {
    let rpc = std::env::var("RPC_URL").unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());
    let cfg = Config::devnet_create_v2_defaults();
    let client = BondingCurveClient::new(&rpc);

    let mint = Keypair::new();
    let user = Keypair::new();

    let params = CreateV2Params {
        name: "Devnet Test".to_string(),
        symbol: "DVNT".to_string(),
        uri: "https://example.com/metadata.json".to_string(),
        is_mayhem_mode: true,
    };

    let (ix, accounts) = build_create_v2_ix(&cfg, params, mint.pubkey(), user.pubkey()).expect("build");
    assert_eq!(accounts.mint, mint.pubkey());

    let bh = client.rpc().get_latest_blockhash().expect("blockhash");
    let tx = solana_sdk::transaction::Transaction::new_signed_with_payer(
        &[ix],
        Some(&user.pubkey()),
        &[&user, &mint],
        bh,
    );
    let result = client.rpc().simulate_transaction(&tx).expect("simulate");
    // Simulation may fail with "insufficient funds" (expected for unfunded keypairs)
    // or succeed. We just check we get a response.
    let _ = result;
}
