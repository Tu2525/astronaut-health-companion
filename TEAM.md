# Who is responsible for what

One page to answer "who do I ask?" and "whose job is this?". Folder rules are in `AGENTS.md`; task details are in `docs/plan.md`.

## People

Fill in the **Person** column (first PR by each driver, or @Tu2525 edits it). A *driver* is the human running that agent session: they start it, relay questions, and open its PRs.

| Role | Person (GitHub) | Agent | Responsible for | Folders | Tasks |
|---|---|---|---|---|---|
| **PLANNER / reviewer** | @Tu2525 | Claude (planner, Opus for questions) | Reviews and merges every PR. Plan, scope cuts, answers in `QUESTIONS.md`, design references, sources | root, `docs/`, `design/`, `.github/` | PLAN-1..3 |
| **ENGINE** | @Mostafa-Emad-El-Din | NA | The contract and the engine: every status decision. Ground Station backend and its screens. Answers `contract:` issues within 15 min | `crates/core`, `crates/core-wasm`, `crates/api`, `apps/web/src/contract`, `apps/web/src/faces/ground` | ENGINE-1..10 |
| **WEB** |@AMRYB | NA | The hero Crew App: body figure, measure, guidance, today, crew, comms, trends, library, settings. The public demo link looking good | `apps/web` (except ENGINE's two folders and `public/safe/`) | WEB-1..13 |
| **SAFE** | _TBD_ | NA | Safe-Mode Console (native + `/safe/` in the browser), Pages deploy, Tauri desktop shell | `apps/egui`, `apps/desktop`, `apps/web/public/safe/` | SAFE-1..7 |
| **MOBILE** | @AMRYB | NA | React Native companion: camera pulse, Health Connect. Honours the T+6 kill switch. Comes second to WEB | `apps/mobile`, `crates/core-ffi` | MOBILE-1..6 |
| **PITCH** | @Tu2525, @m7mdnasrr | NA | Everything judges see besides the product: travel case, PowerPoint deck, brag video, PDFs and presentations, demo video, submission page | `pitch/` | PITCH-1..7 |

Still unassigned: SAFE.
@AMRYB drives two roles: one session and one worktree per role, never one session editing both folders.

## Who decides

| Decision | Who |
|---|---|
| Anything in the contract (types, engine behaviour, API shapes) | ENGINE proposes, @Tu2525 approves the PR |
| How a screen looks | The reference in `design/references/` if there is one; otherwise the face's owner, using `design/tokens.css` |
| Colours and tokens | Humans via `design/tokens.css`; nobody invents colours |
| Thresholds, ranges, medical wording | ENGINE, in `crates/core/registry.json`, with an honest source status |
| Cutting scope / firing the mobile kill switch | @Tu2525, following the cut lines in `docs/plan.md` |
| Merging to `main` | @Tu2525 only |
| Open questions | Asked in `QUESTIONS.md`, answered by the planner |

## Shared jobs with one named owner

| Job | Owner | Helps |
|---|---|---|
| Deployed web link (GitHub Pages, includes `/safe/`) | SAFE | WEB |
| Deployed API (Oracle VM) | ENGINE | PLANNER |
| The web build step that pulls in the safe-mode WASM | WEB (their `package.json`) | SAFE supplies the command |
| Demo seed data and its tests | ENGINE | everyone reports mismatches |
| Faces agreeing on status | ENGINE | SAFE and MOBILE compare against web before each PR |
| Sources and "NASA data used" section | PLANNER | ENGINE (registry citations) |
| AI-use disclosure (to build it, and inside it) | PLANNER | everyone lists what they used in their PRs; PITCH puts it in the deck |
| Demo video, deck, brag video, PDFs | PITCH | each owner records their part and supplies screenshots |

## Cover: if someone is stuck or away

| Role | Covered by | Notes |
|---|---|---|
| ENGINE | PLANNER | contract issues cannot wait |
| WEB | SAFE first; @AMRYB's own mobile time after the kill switch | WEB has the most screens; helpers take whole screens (WEB-9, WEB-11, WEB-12), in their own folders |
| SAFE | ENGINE | both are Rust |
| MOBILE | nobody | it is cut instead |
| PITCH | the two owners cover each other | |

Helping in someone else's folder: only on a task they handed you, in a folder they named, and say so in the PR.
