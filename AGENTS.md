# Astronaut Health Companion: rules for every agent and human

NASA Space Apps 2026, challenge: *Create Health Monitoring Software for Astronauts on Space Missions*.
The 48-hour clock is running. Read this file, then the `AGENTS.md` in your own folder, then `docs/plan.md` for your tasks.

| Read | For |
|---|---|
| `TEAM.md` | Who is responsible for what, who decides, who covers |
| `docs/challenge.md` | What the judges asked for |
| `docs/architecture.md` | How the pieces fit and every cross-cutting decision |
| `docs/contract.md` | The shared types, engine API, storage keys and HTTP API |
| `docs/plan.md` | Milestones, your task list, cut lines |
| `docs/reference/` | Original idea docs (UI screens and workflows, stack notes) |
| `design/` | **UI look and the colour code.** The look is not final: check `design/references/` before any UI work, and use only tokens from `design/tokens.css` |

## The three laws

1. **Status is computed once, in `crates/core`.** No thresholds, ranges, status or escalation logic in TypeScript (web or mobile), egui code or the API. You call the engine and render what it says. If two faces disagree during the demo, we lose.
2. **Stay in your folder.** You edit only what the ownership table gives you. Need something elsewhere? Open an issue or a PR request to that owner (see "Asking another owner").
3. **Simulated data is never called NASA data.** Every range shows its source status (`sourced`, `adapted`, `placeholder`). No medication doses, anywhere.

## Ownership

| Role | Owns (may edit) |
|---|---|
| **ENGINE** | `crates/core`, `crates/core-wasm`, `crates/api`, `apps/web/src/contract` (generated), `apps/web/src/faces/ground` |
| **WEB** | `apps/web` except `src/contract` and `src/faces/ground` |
| **SAFE** | `apps/egui`, `apps/desktop` |
| **MOBILE** | `apps/mobile` (and `crates/core-ffi` only if the on-device engine stretch is started) |
| **DEVOPS** | `.github/` (workflows, templates), `deploy/` |
| **PITCH** | `pitch/` (deck, PDFs, brag video, travel case) |
| **PLANNER** | root files, `docs/`, `design/` |

Root `Cargo.toml`: each owner may add **only their own crate** to `members`, nothing else.
People, decisions and cover are in `TEAM.md`. Drivers: put your GitHub handle there in your first PR.

## Git workflow

- **One worktree per agent**, so nobody overwrites anybody's uncommitted files:
  `git worktree add ../ahc-<area> -b <area>/<topic> origin/main`
- **Branches:** `<area>/<short-topic>`, area is one of `core api web ground egui desktop mobile pitch docs ci deploy`. Example: `web/body-figure`.
- **Never push to `main`.** Everything lands through a PR reviewed by @Tu2525.
- **One feature or one fix per PR.** Small (aim under ~400 changed lines; generated files do not count). If you did two things, open two PRs.
- **Rebase on `origin/main` before opening a PR and whenever main moves.** Never force-push someone else's branch.
- Squash-merge. The PR title becomes the commit on main, so it must be a good commit message.

### Commits and PR titles: Conventional Commits

`<type>(<area>): <imperative summary, lower case, no full stop>`

Types: `feat fix refactor test docs ci chore`. Add `!` for a contract-breaking change.

```
feat(web): colour body regions from engine status
fix(core): judge readings by the environment they were taken in
feat(core)!: add Report and Sbar to the contract
```

Commit body (when not obvious): what changed and **why**, not how. Commit often; each commit should build.

### Pull requests

Fill in the template. A reviewer who has not seen your session must be able to understand and test the PR from the description alone:

- **What**: the one thing this PR does, in plain words.
- **Why**: which task ID from `docs/plan.md` (for example `WEB-3`) and which demo story step it serves.
- **How to test**: exact commands, then what to click and what you should see.
- **Screenshots** for any UI change: required, taken by you, rules below.
- Say what you did **not** do or could not verify. Do not claim tests pass unless you ran them.

Before opening the PR: run your area's checks (in your folder's `AGENTS.md`), then have a subagent review your diff.

### Screenshots in PRs (required for anything visual)

The reviewer judges UI by looking at it. **The agent takes the photos itself**; do not leave it to the driver.

**When:** every PR that touches `apps/web`, `apps/egui`, `apps/mobile` or `apps/desktop`. CI (`pr-screenshots`) fails the PR if the description has no image. If the change truly cannot be seen (a refactor, a type fix), write a line `No visual change: <reason>` instead.

**What to capture** (real app, running, on the demo seed; never a mock-up or an edited image):

- Every screen or component the PR adds or changes.
- Each state the PR handles: at least the normal one and the flagged one; add stale, empty, error or blackout when the PR deals with them.
- For a fix or a restyle: **before and after**, side by side.
- Web: once at desktop width and once at phone width (about 390 px).
- If a reference exists in `design/references/`, put it next to your screenshot.

**How to capture**

| Face | How |
|---|---|
| Web, ground | Run `npm run dev`, open it in your browser tool (Claude Code's browser, Playwright, or Chrome DevTools) and screenshot the page |
| Safe mode | Screenshot the WASM build in the browser the same way, or the native window |
| Mobile | `adb exec-out screencap -p > shot.png` from the device or emulator |

**Where they go**

1. Save PNGs in `docs/screenshots/<area>/`, named `<TASK-ID>-<screen>-<state>.png`, for example `docs/screenshots/web/WEB-2-body-flagged.png`. Keep each under about 500 KB (crop, do not paste a 4K desktop).
2. Commit them in the same PR. They stay in the repo on purpose: PITCH uses them for the deck.
3. In the PR description, under **Screenshots**, embed each one by commit, so the link survives the branch being deleted:

```
![Body view, flagged, desktop](https://github.com/Tu2525/astronaut-health-companion/blob/<commit-sha>/docs/screenshots/web/WEB-2-body-flagged.png?raw=true)
```

4. Put one line of caption under each: what it shows and what to look at.

Look at your own screenshot before you post it. If it shows a broken layout, an error overlay or an empty page, the PR is not ready.

## Use subagents

Use subagents (Claude Code: the Agent/Task tool; Codex: its equivalent) instead of doing everything in one long context:

- **Delegate:** codebase searches, boilerplate and scaffolding, writing tests for code you just wrote, a review of your diff before every PR, reading long docs, independent screens or components that do not share files.
- **Keep in the main session:** design decisions, anything touching the contract, final integration.
- Give each subagent a complete brief (it starts with no context): the files, the goal, what done looks like, and "do not edit outside `<your folder>`".
- Run independent subagents in parallel. Check what they produced before you rely on it.

## Asking another owner

- **Contract change** (new field, type, engine behaviour): open an issue titled `contract: <what you need>` with the exact shape you want and why. ENGINE answers fast and ships it as its own PR. Do not work around it by computing things locally.
- Until it lands, mock **the data**, not the logic: hard-code a JSON value of the shape you asked for.
- **CI or deploy change** (a new build step, a secret, a workflow fix): open an issue titled `devops: <what you need>` with the exact command that works locally. DEVOPS owns `.github/` and `deploy/`; do not edit workflows yourself.
- Blocked more than 30 minutes: say so in the team chat and move to your next task.

## Questions

Unsure about a decision that `AGENTS.md` and `docs/` do not settle? Append it to `QUESTIONS.md` at the repo root (the template is in that file), commit it alone, push, and tell your driver. It is relayed to the Opus planner session, which writes the answer into the file. **Do not stop and wait:** write the assumption you are proceeding on and keep going. Do not guess silently on anything that touches the contract, medical wording, or another owner's folder.

## Definition of done

- Builds and your area's checks pass locally.
- Works with the backend **off** and the AI **off** (offline-first).
- Shows status as **colour + icon + word**, never colour alone.
- No raw colours: every colour is a token from `design/tokens.css` (or your face's one mirror file). Matches the reference in `design/references/` if one exists.
- No logic that belongs in the engine.
- UI change: screenshots are in the PR description (see "Screenshots in PRs").
- PR description lets @Tu2525 test it in two minutes.
