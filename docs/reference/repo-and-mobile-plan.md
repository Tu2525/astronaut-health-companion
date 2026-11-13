# NASA Space Apps 2026: Repo Organisation and Mobile Plan

Project: astronaut health monitoring software (Create Health Monitoring Software for Astronauts on Space Missions).
Team: 4 people working as 3 Claude instances and 1 Codex. A separate Claude instance owns the architecture.
Repo link: (add yours here)

This file combines everything discussed in the chat: the repo and agent setup, the Jetpack Compose version of the mobile plan, the Flutter version (the current choice), and the sources.

---

## 1. Stack at a glance

| Piece | Choice |
|---|---|
| Hero experience | Web UI (human body figure, hazard lens, trends, guided measurements, color palettes, interactive stuff) |
| Desktop shell | Tauri (same web UI in a webview) |
| Safe-mode console | egui (native plus WASM build), the redundancy UI |
| Doctor dashboard | Same web UI with a "flight surgeon" role |
| Mobile | Flutter (current choice). Jetpack Compose was the earlier option |
| Backend | Rust (axum), framed as "Ground Station / Earth" |
| Assessment engine | One shared Rust crate, native on desktop, WASM in the web UI, used in the backend and in mobile |
| Local AI | Optional, grounded explainer. Explains and drafts, never decides |

Core idea from the architecture doc: build the body once, compute status once. If each client calculates status on its own, the phone will show yellow while the dashboard shows green, and it will happen during the demo.

---

## 2. Repo layout

Do not make one giant CLAUDE.md. Make a short root file plus a small one per area. Claude Code loads the nested CLAUDE.md automatically when it works in that folder, so each agent only sees what it needs. Codex reads AGENTS.md instead, so the shared rules go in AGENTS.md and the root CLAUDE.md just says "read AGENTS.md".

```
repo/
  AGENTS.md            # shared rules, ownership table, how to run and test
  CLAUDE.md            # "read AGENTS.md"
  CODEOWNERS
  docs/architecture.md # from the architecture instance
  crates/
    core/              # shared types + assessment engine (the contract)
    core-ffi/          # bridge crate for mobile (UniFFI or flutter_rust_bridge)
    api/               # axum backend
  apps/
    desktop/           # tauri
    egui/              # redundancy UI, reuses crates/core
    web/               # human hero, palettes, interactive stuff
    mobile/            # Flutter (or android/ if Compose)
  .github/workflows/
```

Each folder gets its own CLAUDE.md or AGENTS.md saying what it owns, what it must not touch, and the commands to build and test it.

---

## 3. Keeping agents from stepping on each other

1. Freeze the contract first. Put the shared types in crates/core and generate TypeScript types for the web with ts-rs or specta. Whoever owns crates/core is the only one who changes it. Everyone else requests changes. This is where most hackathon merge pain comes from.
2. One owner per folder. Write it in an ownership table in AGENTS.md and in a CODEOWNERS file. Tell each agent explicitly "do not edit outside your folder".
3. One git worktree per agent, for example `git worktree add ../wt-backend backend/main`. Separate working directories mean two agents never overwrite each other's uncommitted files, which is the real danger with 3 Claudes and a Codex running at once.
4. Branches by area (`backend/...`, `desktop/...`, `web/...`, `mobile/...`), small PRs into main, rebase on main often. Protect main so nothing lands without CI passing.

---

## 4. CI/CD

Keep it minimal for a hackathon and do not spend more than 20 minutes on it.

- cargo fmt --check, cargo clippy, cargo test
- A web build
- Path filters so web changes do not trigger the whole Rust build, and so mobile only builds when it needs to
- Deploy only the web UI, and only if you need a public demo link

---

## 5. Mobile option A: Jetpack Compose (earlier plan)

Folder: `apps/android/` with its own Gradle project and CLAUDE.md. Bridge crate: `crates/core-ffi` using UniFFI.

- Wrap crates/core with UniFFI in a thin core-ffi crate and generate the Kotlin bindings. Build it with cargo-ndk.
- Do not rewrite the rules in Kotlin. The engine decides status.
- The registry and event types are the contract. Exposed through UniFFI, Kotlin gets them generated and the Android owner never edits them.
- First hour: a hello-world call from Kotlin into the engine, before any screens.

CI additions:
- Separate workflow with `paths: apps/android/**` and `crates/core*/**`
- `./gradlew assembleDebug` and lint. Skip instrumented tests
- Upload the debug APK as an artifact so anyone can sideload it

Android CLAUDE.md should say: owns apps/android only, uses the engine through generated bindings, never reimplements thresholds, Compose previews are the fast loop.

---

## 6. Mobile option B: Flutter (current choice)

Folder: `apps/mobile/` with its own CLAUDE.md. Bridge crate: `crates/core-ffi` using flutter_rust_bridge.

- Use flutter_rust_bridge instead of UniFFI. It generates the Dart types from the Rust structs, so the registry and event types still come from crates/core and the mobile owner never edits them.
- No thresholds or status logic in Dart. The engine decides.
- Use the flutter_rust_bridge project template to start. First hour: a hello-world call into the engine before any screens.

Plugins for the two jobs mobile exists for:
- Health Connect: the `health` package
- Camera pulse: the `camera` package for frames, then the signal processing. Doing the pulse math in Rust inside core-ffi is cleaner than in Dart, but only if the bridge already works. Otherwise do it in Dart and move on.

CI additions:
- Separate workflow with `paths: apps/mobile/**` and `crates/core*/**`
- `flutter analyze`, `flutter test`, `flutter build apk --debug`, upload the APK as an artifact
- Run the bridge codegen in CI and gitignore the generated files. Committed generated code is a merge conflict magnet when several agents touch the engine

Mobile CLAUDE.md should say: owns apps/mobile only, never hand-edits generated bridge files, never reimplements engine logic, target is Android only, Flutter web and desktop targets are out of scope.

---

## 7. Summary of the architecture doc (stack-and-architecture-ideas.md)

- Offline-first astronaut side. The ground backend is a feature, not a dependency.
- Indicator registry: rules as data. Each indicator has units and conversions, body region, related NASA hazards, how to measure it in space, expected in-flight change, ranges per environment, source citation and status (sourced, adapted, placeholder), actions, kit items and escalation level.
- Append-only event log: every reading, symptom, check-in, kit use, flag, ground message and override is an event with value, unit, source, confidence, environment, mission time, Earth time and author.
- Guardrails: gamification only sees activity events, never health values. The AI cannot change status. Views differ by role (astronaut, crewmates, leaderboard, flight surgeon).
- egui for safe mode, not GPUI, because the WASM build lets judges click the degraded mode. Prefer an SVG or 2D body over 3D because WebKitGTK is weak with WebGL.
- Local AI explains, drafts SBAR and structures logging. The app must work fully with the AI off.
- Mobile only earns its place for camera pulse, Health Connect and notifications. If none of these make the demo, cut mobile.
- Data honesty: simulated vitals are likely. Never call simulated data NASA data. Verify what the NASA Open Science Data Repository actually offers before writing the data section.
- The demo scenario is Mars transit, day 143, touching every module from sleep debt through the SBAR report, the delayed flight surgeon note, the egui safe-mode fallback and the leaderboard.

---

## 8. Warnings

- You are building five surfaces in 48 hours: web, Tauri, egui, backend and mobile. Get the web UI and one desktop path working end to end before polishing the second one, or you will demo two half-finished things.
- Give mobile to whoever is least needed elsewhere and scope it to camera pulse and Health Connect only. If the bridge is not working by about hour 6, drop it and show the phone part as a screen recording.
- Flutter can also build for web and desktop, so someone will be tempted to use it for the body figure or a second dashboard. Do not. Keep it strictly as the phone companion.
- Check that someone on the team actually knows Flutter and Dart. If nobody does, Compose or dropping mobile is faster than learning a new stack mid-hackathon.

---

## 9. Links and sources

Tools and docs referenced (official pages, from my own knowledge, not re-checked live in this chat):

- AGENTS.md convention: https://agents.md
- Claude Code memory and CLAUDE.md files: https://docs.claude.com/en/docs/claude-code/memory
- git worktree docs: https://git-scm.com/docs/git-worktree
- GitHub CODEOWNERS docs: https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners
- Tauri: https://tauri.app
- egui: https://github.com/emilk/egui
- ts-rs: https://github.com/Aleph-Alpha/ts-rs
- specta: https://github.com/specta-rs/specta
- UniFFI: https://github.com/mozilla/uniffi-rs
- cargo-ndk: https://github.com/bbqsrc/cargo-ndk
- flutter_rust_bridge: https://github.com/fzyzcjy/flutter_rust_bridge
- Flutter health package: https://pub.dev/packages/health
- Flutter camera package: https://pub.dev/packages/camera

Project sources:

- Uploaded file: stack-and-architecture-ideas.md (your architecture instance's notes, companion to space-apps-2026-winning-playbook.md)
- NASA Space Apps challenge page: (add the link, I was not given it)
