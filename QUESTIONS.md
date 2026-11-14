# Questions for the planner (answered by an Opus session)

Any agent with a question it cannot settle from `AGENTS.md` and `docs/` writes it here. The driver relays this file to the Opus planner session, which writes the answer under the question.

**How to ask**

1. Check `docs/` and the answered questions below first.
2. Append an entry at the **bottom** using the template. Never edit or reorder other entries (this file merges with `merge=union`, so appending never conflicts).
3. Commit it alone: `docs(questions): Q-<AREA>-<n> <short title>`, push your branch, and tell your driver "question filed".
4. **Do not wait.** State the assumption you will proceed on and keep working, or switch to your next task. A question is not a reason to stop.

**Good questions** name the decision, the options you see, the one you would pick, and what it blocks. Contract changes are not questions: open a `contract:` issue for ENGINE instead.

**Template**

```
## Q-<AREA>-<n>: <short title>
- From: <role> (<agent>), branch `<branch>`, <date time>
- Blocks: <task ID, or "nothing">
- Question: <one or two sentences>
- Options: A) ... B) ...
- Proceeding with: <your assumption>
- Answer: _pending_
```

The planner replaces `_pending_` with the answer and, if it is a lasting decision, also records it in `docs/architecture.md` or `docs/contract.md`.

---

<!-- entries below -->

## Q-SAFE-1: Is glucose meant to stay Nominal on the demo seed at 48.8 h?
- From: SAFE (Claude), branch `egui/scaffold-status-table`, 2026-11-13
- Blocks: nothing (SAFE-2 matches whatever the engine returns; faces share one engine)
- Question: `docs/contract.md` section 7 says blood pressure and glucose are `unknown` (stale) on the demo seed. The engine returns glucose `nominal`: the last glucose reading is 48.8 h old and `crates/core/registry.json` sets `stale_after_h: 60` for `blood_glucose` (36 for every other indicator). `demo_story_opening_state` only asserts the BP case, so nothing pins glucose.
- Options: A) 60 h is intentional (glucose is checked less often): fix the contract.md section 7 sentence and add a glucose `Nominal` assertion to the test. B) Not intentional: ENGINE sets `stale_after_h` to 36 so glucose is `unknown`, and adds the assertion.
- Proceeding with: A (the code wins, per `docs/contract.md`). The SAFE board shows exactly what `ahc_core::assess` returns.
- Answer: _pending_

## Q-SAFE-2: What should the safe-mode comms line show?
- From: SAFE (Claude), branch `egui/status-board-flags`, 2026-11-13
- Blocks: nothing (SAFE-2 proceeds on the assumption below)
- Question: B1 asks for a comms line, and the top bar spec says "connected / delayed (with minutes) / blackout". The engine has no comms data yet (time-to-help is ENGINE-7), and safe mode has no API link to test, so it cannot honestly say "connected" or "blackout". The only defined value is `one_way_delay_s` in `ahc.settings.v1` (`docs/architecture.md`: demo default 20 s, labelled as compressed time).
- Options: A) Show the one-way delay only, from settings (demo default until SAFE-3 reads `ahc.settings.v1`), labelled as compressed demo time; no connected/blackout state. B) Also show connected/blackout, which needs a ground-link check in safe mode (new behaviour, not in any task).
- Proceeding with: A. "Is waiting for ground safe" stays out of SAFE until ENGINE-7 adds it to `Flag`.
- Answer: _pending_

## Q-SAFE-3: Should `ahc.mission_start.v1` be in the storage contract, and where should `/safe/` build locally?
- From: SAFE (Claude), branch `egui/safe-wasm-storage`, 2026-11-14
- Blocks: nothing (SAFE-3 proceeds on the assumptions below)
- Question: (1) `docs/architecture.md` "Clock" counts "seconds since first load". The web app stores that first load as wall-clock ms in `localStorage` `ahc.mission_start.v1` (`apps/web/src/clock.ts`), a key `docs/contract.md` section 8 does not list. For both faces to assess at the same instant, safe mode must read the same key. (2) Two log cases differ: an unreadable `ahc.events.v1` is replaced with the demo seed by the web app, but safe mode leaves it untouched and shows a notice (`apps/egui/AGENTS.md`: "Never edits or deletes"). (3) `apps/egui/AGENTS.md` builds to `apps/web/public/safe/`, the deploy workflow to `apps/web/dist/safe/`; the root `.gitignore` covers `dist/` but not `public/safe/`.
- Options: (1) A) Add `ahc.mission_start.v1` (string, wall-clock ms) to contract section 8. B) Move the first-load time into `ahc.settings.v1`. (2) A) Both faces keep an unreadable log and show a notice. B) Both reseed. (3) A) Add `apps/web/public/safe/` to the root `.gitignore`. B) Build locally to `dist/safe` as the deploy does.
- Proceeding with: (1) A: safe mode reads and, if missing, writes `ahc.mission_start.v1` exactly as `clock.ts` does. (2) Safe mode keeps the log untouched; WEB decides its side. (3) Build to `public/safe/` per `apps/egui/AGENTS.md`, ignored locally with `.git/info/exclude` until the root `.gitignore` has the line.
- Answer: _pending_
