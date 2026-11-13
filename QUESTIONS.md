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
