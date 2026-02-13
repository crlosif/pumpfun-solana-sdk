# pumpfun-sdk-core

Minimal, dependency-light building blocks for [Pump.fun](https://pump.fun) on Solana.

## Features

- **Program IDs** — Pump, Mayhem, Token-2022 constants
- **PDA helpers** — Bonding curve, global, mint authority, event authority, Mayhem PDAs
- **ATA helpers** — Associated token accounts (legacy + Token-2022)
- **Instruction builders** — `create_v2`, `buy`, `sell` with exact account ordering

## Installation

```toml
[dependencies]
pumpfun-sdk-core = "0.1"
```

## Usage

### Config

```rust
use pumpfun_sdk_core::Config;

let cfg = Config::mainnet_create_v2_defaults();
// or Config::devnet_create_v2_defaults() for devnet
```

### Create Token (Token-2022)

```rust
use pumpfun_sdk_core::{Config, build_create_v2_ix, CreateV2Params};
use solana_sdk::pubkey::Pubkey;

let cfg = Config::mainnet_create_v2_defaults();
let (ix, accounts) = build_create_v2_ix(
    &cfg,
    CreateV2Params {
        name: "My Coin".to_string(),
        symbol: "COIN".to_string(),
        uri: "https://example.com/metadata.json".to_string(),
        is_mayhem_mode: true,
    },
    mint,
    user,
)?;
```

### Buy / Sell on Bonding Curve

```rust
use pumpfun_sdk_core::{Config, build_buy_ix, build_sell_ix, ids};

let (buy_ix, _) = build_buy_ix(
    &cfg,
    mint,
    user,
    fee_recipient,  // fetch via client.fetch_fee_recipient()
    amount,
    max_sol_cost,
    &ids::TOKEN_2022_PROGRAM_ID,
)?;

let (sell_ix, _) = build_sell_ix(
    &cfg,
    mint,
    user,
    fee_recipient,
    amount,
    min_sol_output,
    &ids::TOKEN_2022_PROGRAM_ID,
)?;
```

### PDA Helpers

```rust
use pumpfun_sdk_core::pda;
use pumpfun_sdk_core::ids::DEFAULT_PUMP_PROGRAM_ID;

let (bonding_curve, _) = pda::pump_bonding_curve(&DEFAULT_PUMP_PROGRAM_ID, &mint);
let (global, _) = pda::pump_global(&DEFAULT_PUMP_PROGRAM_ID);
let (mint_authority, _) = pda::pump_mint_authority(&DEFAULT_PUMP_PROGRAM_ID);
let (event_authority, _) = pda::pump_event_authority(&DEFAULT_PUMP_PROGRAM_ID);
```

## Dependencies

- `pumpfun-sdk-idl` — Generated instruction types
- `solana-sdk`, `solana-system-interface`, `spl-associated-token-account`
- `borsh`, `sha2`, `thiserror`

## License

MIT OR Apache-2.0
