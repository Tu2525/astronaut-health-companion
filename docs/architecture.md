# Architecture

Decisions only. Shapes and endpoints are in `contract.md`; tasks are in `plan.md`. Background reasoning is in `reference/`.

## 1. One picture

```
                     crates/core  (types + registry.json + assess() + gamify + demo seed)
                          │ the ONLY place status is decided
      ┌───────────────┬───┴────────────┐
  core-wasm        (direct)         (direct)
      │               │                │
  apps/web        apps/egui        crates/api  <── HTTP ──  apps/mobile
  Crew App        Safe-Mode        "Ground Station"         React Native companion
  + Ground face   native + WASM    axum, holds reports      camera pulse,
  (React, SVG)    at /safe/        for the comms delay,     Health Connect
      │                            runs assess for mobile
  apps/desktop (Tauri shell around apps/web's build)
```

## 2. Faces and who builds them

| Face | Where | Owner | Demo-critical screens |
|---|---|---|---|
| A. Crew App (hero) | `apps/web/src/faces/crew` | WEB | A1 Body, A2 Region, A5 Measure, A7 Guidance, A10 Crew, A12 Ground |
| B. Safe-Mode Console | `apps/egui` | SAFE | B1 Status board, B2 Quick entry |
| C. Ground Station | `apps/web/src/faces/ground` + `crates/api` | ENGINE | C2 Inbox, C4 Report, C5 Compose note |
| D. Mobile Companion | `apps/mobile` | MOBILE | The crew website in a WebView shell, plus D2 Camera pulse, D3 Wearable sync |
| Desktop shell | `apps/desktop` | SAFE | wraps the web build, adds "Enter safe mode" |

Screen specs: `reference/ui_faces_and_workflows.md`. Earth Mode (face E) is a settings profile, built only if time remains.

## 3. Decisions

**Web stack.** React + Vite + TypeScript, plain CSS variables for the theme, static build, no SSR. Router: hash routes (`#/crew/body`, `#/ground/inbox`) so it works from `file://`, Cloudflare Pages and Tauri alike. Charts: hand-rolled SVG or one small chart lib, WEB's call. Body figure: **inline SVG, 2D** (WebKitGTK in Tauri on Linux is weak at WebGL).

**Engine in the browser.** `wasm-pack build crates/core-wasm --target web --out-dir ../../apps/web/src/engine-pkg` (script `npm run engine`). Exactly one file, `apps/web/src/engine.ts`, imports the WASM and exposes typed functions. Components never import the WASM directly.

**State.** The event log is the only state that matters: `Event[]` in `localStorage` (`ahc.events.v1`). Every screen derives what it shows from `assess(log)`. No Redux; one small store (a React context or zustand) holding `events`, `settings` and the derived `assessment`. Writing = append an event, re-run `assess`.
`ponytail:` localStorage holds about 5 MB; fine for a demo log of a few thousand events. Move to IndexedDB only if it fills.

**Clock.** `now_met_s = demo.now_met_s + seconds since first load + settings.met_offset_s`. A hidden demo control adds to `met_offset_s` ("skip 2 h") so rechecks and delays can be shown live. `earth_ms = launch_earth_ms + met_s * 1000`. Never use `Date.now()` for health logic.

**Comms delay.** One number, `one_way_delay_s` (demo default 20 s, labelled as compressed time). Crew to ground: the report is POSTed immediately with `deliver_at_earth_ms`; the API hides it from ground until then. Ground to crew: the API hides the note from the crew's poll until its delivery time. The crew app polls `/api/notes` every few seconds; if the API is unreachable the outbox simply stays queued and the top bar shows "blackout". Nothing on the crew side waits on the network.

**Safe mode.** `apps/egui` is one eframe app, built twice: native binary, and WASM served at `/safe/` on the same origin as the web app. The WASM build reads and writes the same `ahc.events.v1` key, so a reading entered in safe mode appears in the rich UI and the reverse. The web app has an "Enter safe mode" control and an error boundary that links to `/safe/`.
`ponytail:` the native binary uses its own `events.json`, seeded from the demo, and does not share live state with the Tauri webview. Upgrade path: a Tauri command that reads and writes the same file.

**Desktop.** Tauri v2 wraps `apps/web/dist`. No Rust logic in the shell beyond a command to launch the safe-mode binary. It is the last thing SAFE builds; a browser tab is an acceptable fallback.

**Backend.** axum, in-memory `Mutex<Vec<_>>` plus an append-only JSONL file so a restart keeps the demo state. Basic auth on ground routes from env vars. `ponytail:` single process, single file, no database; enough for one crew and one judge at a time.

**Roles and privacy.** `settings.role` is `astronaut` or `flight_surgeon`. Crew and leaderboard views get `Score` only (see contract section 6). The ground face is the only one that shows another person's values.

**Local AI (stretch, after M3).** Explainer text v1 is the registry's `what` / `why_in_space` plus the engine's `reason`: deterministic and already sourced. If time allows, WEB adds an optional local model (Ollama on `localhost:11434`) that rephrases those texts and drafts the SBAR wording. It receives the engine result and registry text, shows the engine status above its answer, has an off switch, and the app must be complete without it.

**Mobile.** React Native (Expo), Android only, driven by the same person as WEB. **The app is a thin native shell around the crew website** (`react-native-webview`), so everything on the website is on the phone with the same look, and nothing is rebuilt twice. The WebView is Chromium, so the website runs the WASM engine itself on the phone, exactly as in a browser: Law 1 holds with no API call. Dev builds load the local Vite server (`http://10.0.2.2:5180` from the emulator); release builds load the deployed site, set by `EXPO_PUBLIC_WEB_URL`. Native code is only for what a browser cannot do: camera pulse logs a `reading` with `confidence: "low"`, which the engine turns into "confirm with a proper device" (workflow W5); Health Connect imports heart rate and sleep as `reading` events with `device: "health_connect"`. Those readings reach the website's event log through a small WebView message bridge (a WEB change, requested when MOBILE-2 starts). `ponytail:` the phone needs the network to load the site; it is not offline-first yet. Bundling the site into the APK needs an in-app server or `WebViewAssetLoader`, because WebView cannot fetch the engine's `.wasm` over `file://`. Contract types are shared with the web by type-only imports from `apps/web/src/contract`.

**Hosting.** Web static build + `/safe/` on Cloudflare Pages (public link for judges; free and works with a private repo). API on the Oracle VM behind HTTPS. Both must stay up until winners are announced (January 2027).

## 4. Guardrails the code enforces

- Gamification input type has no value field.
- The AI is never passed a function that writes events or status.
- Readings carry their environment; switching environment never rewrites history.
- Low confidence never flags.
- Every range carries `source.status`; the UI shows it.

## 5. The demo story (every feature must appear here or be cut)

1. Mars transit, day 143. Body view opens: heart, head and mind regions at Watch. Top flag card.
2. Today shows a bonus task: blood pressure check (it is stale).
3. Guided measurement card. Astronaut enters 138 / 88.
4. Engine: Caution. Heart region turns amber. Hazard lens shows isolation and closed environment.
5. "Why?" shows the explainer with source chips.
6. Guidance: ladder at Self-care, actions filtered by kit. Take the electrolyte drink (kit decrements). Set recheck.
7. Skip 2 h. Recheck still high: ladder moves to Consult ground. SBAR drafted, sent, countdown to arrival.
8. Ground Station inbox receives it after the delay. Flight surgeon replies. Note arrives at crew after the delay.
9. Comms blackout / rich UI "lost": safe-mode console shows the same status and accepts a reading.
10. Crew screen: XP and streak tick up, leaderboard shows no health values.
11. (If mobile lives) phone camera pulse reads low confidence: "confirm with a cuff".
