# ZeroG Pulse (Astronaut Health Companion)

NASA Space Apps 2026 challenge: *Create Health Monitoring Software for Astronauts on Space Missions*.

**Live demo:** https://ahc-37u.pages.dev

A self-assessment tool for crews on long missions. The astronaut taps a body figure, takes a guided measurement, and sees at a glance what is fine and what is not. When something is off, an escalation ladder says what to do and how far Earth is, and a delayed link carries the report to the flight surgeon. A native safe-mode console keeps working when the rich interface does not.

One Rust engine decides health status for every face, so the web app, the safe-mode console and the Ground Station can never disagree.

> **Honesty note.** All crew readings are **simulated**. They are not NASA data and this is not a medical device. Every range in the app carries a source status: `sourced`, `adapted` or `placeholder`. Currently two ranges (sleep, radiation dose) are adapted from NASA figures and the rest are placeholders; blood pressure is a placeholder because no NASA document we found gives an in-flight range. The app contains no AI and gives no medication doses.

## What it does

| Brief asks for | Where |
|---|---|
| Visual tool, clickable human figure with function and status | Body view (Crew App) |
| Indicators: blood pressure, heart rate, temperature, sleep, mood, radiation dose and more | `crates/core/registry.json` |
| Guided measurement, "why does this matter in space?" | Measure and region screens |
| When to call the crew or flight surgeon | Four-step escalation ladder, delayed ground loop |
| Colour coding at a glance | Status is always colour + icon + word |
| Gamification, trends, prompts | Crew, Trends, Today screens |
| Works when things break | Safe-Mode Console, offline-first |

The full mapping of the challenge text to features is in [`docs/challenge.md`](docs/challenge.md); NASA sources are listed in [`docs/sources.md`](docs/sources.md).

## Architecture

```
                crates/core  (types + registry.json + assess() + demo seed)
                     │  the ONLY place status is decided
     ┌───────────────┼────────────────┐
 core-wasm        (direct)         (direct)
     │               │                │
 apps/web        apps/egui        crates/api  <── HTTP ──  apps/mobile
 Crew App        Safe-Mode        Ground Station           React Native
 (React, SVG)    console          (axum)                   companion
```

Three laws (details in [`AGENTS.md`](AGENTS.md)):

1. Status is computed once, in `crates/core`. No thresholds in TypeScript or egui code.
2. Each owner stays in their own folder.
3. Simulated data is never called NASA data.

## Repository layout

| Path | What |
|---|---|
| `crates/core` | Shared types, indicator registry, assessment engine, demo seed |
| `crates/core-wasm` | JSON bridge so the browser can call the engine |
| `crates/api` | Ground Station backend (axum) |
| `apps/web` | Crew App and Ground Station faces (React, Vite, TypeScript) |
| `apps/egui` | Safe-Mode Console, native and WASM |
| `apps/desktop` | Tauri shell around the web build |
| `apps/mobile` | React Native companion |
| `deploy/` | Cloudflare Pages and API service files |
| `pitch/` | Brag video, PDFs, deck |
| `design/` | Colour tokens and UI references |
| `docs/` | Architecture, contract, plan, challenge summary, sources |

## Run it

Requirements: Rust (stable), Node 20+, and `wasm-pack` for the web build.

```bash
cargo test --workspace
```

```bash
cd apps/web
npm install
npm run engine   # builds the Rust engine to WASM
npm run dev      # http://localhost:5173
```

```bash
cargo run -p ahc-safe   # native Safe-Mode Console
```

Everything runs with the backend and any AI **off**; state lives in the browser's `localStorage` as an append-only event log.

## Team

Roles and owners are in [`TEAM.md`](TEAM.md). Contribution rules (branches, Conventional Commits, PR template, screenshots) are in [`AGENTS.md`](AGENTS.md).

## AI use

Built with AI coding assistants. Manus AI generated the intro/outro backgrounds and music of the brag video (watermarked and noted in the file metadata); every product shot is a real capture of the running app.

## Licence

MIT.
