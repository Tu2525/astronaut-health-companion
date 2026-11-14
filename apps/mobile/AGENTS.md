# MOBILE: `apps/mobile/`

Read the root `AGENTS.md` first. Tasks: `docs/plan.md`, section MOBILE. Screens: face D in `docs/reference/ui_faces_and_workflows.md` (it says Compose; we chose **React Native**).

**You own:** `apps/mobile`.
**Not yours:** `apps/web` (even though the same driver, @AMRYB, runs both: separate sessions, separate worktrees, separate PRs), `crates/`.

## What the app is now (MOBILE-1, web shell)

The Android app is a thin native shell around the real website in a full-screen `react-native-webview` (`App.tsx`). The phone shows exactly what the website shows, with the same look. The website runs the engine itself in WASM, so **no status, range or escalation logic exists in this app** (law 1). The native layer only adds: black background (no white flash), status-bar padding, hardware Back goes back inside the site, off-origin links open in the system browser, and a "Website unreachable" screen with a Retry button.

Native features that a browser cannot do (camera pulse, Health Connect) will be added next to the WebView as native modules; they are not built yet.

- Site URL: `EXPO_PUBLIC_WEB_URL`, default `http://10.0.2.2:5180` (the host's Vite dev server as seen from the emulator). **Release builds must set `EXPO_PUBLIC_WEB_URL=https://ahc-37u.pages.dev`** (it is inlined into the JS bundle at build time).
- Debug builds already allow cleartext http (Expo's debug manifest); release does not.
- The site keeps its event log in `localStorage` (`domStorageEnabled`); it persists across launches. WebView flushes storage to disk a few seconds after a write, so a force-stop within a second of a write can lose it.
- Edge-to-edge: only the top inset is handled (`StatusBar.currentHeight`). `react-native-safe-area-context` was tried and dropped: its CMake codegen path exceeds Windows' 260-character limit in this repo location, so the Android build fails.

### Known limit

The first load needs the network: the app is **not offline-first yet**. Bundling the site into the APK is not done, because Android WebView cannot fetch the engine's `.wasm` over `file://`; it would need an in-app local server or `WebViewAssetLoader`. Also, a firewalled host makes the load hang about 2 minutes before the error screen appears.

### Run dev

```bash
# the website must be running on the host (apps/web: npm run dev, port 5180)
cd apps/mobile && npm install
export ANDROID_HOME=<sdk>             # emulator started, `adb devices` shows it
npx expo run:android                  # native rebuild; needed after adding native modules
npx expo start --clear --dev-client   # Metro; use --clear, the watcher can serve stale bundles
adb reverse tcp:8081 tcp:8081
```

If port 8081 is taken, `expo run:android` moves to 8082: open the app with `adb reverse tcp:8082 tcp:8082` and the deep link `exp+zerogpulse://expo-development-client/?url=http%3A%2F%2F10.0.2.2%3A8082`. In the dev build the website's 3D body can fail to load (`Failed to fetch dynamically imported module .../BodyCanvas.tsx`, Vite's unbundled ~11 MB three/drei deps); the release APK, which loads the built site, renders it.

Before trusting a screenshot, check the bundle has your code: `curl -s "localhost:8081/index.bundle?platform=android&dev=true" | grep -c <new string>`.

### Build the release APK (points at the deployed site)

```bash
cd apps/mobile && EXPO_PUBLIC_WEB_URL=https://ahc-37u.pages.dev npx expo prebuild --platform android   # if android/ is missing
cd android && EXPO_PUBLIC_WEB_URL=https://ahc-37u.pages.dev ./gradlew assembleRelease
# -> android/app/build/outputs/apk/release/app-release.apk (signed with the debug key; do not commit; android/ is gitignored)
adb install -r app/build/outputs/apk/release/app-release.apk
```

## Original plan (kept for the later native tasks)

Where this conflicts with the section above (screens, API calls from the app, `EXPO_PUBLIC_API_URL`), the web shell wins; the camera pulse and Health Connect notes still apply.

### Why mobile exists

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
