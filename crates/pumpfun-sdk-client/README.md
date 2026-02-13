# pumpfun-sdk-client

Off-chain RPC client helpers for [Pump.fun](https://pump.fun): simulate, send, confirm transactions, and fetch on-chain account data.

## Features

- **BondingCurveClient** — Fetch global fee recipient, bonding curve state; simulate/send buy and sell
- **PumpswapClient** — Simulate/send PumpSwap AMM buy and sell
- **No unsafe code** — `#![forbid(unsafe_code)]`

## Installation

```toml
[dependencies]
pumpfun-sdk-client = "0.1"
```

Requires `pumpfun-sdk-core` for instruction building (included as dependency).

## Usage

### BondingCurveClient

```rust
use pumpfun_sdk_client::bonding_curve::{BondingCurveClient, BondingCurveState};
use pumpfun_sdk_core::Config;

let client = BondingCurveClient::new("https://api.mainnet-beta.solana.com");
let cfg = Config::mainnet_create_v2_defaults();

// Fetch fee recipient (needed for buy/sell)
let fee_recipient = client.fetch_fee_recipient(&cfg.pump_program_id)?;

// Fetch bonding curve state
let state: BondingCurveState = client.fetch_bonding_curve(&cfg.pump_program_id, &mint)?;
println!("Complete: {}", state.complete);

// Simulate buy
let result = client.simulate_buy(&cfg, &payer, mint, amount, max_sol_cost)?;

// Send buy (real transaction)
let sig = client.send_buy(&cfg, &payer, mint, amount, max_sol_cost)?;
```

### PumpswapClient

```rust
use pumpfun_sdk_client::pumpswap::PumpswapClient;
use pumpfun_sdk_idl::generated::pump_amm_min;

let client = PumpswapClient::new("https://api.mainnet-beta.solana.com");

// Build accounts and args, then simulate or send
let result = client.simulate_buy(&payer, pumpswap_program_id, accounts, args)?;
let sig = client.send_buy(&payer, pumpswap_program_id, accounts, args)?;
```

## Examples

```bash
cargo run -p pumpfun-sdk-client --example create_v2_example
cargo run -p pumpfun-sdk-client --example bonding_curve_buy
cargo run -p pumpfun-sdk-client --example pumpswap_build_buy
```

## Dependencies

- `pumpfun-sdk-core`, `pumpfun-sdk-idl`
- `solana-client`, `solana-commitment-config`, `solana-sdk`
- `thiserror`

## License

MIT OR Apache-2.0
