# PumpFun Rust SDK

A minimal, dependency-light Rust SDK for [Pump.fun](https://pump.fun) on Solana. Build on-chain instructions, derive PDAs, and send transactions—without trading bots, snipers, or MEV logic.

## Features

- **On-chain instruction builders** — Anchor-style builders for `create_v2`, `buy`, and `sell`
- **PDA / ATA helpers** — Derive bonding curve, global, mint authority, and associated token accounts
- **Token-2022** — Full support for `create_v2` (Mayhem mode)
- **Client utilities** — Simulate, send, confirm transactions; fetch global and bonding curve state
- **Pinned IDL** — Generated types from pinned IDL snapshots for exact account ordering

## Crates

| Crate | Description |
|-------|-------------|
| `pumpfun-sdk-core` | PDAs, ATAs, instruction builders (`create_v2`, `buy`, `sell`) |
| `pumpfun-sdk-idl` | IDL snapshots + generated Anchor-style types |
| `pumpfun-sdk-client` | RPC client: simulate, send, fetch global/bonding curve |

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
pumpfun-sdk-core = "0.1"
pumpfun-sdk-client = "0.1"   # optional: for RPC/simulate/send
```

## Quick Start

### Create a Token (Token-2022)

```rust
use pumpfun_sdk_core::{Config, build_create_v2_ix, CreateV2Params};
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signature::Keypair, signer::Signer};

let cfg = Config::devnet_create_v2_defaults();  // or mainnet_create_v2_defaults()
let mint = Keypair::new();
let user = Keypair::new();

let params = CreateV2Params {
    name: "My Coin".to_string(),
    symbol: "COIN".to_string(),
    uri: "https://example.com/metadata.json".to_string(),
    is_mayhem_mode: true,
};

let (ix, accounts) = build_create_v2_ix(&cfg, params, mint.pubkey(), user.pubkey())?;

// Build transaction with [ix], signers: [&user, &mint]
```

### Buy / Sell on Bonding Curve

```rust
use pumpfun_sdk_core::{Config, build_buy_ix, build_sell_ix, ids};

let cfg = Config::mainnet_create_v2_defaults();
let mint: Pubkey = /* ... */;
let user: Pubkey = /* ... */;
let fee_recipient: Pubkey = /* fetch from Global via client.fetch_fee_recipient() */;

// Buy: amount tokens, max_sol_cost lamports
let (buy_ix, _) = build_buy_ix(
    &cfg,
    mint,
    user,
    fee_recipient,
    1_000_000,   // amount
    100_000_000, // max_sol_cost (slippage)
    &ids::TOKEN_2022_PROGRAM_ID,
)?;

// Sell: amount tokens, min_sol_output lamports
let (sell_ix, _) = build_sell_ix(
    &cfg,
    mint,
    user,
    fee_recipient,
    1_000_000,   // amount
    90_000_000,  // min_sol_output (slippage)
    &ids::TOKEN_2022_PROGRAM_ID,
)?;
```

### Client: Simulate & Send

```rust
use pumpfun_sdk_client::bonding_curve::BondingCurveClient;
use pumpfun_sdk_core::Config;

let client = BondingCurveClient::new("https://api.mainnet-beta.solana.com");
let cfg = Config::mainnet_create_v2_defaults();

// Fetch fee recipient (needed for buy/sell)
let fee_recipient = client.fetch_fee_recipient(&cfg.pump_program_id)?;

// Fetch bonding curve state (reserves, complete)
let state = client.fetch_bonding_curve(&cfg.pump_program_id, &mint)?;
println!("Complete: {}", state.complete);

// Simulate buy
let result = client.simulate_buy(&cfg, &payer, mint, amount, max_sol_cost)?;

// Send buy (real transaction)
let sig = client.send_buy(&cfg, &payer, mint, amount, max_sol_cost)?;
```

### PDA Helpers

```rust
use pumpfun_sdk_core::pda;
use pumpfun_sdk_core::ids::DEFAULT_PUMP_PROGRAM_ID;

let (bonding_curve, _) = pda::pump_bonding_curve(&DEFAULT_PUMP_PROGRAM_ID, &mint);
let (global, _) = pda::pump_global(&DEFAULT_PUMP_PROGRAM_ID);
let (mint_authority, _) = pda::pump_mint_authority(&DEFAULT_PUMP_PROGRAM_ID);
```

## Examples

```bash
# create_v2 — build and simulate (dry run); SEND=1 to submit
cargo run -p pumpfun-sdk-client --example create_v2_example

# bonding curve buy — simulate (set MINT for a real coin)
MINT=<mint_pubkey> cargo run -p pumpfun-sdk-client --example bonding_curve_buy

# pumpswap AMM buy (PumpSwap program)
cargo run -p pumpfun-sdk-client --example pumpswap_build_buy
```

## Devnet Testing

Pump recommends testing on devnet before mainnet:

```bash
RPC_URL=https://api.devnet.solana.com cargo test -p pumpfun-sdk-client --test devnet_harness -- --ignored
```

## Project Structure

```
pumpfun-sdk/
├── crates/
│   ├── pumpfun-sdk-core/      # PDAs, ATAs, instruction builders
│   ├── pumpfun-sdk-idl/       # IDL + generated types (idl/*.json)
│   └── pumpfun-sdk-client/    # RPC client (simulate, send, fetch)
└── tools/
    └── idl-gen/               # Generate Rust from IDL JSON
```

## Regenerating IDL Types

When the Pump program IDL changes:

```bash
cargo run -p idl-gen -- crates/pumpfun-sdk-idl/idl/pump_bonding_curve_trade.min.json \
  crates/pumpfun-sdk-idl/src/generated/pump_bonding_curve_trade_min.rs
```

## Supported Instructions

| Program | Instruction | Status |
|---------|-------------|--------|
| Pump Bonding Curve | `create_v2` (Token-2022, Mayhem) | ✅ |
| Pump Bonding Curve | `buy` | ✅ |
| Pump Bonding Curve | `sell` | ✅ |
| PumpSwap AMM | `buy` | ✅ (low-level) |
| PumpSwap AMM | `sell` | ✅ (low-level) |

## License

MIT OR Apache-2.0
