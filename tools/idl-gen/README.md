# idl-gen

Rust-only IDL → code generator for `pumpfun-sdk-idl`. Generates Anchor-style instruction builders from JSON IDL files.

## Usage

```bash
cargo run -p idl-gen -- <idl_json_path> <out_rs_path>
```

## Example

```bash
cargo run -p idl-gen -- \
  crates/pumpfun-sdk-idl/idl/pump_bonding_curve_trade.min.json \
  crates/pumpfun-sdk-idl/src/generated/pump_bonding_curve_trade_min.rs
```

## Output

For each instruction in the IDL, generates:

- `DISCRIMINATOR` — Anchor `global:<name>` 8-byte discriminator
- `Args` — Borsh-serializable args struct
- `Accounts` — Pubkey accounts struct
- `build_ix(program_id, accounts, args)` — Returns `Instruction`

## Dependencies

- `pumpfun-sdk-idl` — For `Idl` parsing and `anchor_global_discriminator`
- `anyhow`, `serde_json`, `sha2`

## Note

This tool is for local development and is **not published** to crates.io.
