# Astronaut Health Companion

NASA Space Apps 2026: *Create Health Monitoring Software for Astronauts on Space Missions*.

A self-assessment tool for crews on long missions: a clickable body figure, guided measurements, an escalation ladder that knows how far Earth is, a delayed link to the flight surgeon, and a native safe-mode console that keeps working when the rich interface does not. One Rust engine decides health status for every face.

> Health data in this project is **simulated**. It is not NASA data and this is not a medical device.

## Start here

| You are | Read |
|---|---|
| Any agent or teammate | [`AGENTS.md`](AGENTS.md), then the `AGENTS.md` in your folder |
| Asking "whose job is this?" | [`TEAM.md`](TEAM.md) |
| Building UI | [`design/README.md`](design/README.md) (references + colour code) |
| Looking for your tasks | [`docs/plan.md`](docs/plan.md) |
| Wiring to the engine or API | [`docs/contract.md`](docs/contract.md) |
| Wondering why | [`docs/architecture.md`](docs/architecture.md), [`docs/challenge.md`](docs/challenge.md) |
| Stuck on a decision | [`QUESTIONS.md`](QUESTIONS.md) |

## Layout

```
crates/core        shared types, indicator registry, assessment engine, demo seed
crates/core-wasm   JSON bridge for the browser
crates/api         "Ground Station" backend (axum)            [to build]
apps/web           Crew App + Ground Station (React)           [to build]
apps/egui          Safe-Mode Console, native + WASM            [to build]
apps/desktop       Tauri shell                                 [to build]
pitch              deck, PDFs, brag video, travel case         [to build]
apps/mobile        React Native companion                      [to build]
```

## Quick check

```bash
cargo test --workspace
```
