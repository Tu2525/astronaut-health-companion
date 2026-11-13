# Design references and the colour code

**The UI look is not final.** This folder is where the team drops what it should look like, and where the one shared colour code lives. Every face (web, safe mode, mobile, ground) follows it.

Owner: humans / PLANNER. Agents read this folder; they change it only when a human asks.

## 1. Dropping references

Put files in `design/references/`. Any of: **PDF**, **HTML** (single self-contained file), **PNG / JPG / SVG**.

Name them so an agent can find the right one without asking:

```
<face>-<screen>-<state>-v<n>.<ext>

crew-body-flagged-v1.png        A1 body view with an active flag
crew-guidance-v2.pdf            A7 guidance screen
ground-inbox-v1.html
all-palette-v1.png              applies to every face
all-moodboard-v1.pdf
```

`face`: `crew`, `ground`, `safe`, `mobile`, or `all`. `screen`: the names in `docs/reference/ui_faces_and_workflows.md` (body, region, measure, guidance, today, crew, comms, trends, ...).
Newer version wins; delete or keep old ones, your choice. Keep each file under about 5 MB.

Then add one line to the table below so agents know what is binding:

| File | Applies to | Binding? | Notes |
|---|---|---|---|
| _none yet_ | | | |

"Binding" = match it closely. Otherwise it is inspiration: follow the mood, layout is free.

## 2. The colour code: `design/tokens.css`

One file holds every colour, plus radius, spacing and font tokens. It is **provisional** until a human replaces the values from the references. Names stay stable; only values change. That is the point: re-skinning the whole product is an edit to this one file.

- **Web** imports `design/tokens.css` directly and uses `var(--...)` everywhere.
- **Safe mode (egui)** and **mobile (Flutter)** cannot import CSS: each keeps **one** file (`theme.rs`, `theme.dart`) that mirrors these tokens with the same names, with a comment pointing here. When `tokens.css` changes, the PR that changes it says so, and those owners update their mirror.

## 3. Rules for agents building UI

1. Before any UI work, look in `design/references/` for your face and screen, and read the table above. If a reference exists, follow it. Read PDFs and images directly; you can see them.
2. **Never write a raw colour** (`#hex`, `rgb()`, named colours) in a component, widget or screen. Use a token. Need a colour that does not exist? Ask in `QUESTIONS.md`; do not invent one.
3. Status colours are fixed in meaning: nominal, watch, caution, urgent, unknown. Always colour **+ icon + word**. Never reuse a status colour for decoration.
4. No reference for your screen yet? Build structure and behaviour with the tokens and keep the styling thin. Do not spend hours on a look that may be replaced; do not block waiting for a design either.
5. When a new reference lands that contradicts what you built, that is a normal `fix(<area>): match <reference file>` PR, not a failure.
