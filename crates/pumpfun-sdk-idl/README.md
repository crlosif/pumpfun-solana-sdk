# pumpfun-sdk-idl

Pump.fun IDL snapshots and generated Anchor-style instruction builders for Rust.

## Features

- **Pinned IDL** — Minimal JSON IDL snapshots for Pump bonding curve and PumpSwap AMM
- **Generated types** — Rust modules with `Accounts`, `Args`, `build_ix()` per instruction
- **Discriminator helpers** — Anchor `global:<name>` discriminator computation
- **Embedded IDL** — `include_str!` for IDL JSON at compile time

## Installation

```toml
[dependencies]
pumpfun-sdk-idl = "0.1"
```

## IDL Coverage

| IDL | Instructions |
|-----|--------------|
| `pump_bonding_curve_min` | `create_v2` |
| `pump_bonding_curve_trade_min` | `buy`, `sell` |
| `pump_amm_min` | `buy`, `sell` |

## Usage

### Generated instruction builders

```rust
use pumpfun_sdk_idl::generated::pump_bonding_curve_min::create_v2;
use pumpfun_sdk_idl::generated::pump_bonding_curve_trade_min::{buy, sell};
use pumpfun_sdk_idl::generated::pump_amm_min::{buy as amm_buy, sell as amm_sell};
```

### Embedded IDL JSON

```rust
use pumpfun_sdk_idl::{pump_bonding_curve_min_idl_json, pump_amm_min_idl_json};

let json = pump_bonding_curve_min_idl_json();
// pump_bonding_curve_trade_min_idl_json via pumpfun_sdk_idl::embedded
```

### IDL parsing

```rust
use pumpfun_sdk_idl::{Idl, pump_bonding_curve_min_idl_json};

let idl: Idl = serde_json::from_str(pump_bonding_curve_min_idl_json())?;
```

## Regenerating types

When the Pump program IDL changes, regenerate with `idl-gen`:

```bash
cargo run -p idl-gen -- crates/pumpfun-sdk-idl/idl/<idl>.min.json \
  crates/pumpfun-sdk-idl/src/generated/<output>.rs
```

## Dependencies

- `serde`, `serde_json`, `borsh`, `sha2`, `solana-sdk`

## License

MIT OR Apache-2.0
