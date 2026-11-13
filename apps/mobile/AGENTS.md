# MOBILE: `apps/mobile/` (and `crates/core-ffi/`)

Read the root `AGENTS.md` first. Tasks: `docs/plan.md`, section MOBILE. Screens: face D in `docs/reference/ui_faces_and_workflows.md` (it says Compose; we chose Flutter).

**You own:** `apps/mobile`, `crates/core-ffi`.
**Not yours:** `crates/core` (you consume it), everything else.

## Why mobile exists

Judges will not install an APK. The phone earns its place only by doing what a browser cannot:

1. **Camera pulse** (fingertip over lens and torch), labelled an estimate, with a confidence meter.
2. **Health Connect** as the stand-in for crew wearables (heart rate, sleep).
3. Quick check-in and prompts, if time remains.

Android only. Flutter web and desktop targets are out of scope: do not use Flutter for the body figure or a second dashboard.

## Kill switch

**MOBILE-1 must work within 6 hours of starting**: the app calls the Rust engine through the bridge and shows the demo `overall` status on a device or emulator. If not, stop, record whatever runs for the video, and move to helping WEB (tasks listed in the plan). Say so in chat; this is the plan working, not a failure.

## Rules

- Start from the flutter_rust_bridge template. `crates/core-ffi` is a thin wrapper around `ahc_core::assess_json`, `ahc_core::registry`, and the `demo` module. JSON strings across the bridge are fine and the fastest path; decode in Dart.
- **No thresholds or status logic in Dart.** The engine decides. Dart renders.
- Never hand-edit generated bridge files. They are gitignored; codegen runs in CI (add that step to `.github/workflows/mobile.yml` through a PR to PLANNER, or ask).
- Camera pulse always logs `confidence: "low"` and `device: "phone camera"`. The engine answers "confirm with a proper device": show that, do not soften or override it.
- Health Connect imports become `reading` events with `device: "health_connect"`, `confidence: "medium"`, values converted to the indicator's canonical unit (`docs/contract.md`).
- Pulse signal processing: do it in Dart unless the bridge is already solid, then Rust in `core-ffi` is cleaner. Do not spend the first hours on it either way.
- Packages: `health`, `camera`. Add others only when needed.
- Colours: one file `lib/theme.dart` mirroring `design/tokens.css` (same names). No colour literals anywhere else. Check `design/references/` for `mobile-*` files first.
- Status shown as colour + icon + word, same vocabulary as everywhere else.
- Add `crates/core-ffi` to workspace `members` in the root `Cargo.toml` (that line is yours).

## Commands

```bash
flutter pub get
flutter_rust_bridge_codegen generate
flutter analyze && flutter test
flutter run            # device or emulator
flutter build apk --debug
```
