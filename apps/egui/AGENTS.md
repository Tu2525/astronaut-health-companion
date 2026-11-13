# SAFE: `apps/egui/` (and `apps/desktop/`)

Read the root `AGENTS.md` first. Tasks: `docs/plan.md`, section SAFE. Screens: face B in `docs/reference/ui_faces_and_workflows.md`.

**You own:** `apps/egui`, `apps/desktop`, and the build output folder `apps/web/public/safe/`.
**Not yours:** anything else in `apps/web`, anything in `crates/`.

## What you are building

The redundancy story: *"If the rich interface fails, the crew still has a native health console."*
One eframe app (`ahc-safe`), built twice:

1. **WASM**, served at `/safe/` on the same origin as the web app. This is the one judges click, so it matters most.
2. **Native** binary for the desktop story.

It is deliberately minimal: one status board, a quick-entry panel, a protocol card, a banner reading `SAFE MODE: rich interface unavailable. Same engine, same results.` Keyboard-first, high contrast. Style it on purpose; default egui looks like a debug tool.

## Rules

- Depend on `ahc-core` directly and call `ahc_core::assess`. No thresholds or status logic here.
- Storage adapter with two implementations behind `cfg(target_arch = "wasm32")`:
  - web: `localStorage` key `ahc.events.v1`, JSON `Event[]`, the same array the web app uses. Seed from `ahc_core::demo::events()` if empty. Re-read on focus so changes from the web app show up.
  - native: `events.json` in the app data dir, same shape.
- Quick entry appends a `reading` event with `device: "safe-mode"`, `confidence: "high"`, the current environment, and both clocks. Never edits or deletes.
- Status is shown as colour + icon + word, same vocabulary as the web (`docs/contract.md` section 2).
- On the demo seed your board must show exactly the statuses the web app shows. Compare before every PR.
- Getting the WASM output into `apps/web/public/safe/` needs one build step on the web side: agree it with WEB, do not edit their `package.json` yourself.
- `apps/desktop` is Tauri v2 around `apps/web/dist` and is the first thing cut if you are late. No logic in the shell.

## Commands

```bash
cargo run -p ahc-safe                                   # native
cargo install trunk && rustup target add wasm32-unknown-unknown
trunk build --release --public-url /safe/ --dist ../web/public/safe   # from apps/egui
cargo fmt --all && cargo clippy --workspace --all-targets -- -D warnings
```

Add `apps/egui` to the workspace `members` in the root `Cargo.toml` (that one line is yours to add).
