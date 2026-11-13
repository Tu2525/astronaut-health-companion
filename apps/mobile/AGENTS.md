# MOBILE: `apps/mobile/`

Read the root `AGENTS.md` first. Tasks: `docs/plan.md`, section MOBILE. Screens: face D in `docs/reference/ui_faces_and_workflows.md` (it says Compose; we chose **React Native**).

**You own:** `apps/mobile`.
**Not yours:** `apps/web` (even though the same driver, @AMRYB, runs both: separate sessions, separate worktrees, separate PRs), `crates/`.

## Why mobile exists

Judges will not install an APK. The phone earns its place only by doing what a browser cannot:

1. **Camera pulse** (fingertip over lens and torch), labelled an estimate, with a confidence meter.
2. **Health Connect** as the stand-in for crew wearables (heart rate, sleep).
3. Quick check-in and prompts, if time remains.

Android only. No iOS, no React Native Web. The body figure and dashboards belong to the web app.

## Stack (decided)

- **Expo + TypeScript, with a development build** (`expo prebuild` / `expo run:android`). Expo Go is not enough: frame access to the camera and Health Connect are native modules.
- Suggested libraries: `react-native-vision-camera` (frames + torch) and `react-native-health-connect`. These are suggestions from memory, not verified against current releases: check each library's current docs and Expo compatibility before installing, and swap if another is simpler.
- Types: `import type { Assessment, Event, ... } from` the generated contract in `apps/web/src/contract` through a `tsconfig` path alias. Type-only imports are erased at build time, so Metro needs no extra config. Never copy the types by hand.

## How the phone reaches the engine

Hermes (React Native's JS engine) does not run WebAssembly, so the web's WASM bridge cannot be reused.

- **Now:** call the Ground API. `POST /api/assess` with an `AssessInput` returns the `Assessment`; `GET /api/demo` and `GET /api/registry` give the seed and the texts (`docs/contract.md` section 9). ENGINE ships these in ENGINE-1; until then mock **the data** with a saved JSON response.
  `ponytail:` the phone needs the API reachable to compute status. Show "engine unreachable" plainly when it is not; never guess a status locally.
- **Stretch (MOBILE-5):** an on-device engine through a native Rust module (`crates/core-ffi`, for example with uniffi-bindgen-react-native). Only after everything else works.

## Kill switch

**MOBILE-1 must work within 6 hours of starting**: a development build on a device or emulator shows the demo `overall` status from the API. If not, stop, record whatever runs for the video, and put the time into WEB. Say so in chat; this is the plan working, not a failure.

Web is the hero and has the same driver. If WEB is behind its milestone, mobile waits.

## Rules

- **No thresholds or status logic in the app.** No comparing values to ranges. The engine decides; you render.
- Camera pulse always logs a `reading` with `confidence: "low"` and `device: "phone camera"`. The engine answers "confirm with a proper device": show that, do not soften or override it.
- Health Connect imports become `reading` events with `device: "health_connect"`, `confidence: "medium"`, values converted to the indicator's canonical unit.
- Pulse signal processing lives in TypeScript (average red channel per frame, band-pass, peak count). Keep it simple and label the result an estimate. `ponytail:` naive peak counting; fine for a demo, not a medical measurement.
- Colours: one file `src/theme.ts` mirroring `design/tokens.css` (same names). No colour literals anywhere else. Check `design/references/` for `mobile-*` files first.
- Status shown as colour + icon + word, same vocabulary as everywhere else.
- Every PR includes screenshots you took from the device or emulator with `adb exec-out screencap -p` (root `AGENTS.md`, "Screenshots in PRs").
- The API base URL comes from one config value (`EXPO_PUBLIC_API_URL`), never hard-coded in screens.
- `package.json` scripts CI expects: `typecheck` (`tsc --noEmit`), `lint`, `test`.

## Commands

```bash
npm install
npx expo run:android      # development build on a device or emulator
npm run typecheck && npm run lint && npm test
```

The debug APK for the video is built locally (`npx expo run:android --variant debug`, or Gradle `assembleDebug` after `expo prebuild`); CI only type-checks, lints and tests.
