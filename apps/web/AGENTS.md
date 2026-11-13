# WEB: `apps/web/`

Read the root `AGENTS.md` first. Tasks: `docs/plan.md`, section WEB. Screen specs: `docs/reference/ui_faces_and_workflows.md` (faces A and C). Shapes: `docs/contract.md`.

**You own:** everything in `apps/web` except:
- `src/contract/` (generated from Rust by ENGINE; never edit, never copy types by hand)
- `src/faces/ground/` (ENGINE builds the Ground Station there)
- `public/safe/` (build output of `apps/egui`, owned by SAFE)

This is the hero. Judges click this link. It must look designed, not generated: dark cockpit, calm by default, only what needs attention is bright.

## Stack (decided)

React + Vite + TypeScript, static build, hash routes, CSS variables from `design/tokens.css` (import it once; never write a raw colour; read `design/README.md` and check `design/references/` before building a screen, the look is not final), inline SVG for the body figure (no WebGL, no 3D). Add a dependency only when a few lines of your own will not do.

## Layout to create in WEB-1

```
src/
  contract/        generated types (import from here)
  engine-pkg/      wasm-pack output, gitignored
  engine.ts        THE ONLY file that imports the WASM. Typed wrappers: assess, registry, demo, score
  store.ts         events + settings + derived assessment; append(event) is the only write
  clock.ts         now_met_s, earth_ms, MET formatting, demo time skip
  ui/              shared: StatusChip, TopBar, Card, Ladder, SourceChip
  faces/crew/<screen>/   one folder per screen (body, region, measure, guidance, today, crew, comms, ...)
  faces/ground/    ENGINE's
```

`package.json` scripts CI expects: `engine` (wasm-pack build `../../crates/core-wasm --target web --out-dir ../../apps/web/src/engine-pkg`), `dev`, `lint`, `build`.

## Rules

- **Never compute status.** No comparisons against thresholds, no "if value > x". Render `assessment`. If you need something the engine does not give you, open a `contract:` issue and mock the data shape meanwhile.
- All explanatory text (what, why, how to measure, actions, sources) comes from `registry()`. Do not hard-code medical text in components.
- Status is always `StatusChip`: colour + icon + word. Mapping table in `docs/contract.md` section 2.
- State lives in the event log (`localStorage` keys in `docs/contract.md` section 8). To change anything, append an event. Never mutate or delete events.
- Time comes from `clock.ts`, never `Date.now()` in screens.
- Leaderboard and crew views receive `Score` only.
- Must work with the API unreachable and the AI off. Network calls are fire-and-forget with a visible queued state.
- Every number has a "why" affordance; every range shows its source status.
- One screen per folder, so parallel subagents and PRs do not collide. Shared components go through `ui/`; change those in their own small PR.
- Accessibility basics are not optional: keyboard reachable, visible focus, contrast, respects reduced motion, works at phone width.

## Commands

```bash
npm install
npm run engine     # needs Rust + wasm-pack (cargo install wasm-pack) + wasm32-unknown-unknown target
npm run dev
npm run lint && npm run build
```

For any UI PR: run it, look at it in the browser, and take the screenshots yourself (desktop and phone width, each changed state) following "Screenshots in PRs" in the root `AGENTS.md`. CI rejects a UI PR without them. "It compiles" is not tested.
