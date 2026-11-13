# Build plan

Times are hours since the team starts building from this repo (T+0). The hackathon clock is already running, so if you are behind a milestone, cut scope (see "Cut lines"), do not slip the milestone.

Each task = one PR unless it says otherwise. PR title uses the task's area; mention the task ID in "Why".

## Milestones

| | By | Gate (must be true on `main`) |
|---|---|---|
| **M0 Hello engine** | T+2 | Every face prints the demo `Assessment.overall` ("watch") from the real engine |
| **M1 Vertical slice** | T+8 | Web: body figure coloured by engine, enter a reading, figure and flag card update |
| **M2 Core loop** | T+18 | Demo story steps 1-7 work in the web app; API accepts and delays a report |
| **M3 All faces** | T+28 | Steps 8-10: ground inbox and reply, safe mode at `/safe/`, crew score. Mobile go/no-go decided |
| **M4 Honest and public** | T+36 | Deployed link works on a clean browser, ranges sourced or labelled, library screen, README |
| **M5 Freeze** | T+42 | Feature freeze. Only fixes. Record the demo video, write the submission |
| Submit | T+46 | Two hours of slack before the deadline. Do not use it for features |

## ENGINE (`crates/core`, `core-wasm`, `api`, ground face)

| ID | Task | Done when |
|---|---|---|
| ENGINE-1 | `crates/api` skeleton: axum, `/api/health`, `/api/assess`, `/api/demo`, `/api/registry`, CORS, add to workspace. **Do this first: MOBILE is blocked on it** | `cargo run -p ahc-api` answers all four; `curl` examples in the PR |
| ENGINE-2 | Contract: `Report`, `Sbar`, `draft_sbar()` in core + wasm export + TS types | test asserts SBAR for the BP flag; `contract.md` updated |
| ENGINE-3 | API reports and notes with two-way delay, JSONL persistence, basic auth | test: report invisible before `deliver_at`, visible after |
| ENGINE-4 | Prompt scheduler in core: `due_tasks(input) -> Task[]` (routine slots + bonus tasks for stale or drifting indicators; bonus tasks never break streaks) | test: demo state yields a BP bonus task |
| ENGINE-5 | Ground face: C2 inbox, C4 report detail, C5 compose note (in `apps/web/src/faces/ground`, after WEB-1 lands) | story step 8 works against a local API |
| ENGINE-6 | Source the registry: replace placeholder ranges with cited NASA / agency sources where they exist, mark the rest honestly; add env-specific ranges where justified | each indicator has a real citation or stays `placeholder` on purpose; list in `docs/sources.md` |
| ENGINE-7 | Time-to-help in core: given delay and escalation, say whether waiting for ground is safe (ladder wording by environment) | field on `Flag`, test per environment |
| ENGINE-8 | Second and third seeded crews (lunar base, station) producing `Score`s for the leaderboard | `demo()` exposes crew scores, no values |
| ENGINE-9 | C1 crew overview, C6 event timeline with conflict line ("note arrived 40 min after your action") | stretch |

Order: 1, 2, 3 by M2. 4, 5 by M3. 6, 7, 8 by M4. Deploying the API is DEVOPS-3: give them the run command and the env vars it reads.
You are also on call for `contract:` issues: answer within 15 minutes, they block other people.

## WEB (`apps/web`)

| ID | Task | Done when |
|---|---|---|
| WEB-1 | Scaffold: Vite React TS, `npm run engine`, `engine.ts`, store, hash router, theme tokens, status chip component (colour + icon + word), top bar | page shows "Overall: Watch" from WASM; `npm run lint && npm run build` pass |
| WEB-2 | A1 Body view: inline SVG figure, 10 regions coloured from `assessment.regions`, top flag card, states: nominal, flagged, cold start, stale | story step 1 |
| WEB-3 | A5 Measure: picker, guided card from registry, keypad input with unit conversion, appends a `reading`, result screen | story step 3-4 |
| WEB-4 | A7 Guidance: situation, ladder, kit-filtered actions, start action (`action_taken` + `kit_use`), set recheck, send to ground | story step 6 |
| WEB-5 | A2 Region detail + hazard lens chips on A1 + "Why?" explainer card with source chips and source status | story steps 4-5 |
| WEB-6 | A12 Ground comms: outbox with arrival countdown, inbox polling `/api/notes`, blackout state; demo time-skip control | story steps 7-8 (crew side) |
| WEB-7 | A4 Today: routine timeline + bonus tasks from `due_tasks`; A6 Check-in (3 taps) | story step 2 |
| WEB-8 | A10 Crew: XP, streak, badges, leaderboard from `Score`s, privacy label | story step 10 |
| WEB-9 | A3 Indicator detail + A9 Trends: chart with baseline band, preflight line, 24 h / 7 d / mission | charts read from the log |
| WEB-10 | Safe-mode entry: error boundary + manual button linking to `/safe/`; `storage` event sync | story step 9 |
| WEB-11 | A13 Library (registry browser, doubles as evidence page) + A14 Settings (environment, units, role, AI off, reset demo) | |
| WEB-12 | Polish pass: motion, empty states, accessibility (keyboard, contrast, text size), mobile-width layout | |
| WEB-13 | Optional local AI explainer (see architecture) | stretch, only after M3 |

Order: 1 by M0. 2, 3 by M1. 4, 5, 6, 7 by M2. 8, 9, 10 by M3. 11, 12 by M4.
WEB and MOBILE have the same driver (@AMRYB), in separate sessions and worktrees. WEB comes first whenever the two compete.
WEB has the most screens: run independent screens in parallel subagents once WEB-1 is merged (they touch different folders under `src/faces/crew/`).

## SAFE (`apps/egui`, `apps/desktop`)

| ID | Task | Done when |
|---|---|---|
| SAFE-1 | `apps/egui` eframe app, native, added to workspace: prints demo assessment in a table | `cargo run -p ahc-safe` shows the status board |
| SAFE-2 | B1 Status board: indicator table (name, value, status word + icon, last measured), overall, flags with escalation step, comms line, SAFE MODE banner; deliberate high-contrast style | matches web statuses exactly |
| SAFE-3 | WASM build with trunk, output to `apps/web/public/safe/` at build time (coordinate the one line in web's build with WEB), storage adapter: localStorage on web, JSON file native | `/safe/` loads next to the web app and shows the same log |
| SAFE-4 | B2 Quick entry (keyboard-first) appends a `reading`; B3 Protocol card from flag actions | reading entered in `/safe/` appears in the web app |
| SAFE-5 | `apps/desktop` Tauri v2 shell around the web build, "Enter safe mode" launches the native console | Windows build runs |
| SAFE-6 | Help WEB: take WEB-9 or WEB-11 if free | |

Order: 1 by M0. 2 by M1. 3, 4 by M3 (aim for M2). 5 is cut first if late. Publishing `/safe/` is DEVOPS-2: give them the trunk command that works.

## MOBILE (`apps/mobile`), React Native

| ID | Task | Done when |
|---|---|---|
| MOBILE-1 | Expo + TypeScript app, development build on Android, type-only contract imports, `src/theme.ts`, API client; screen shows overall status from `POST /api/assess` on the demo seed | runs on a device or emulator; `typecheck`, `lint`, `test` scripts pass. **Kill switch: T+6** |
| MOBILE-2 | D2 Camera pulse: fingertip on lens + torch, live signal, bpm estimate, confidence meter, logs `reading` with `confidence: low`, shows the engine's "confirm with a cuff" | workflow W5 |
| MOBILE-3 | D3 Health Connect: permissions, import heart rate and sleep as `reading` events | values appear in the status list |
| MOBILE-4 | D1 Home (status + top flag + next task) and D4 Quick check-in | |
| MOBILE-5 | On-device engine through a native Rust module (`crates/core-ffi`), so the phone works with the API off | stretch |
| MOBILE-6 | Debug APK built locally; screen recording for the video | |

Blocked on ENGINE-1 for the real endpoint; until it lands, develop against a saved JSON response.
If the kill switch fires: keep whatever runs, record it, and the time goes to WEB-9, WEB-11 or WEB-12.

## PLANNER / humans

| ID | Task |
|---|---|
| PLAN-1 | Review and merge PRs quickly (target: under 20 minutes). A waiting PR blocks an agent |
| PLAN-2 | Verify the challenge Resources tab and what NASA OSDR actually offers; feed ENGINE-6 |
| PLAN-3 | `docs/sources.md` and the "NASA data used" + "AI use" sections of the submission (AI used to build it, AI inside it) |

## DEVOPS (`.github/`, `deploy/`): @Tu2525 and @m7mdnasrr

| ID | Task | Done when |
|---|---|---|
| DEVOPS-1 | Keep CI green and quick: fix workflow problems, add build steps owners ask for (`devops:` issues), confirm `pr-screenshots` works on the first UI PR | every PR gets a result in a few minutes |
| DEVOPS-2 | Cloudflare Pages deploy workflow: GitHub Actions builds the web app with the engine WASM, build the safe-mode WASM into `/safe/`, upload the folder with `wrangler pages deploy` on every push to `main` | public link loads on a clean browser, `/safe/` included (needed by M4, aim for M2 so judges' link exists early) |
| DEVOPS-3 | API on the Oracle VM: build or copy the `ahc-api` binary, systemd service, HTTPS (reverse proxy), basic-auth and CORS from env vars, files in `deploy/` | public `/api/health` answers over HTTPS; restart survives a reboot |
| DEVOPS-4 | Secrets and settings: Actions secrets (`CLOUDFLARE_API_TOKEN`, `CLOUDFLARE_ACCOUNT_ID`), the Cloudflare Pages project, branch protection, the web app's API URL as a build variable | nothing secret in the repo; documented in `deploy/README.md` |
| DEVOPS-5 | Stay-up plan: both links must work until winners are announced (January 2027); a simple uptime check and a note on how to redeploy | written in `deploy/README.md` |
| DEVOPS-6 | Release for submission: tag the commit the video was recorded on, attach the mobile APK if mobile lives | tag exists, links in the submission |

Order: 1 always. 4 now. 2 as soon as WEB-1 is merged. 3 first deploy after ENGINE-1 (health only), again after ENGINE-3. 5 by M4. 6 at M5.

### DEVOPS steps

Split (change it if you prefer): **@Tu2525** does what needs an account or the VM (wrangler is logged in and the SSH key is on that machine). **@m7mdnasrr** writes what lives in the repo (workflows, scripts, service files). Tick a box in the PR that finishes it.

**Now, before any app code exists**

- [ ] `4a` @Tu2525: create the Cloudflare Pages project with wrangler (direct upload, not the Git connection) and upload a one-line placeholder page. Write the `pages.dev` link in `deploy/README.md`.
- [ ] `4b` @Tu2525: create a Cloudflare API token limited to Pages edit. Add `CLOUDFLARE_API_TOKEN` and `CLOUDFLARE_ACCOUNT_ID` as Actions secrets.
- [ ] `4c` @Tu2525: check the VM: SSH works, note its CPU type (Oracle free VMs are often ARM), open ports 80 and 443 in **both** the Oracle security list and the VM firewall.
- [ ] `4d` @Tu2525: decide the API hostname. HTTPS needs a name, not a bare IP: a domain you own, or a free one that maps to the IP. Write it in `deploy/README.md`.
- [ ] `1a` @m7mdnasrr: read the four workflows in `.github/workflows/`, open a test PR and confirm each one reports. Note the run time of each.

**When WEB-1 merges (DEVOPS-2)**

- [ ] `2a` @m7mdnasrr: `.github/workflows/deploy.yml`, on push to `main`: build the engine WASM, `npm run build` in `apps/web`, then `wrangler pages deploy` of the output folder. Reuse the setup steps from `web.yml`.
- [ ] `2b` @m7mdnasrr: pass the API address to the build as an Actions variable (not a secret, it is public), never hard-coded.
- [ ] `2c` @Tu2525: open the link in a private window with the network tab open: page loads, WASM loads, no request fails when the API is off.
- [ ] `2d` @m7mdnasrr, when SAFE-3 merges: add the `trunk build` step so safe mode lands in `/safe/` of the same upload. SAFE gives the command.

**When ENGINE-1 merges (DEVOPS-3)**

- [ ] `3a` @m7mdnasrr: `deploy/ahc-api.service` (systemd, restarts on failure, reads an env file) and `deploy/Caddyfile` (HTTPS in front of the API).
- [ ] `3b` @m7mdnasrr: `deploy/deploy-api.sh`: over SSH, pull `main`, `cargo build --release -p ahc-api` **on the VM** (avoids cross-compiling for ARM), restart the service.
- [ ] `3c` @Tu2525: on the VM, install Rust and Caddy, create the env file with the ground user and password (never in the repo), run the script.
- [ ] `3d` @Tu2525: `https://<host>/api/health` answers from a phone on mobile data. Reboot the VM once and check it comes back.
- [ ] `3e` both: from the deployed web page, a call to the API succeeds (CORS allows the `pages.dev` origin) and the ground routes refuse a wrong password.
- [ ] `3f` after ENGINE-3: redeploy, check reports survive a restart (the JSONL data folder is outside the checkout).

**Ongoing (DEVOPS-1)**

- [ ] `1b` answer `devops:` issues quickly; a missing check blocks someone's PR.
- [ ] `1c` first UI PR: confirm `pr-screenshots` fails without an image and passes with one.
- [ ] `1d` watch Actions minutes: a private repo gets a limited free amount per month and Rust builds use it fast. Keep the caches working; if it runs low, tell @Tu2525.

**By M4 and M5 (DEVOPS-5, DEVOPS-6)**

- [ ] `5a` @m7mdnasrr: fill in "how to redeploy" and "how to check both are up" in `deploy/README.md`, tested by the other owner following it.
- [ ] `5b` @Tu2525: a free uptime check on both links that emails on failure. They must stay up until January 2027.
- [ ] `6a` @Tu2525: tag the commit the demo video was recorded on; attach the APK if mobile lives; put both links in the submission.
Hosting is Cloudflare Pages, not GitHub Pages: GitHub Pages on a private repo needs a paid plan. We build in GitHub Actions and upload, because Cloudflare's own builder has no Rust toolchain ready. The API must allow the `pages.dev` origin (CORS) and be HTTPS, or the browser blocks it.

## PITCH (`pitch/`): @Tu2525 and @m7mdnasrr

Rules and folder layout: `pitch/README.md`.

| ID | Task | Done when |
|---|---|---|
| PITCH-1 | Travel case | scope to be confirmed by @Tu2525 |
| PITCH-2 | PowerPoint deck: problem, the five hazards, demo story, architecture in one slide, data honesty, AI use, team | `pitch/deck/deck-v1.pptx` + exported PDF; outline by M2, real screenshots by M4 |
| PITCH-3 | Brag video: short launch video of the product | `pitch/brag/`; needs the deployed link (after M4) |
| PITCH-4 | PDFs: one-pager and project summary for the submission | `pitch/pdf/` |
| PITCH-5 | Demo video script from architecture section 5; rehearse twice on the deployed link; record | by M5 |
| PITCH-6 | Submission page: title, summary, links, team | by T+46 |
| PITCH-7 | Extra or shortened presentations for live judging | stretch |

Start the deck outline and the video script now; they do not depend on code. Screenshots and recordings wait for M3 and M4.

## Cut lines (in this order)

1. Tauri desktop shell (SAFE-5): show the web app in a browser.
2. Mobile (after the kill switch).
3. Local AI (WEB-13).
4. Trends multi-indicator view (keep the single indicator chart).
5. Ground overview and timeline (ENGINE-9).
6. Earth Mode, onboarding flow, buddy checks.

Never cut: body figure, guided measurement, guidance with ladder, delayed ground loop, safe mode in the browser, source honesty.

## Risks

| Risk | Guard |
|---|---|
| Faces disagree on status | Law 1; SAFE-2 and ENGINE-5 compare against the web on the demo seed |
| Contract churn | Only ENGINE edits it; `!` in the PR title; announce in chat |
| One driver for WEB and MOBILE | WEB first; mobile kill switch; SAFE takes whole web screens when free |
| WASM toolchain eats time | WEB-1 and SAFE-3 are first-hour checks; ask ENGINE at once if stuck |
| Deploy left to the last hour | DEVOPS-2 starts right after WEB-1, with an empty page if need be |
| Merge queue jams | Small PRs, one feature each, PLAN-1 |
| Demo breaks offline | Definition of done: works with API and AI off |
| Wrong medical claims | Placeholder labels, no doses, engine text only |
