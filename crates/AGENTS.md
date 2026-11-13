# ENGINE: `crates/`

Read the root `AGENTS.md` first. Tasks: `docs/plan.md`, section ENGINE.

**You own:** `crates/core`, `crates/core-wasm`, `crates/api`, the generated `apps/web/src/contract/`, and `apps/web/src/faces/ground/`.
**Not yours:** `crates/core-ffi` (MOBILE), the rest of `apps/`.

You hold the contract. Everyone else is blocked when you are slow on a `contract:` issue, so those come before your own tasks.

## What is here

- `core/src/types.rs`: the contract. Every type derives `Serialize, Deserialize, TS` with `#[ts(export)]`.
- `core/registry.json`: indicators as data. Thresholds change here, not in code.
- `core/src/engine.rs`: `assess()`. Pure: no clock, no I/O, no randomness. `now` arrives in the input.
- `core/src/gamify.rs`: scoring over `Activity` (which has no value field, on purpose).
- `core/src/demo.rs`: the seeded day-143 story. The tests in `lib.rs` pin its opening state; the whole demo depends on it.
- `core-wasm/`: thin JSON bridge. No logic.
- `api/`: you create it (ENGINE-1). Spec: `docs/contract.md` section 9.

## Rules

- Times are `f64` (`met_s` seconds, `earth_ms` milliseconds). Never `u64`/`i64` in contract types: ts-rs turns them into `bigint`, which JSON cannot carry.
- Enums serialise `snake_case`. Tagged enums use `#[serde(tag = "type")]`.
- Adding an optional field is fine. Renaming or removing is breaking: PR title gets `!`, update `docs/contract.md` in the same PR, announce in chat.
- Every engine behaviour gets one test. Keep the demo-story tests green; if you must change the seed, change the tests and tell WEB and SAFE.
- No medication doses in registry actions. Every range has an honest `source.status`.
- `core` stays dependency-light (serde, serde_json, ts-rs) so it compiles to WASM and Android without surprises.
- Ground face: it lives in the web app, so follow `apps/web/AGENTS.md` conventions there and do not touch files outside `src/faces/ground/`.

## Commands

```bash
cargo test -p ahc-core            # also regenerates apps/web/src/contract/*.ts — commit them
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo build -p ahc-core-wasm --target wasm32-unknown-unknown   # needs: rustup target add wasm32-unknown-unknown
cargo run -p ahc-api              # after ENGINE-1
```
