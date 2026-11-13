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
