# PITCH: `pitch/`

Everything judges see that is not the product itself. Owners: **@Tu2525** and **@m7mdnasrr**. Tasks: `docs/plan.md`, section PITCH.

**You own:** `pitch/`. Nothing here blocks the product, and the product owners do not edit here.

## Deliverables

| What | Where | Notes |
|---|---|---|
| **Travel case** | `pitch/travel-case/` | Scope to be confirmed by @Tu2525 (see `QUESTIONS.md` if you are an agent picking this up) |
| **PowerPoint deck** | `pitch/deck/` | The main presentation, `.pptx` plus an exported `.pdf` |
| **Brag video** | `pitch/brag/` | Short launch video of the product (the `/brag` skill can build it from the web app) |
| **PDFs** | `pitch/pdf/` | One-pager, project summary, anything attached to the submission |
| **Presentations** | `pitch/deck/` | Any extra or shortened versions for live judging |

## Rules

- **Same honesty as the product.** Health data is simulated: say so on any slide or frame that shows values. Never call it NASA data. No medical claims; this is not a medical device.
- **Numbers and statuses come from the real app**, on the demo seed (Mars transit, day 143). Do not invent screenshots or values: capture them from the deployed link, so the deck and the demo agree.
- **Follow the colour code**: `design/tokens.css` and the references in `design/references/`. Status is always colour + icon + word.
- **Tell the demo story** in `docs/architecture.md` section 5, in that order. If a feature is not in the story, it does not need a slide.
- **Map to the brief**: `docs/challenge.md` lists what the challenge asks and where we answer it. The deck should make that mapping obvious.
- **Disclose AI use**, both AI used to build the project and AI inside it, and list sources (`docs/sources.md` once it exists).
- Keep files small: under about 10 MB each. Large video goes to a link (unlisted upload), with only the link and the source project committed.
- File names: `<what>-v<n>.<ext>`, for example `deck-v2.pptx`. Newer version wins.
- PRs here follow the same rules as everywhere else: one deliverable per PR, described so the reviewer knows what to open.
