//! Bonding curve buy example: simulate buying tokens from a pump.fun bonding curve.
//!
//! Run: cargo run -p pumpfun-sdk-client --example bonding_curve_buy
//!
//! Replace MINT with a real bonding curve mint address.

use pumpfun_sdk_client::bonding_curve::BondingCurveClient;
use pumpfun_sdk_core::Config;
use solana_sdk::signature::Keypair;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rpc_url = std::env::var("RPC_URL")
        .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());
    let mint_str = std::env::var("MINT").unwrap_or_else(|_| {
        eprintln!("Set MINT to a bonding curve mint address. Using dummy for dry run.");
        "11111111111111111111111111111111".to_string()
    });

    let mint = mint_str.parse()?;
    let cfg = Config::devnet_create_v2_defaults();
    let client = BondingCurveClient::new(&rpc_url);

    let payer = Keypair::new();
    let amount = 1_000_000; // tokens (6 decimals typical)
    let max_sol_cost = 1_000_000; // lamports

    println!("Simulating buy of {} tokens (max {} lamports)...", amount, max_sol_cost);
    let result = client.simulate_buy(&cfg, &payer, mint, amount, max_sol_cost)?;
    println!("Simulation result: {:?}", result.err);
    Ok(())
}
