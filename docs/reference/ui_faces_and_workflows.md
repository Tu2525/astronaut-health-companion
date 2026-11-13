# UI Faces, Screens & Workflows: Astronaut Health Companion

NASA Space Apps 2026 · *Create Health Monitoring Software for Astronauts on Space Missions*

> A base for the first design pass: which faces exist, what screens each has, and how users move through them.
> No build plan. Companion to `stack-and-architecture-ideas.md` and `space-apps-2026-winning-playbook.md`.

---

## 1. The faces (who uses what)

| Face | User | Runs on | Purpose |
|---|---|---|---|
| **A. Crew App** (hero) | Astronaut | Web link · Tauri desktop | Self-assessment, guidance, trends, routine, gamification |
| **B. Safe-Mode Console** | Astronaut | egui native · WASM link | Minimal fallback when the rich layer fails |
| **C. Ground Station** | Flight surgeon / ground team | Web | Receive reports, review trends, send delayed notes |
| **D. Mobile Companion** | Astronaut | Android (Compose) | Camera pulse, wearable sync, prompts on the go |
| **E. Earth Mode** | Remote clinic / community health worker | Same as A, different profile | Spin-off: same flows, Earth kit and connectivity |

**Earth Mode is a profile, not a separate app:** same screens, different environment, kit and wording.

---

## 2. Shared design language (all faces)

### Status vocabulary (never color alone: color + icon + word)

| Level | Color | Icon | Meaning |
|---|---|---|---|
| **Nominal** | Green | ● filled circle | Within your normal |
| **Watch** | Blue | ◐ half circle | Drifting from baseline, recheck soon |
| **Caution** | Amber | ▲ triangle | Out of range, act now (self-care) |
| **Urgent** | Red | ◆ diamond | Act immediately, follow urgent protocol |
| **Unknown** | Grey | ○ outline | No recent data or low confidence |

### Escalation ladder (the same four steps everywhere)

1. **Monitor:** keep routine, recheck later
2. **Self-care:** do the recommended action from the kit
3. **Consult ground:** send a report (shows expected reply time)
4. **Urgent:** follow the onboard protocol now, don't wait for Earth

The ladder **changes with distance**: on a station "Consult ground" is quick; on a Mars transit the app says when waiting is unsafe.

### Persistent top bar (Crew App, Safe Mode, Mobile)

`Mission day 143 · MET 143d 06:12 · Earth 13:40 UTC · [Mars Transit] · Comms: ⏱ 18 min one-way · ● Overall: Watch · 🔔 2`

- **Environment badge:** station / transit / lunar base / Mars base / Earth clinic
- **Comms indicator:** connected / delayed (with minutes) / blackout
- **Overall status** and **alert inbox count**

### Principles

- **Dark cockpit:** calm by default. Only what needs attention is bright.
- **Glanceable first, details on tap** (progressive disclosure).
- **Every number has a "why"** (an ⓘ opening explanation + source).
- **Large touch targets and voice input** for gloves, motion and low light.
- **Show the data's honesty:** confidence, last-measured time and source status (sourced / adapted / placeholder).

---

## 3. Face A: Crew App (hero)

### Navigation

```
Home (Body) ── Today ── Trends ── Measure ── Crew ── More
                                                  └ Kit · Ground · Library · Settings
```

### A1. Home: Body View *(the hero screen)*
- **Interactive body figure** (2D/SVG) with regions colored by status: head/brain, eyes, heart/circulation, lungs, bones, muscles, gut/nutrition, immune, skin, mind/mood.
- **Hazard lens toggle** (5 chips): radiation · isolation and confinement · distance from Earth · gravity fields · closed environment. Selecting one highlights the systems it stresses and dims the rest.
- **Top flag card:** the 1–2 most important things right now, each with a next step ("Recheck BP in 2 h").
- **Quick actions:** Measure · Check-in · Ask · Send to ground.
- **States:** all nominal (calm, minimal) · active flags · cold start ("building your baseline: day 3 of 7") · stale data (grey regions, "last measured 26 h ago").

### A2. Body Region Detail *(opens from tapping a region)*
- What this system does (one paragraph, plain language).
- **How spaceflight affects it** (with hazard tags + source chips).
- Indicators for this region: current value, status, mini trend, last measured.
- **Expected in-flight change** note ("Some fluid shift to the head is normal in microgravity").
- Buttons: **Measure now** · **See trend** · **What should I do?**

### A3. Indicator Detail
- Big current value + unit + status word.
- Trend chart with **personal baseline band** and **preflight line**, mission events annotated (EVA, schedule shift).
- "What this means" (engine result) → "Why" (explainer) → **"What to do"** (escalation step + actions).
- Data honesty: device, confidence, source status of the range.
- History list of readings.

### A4. Today (Mission Routine)
- Timeline of today's health tasks, scheduled like mission tasks: morning vitals, exercise block, mood check-in, sleep log.
- **Extra prompts** (from drifting values) shown as **bonus tasks**, never as missed obligations.
- Readiness ring + streak + today's quest.
- Each task opens the relevant measurement or check-in flow.

### A5. Measure (Guided Measurement)
Picker → guide → input → result. One card per indicator:
1. **Prepare:** what you need (cuff, glucometer, thermometer), kit location.
2. **Steps:** illustrated, microgravity notes where relevant ("Anchor your arm at heart level").
3. **Why we ask:** one line + source.
4. **Input:** keypad, voice, connected device or camera (pulse estimate with a confidence meter).
5. **Result:** status + what it means + next step (feeds A7 if not nominal).

### A6. Check-in (Behavioral)
- Mood, stress, energy, sleep quality (sliders or faces), optional free text or voice.
- Optional **buddy check** (opt-in, support-framed).
- Ends with a gentle summary and, if needed, a suggestion (talk to crewmate, rest, schedule private ground call).

### A7. Guidance / Action Plan *(the flag-to-action screen)*
- Situation in one sentence ("Heart rate 14% above your baseline for 2 days, with sleep debt").
- **Escalation ladder** with the current step highlighted, and what moves you up or down.
- **Recommended actions** from the registry, **filtered by kit**; unavailable items show the fallback.
- **Time to help panel:** round-trip delay, next comms window, whether waiting is safe.
- Buttons: **Start action** · **Set recheck** · **Send to ground** · **Ask why**.

### A8. Ask (Grounded Explainer)
- Chat or voice. Answers show **source chips**, and always show the engine's status above the text.
- "Not covered → contact flight surgeon" fallback.
- Badge: "Onboard assistant · works offline" and a clear off switch.

### A9. Trends
- Multi-indicator view (choose 2–4) on one mission timeline.
- Range: 24 h · 7 d · mission. Overlays: baseline band, preflight line, events, environment changes.
- Plain-language summary above each chart.

### A10. Crew (Gamification)
- **My progress:** XP, level, streak, badges, weekly quest.
- **Crew score** and cooperative goal ("all 4 hit exercise target").
- **Leaderboard:** crew vs crew (Moon base vs Mars transit vs station), weekly season.
- Privacy label visible: "Leaderboards show activity, never health values."
- Early-report bonus called out ("+30 XP for reporting a symptom early").

### A11. Kit (Medical Inventory)
- Items with quantity, location, expiry. Low-stock warning.
- Using an item from A7 decrements it.
- Setup: pick a preset kit per environment.

### A12. Ground (Comms)
- Outbox: queued reports (SBAR), each with send time and **expected arrival** countdown.
- Inbox: flight surgeon notes, plan changes, threshold changes (with "arrived X min after your action").
- Blackout state: "Reports will send when the window opens at 15:20 MET."

### A13. Library
- Explainers for each indicator, body system and hazard, all sourced. Doubles as the evidence layer for judges.

### A14. Settings & Profile
- Environment selector (shows the effect: ranges, delay, ladder).
- Preflight baseline, units, accessibility (text size, high contrast), AI on/off, privacy choices, Earth Mode profile.

### A0. Onboarding (first run)
1. Choose profile: crew member · Earth clinic
2. Mission and environment
3. Preflight baseline (enter or import)
4. Kit preset
5. Routine schedule
6. Privacy and leaderboard opt-in
7. Short tour of the body view and the ladder

---

## 4. Face B: Safe-Mode Console (egui)

Deliberately minimal: one screen plus two panels. Keyboard-first, high contrast.

- **B1. Status board:** indicator table (name · value · status · last measured), overall status, active flags with escalation step, comms status.
- **B2. Quick entry:** log a reading or symptom fast (keyboard).
- **B3. Protocol card:** the current action steps as plain text.
- **Banner:** "SAFE MODE: rich interface unavailable. Same engine, same results."
- Entry point: auto-switch when the main UI fails, or a manual "Enter safe mode" for the demo.

---

## 5. Face C: Ground Station (Flight Surgeon)

### Navigation

```
Crew Overview ── Reports Inbox ── Crew Member ── Notes Outbox ── Settings
```

- **C1. Crew Overview:** one card per crew member (or per crew/mission): overall status, top flag, last report, comms delay. Sorted by severity.
- **C2. Reports Inbox:** incoming SBAR reports, each showing **sent (MET) · received (Earth) · delay**. Unread and urgent first.
- **C3. Crew Member Detail:** full trends, readings (values visible here), flags history, actions taken, kit use, check-ins.
- **C4. Report Detail (SBAR):** Situation · Background · Assessment (engine status + sources) · Recommendation, plus the readings that triggered it. Actions: acknowledge, reply, adjust plan, change a threshold for this crew member.
- **C5. Compose Note:** reply or plan change, shows **expected arrival at crew**.
- **C6. Event timeline:** merged crew and ground events, showing conflicts ("crew acted before note arrived").
- **Settings:** access control, simulated delay controls (demo), environment per crew.

---

## 6. Face D: Mobile Companion (Compose)

- **D1. Home:** compact status + top flag + today's next task.
- **D2. Camera pulse:** fingertip guide, live signal, result with confidence, "confirm with a cuff" if low.
- **D3. Wearable sync:** Health Connect permissions, last sync, imported heart rate, sleep and activity.
- **D4. Quick check-in:** mood and symptoms in 3 taps or by voice.
- **D5. Notifications:** measurement prompts, ground note arrived, streak reminders (never nagging about bad values).
- Optional: body view in a WebView.

---

## 7. Workflows

### W1. First run
`Onboarding (A0) → Body view in cold-start state → Today shows first routine → first guided measurement`

### W2. Daily routine loop
```
Prompt (Today / notification)
  → Guided measurement (A5) or Check-in (A6)
  → Engine updates status
  → Body view refreshes
  → XP / streak / crew score update (activity only)
```

### W3. Flag-to-action (the core loop)
```
Reading out of baseline or range
  → Flag appears on Body view (top flag card) + region colored
  → Tap → Guidance (A7): situation, ladder step, actions filtered by kit
  → Astronaut picks action → kit decrements → recheck scheduled
  → Recheck (A5)
      ├─ back to normal → flag resolves, ladder steps down, early-report bonus
      └─ worse / persistent → ladder steps up → W4
```

### W4. Consult ground (delayed)
```
Guidance → "Send to ground"
  → SBAR auto-drafted (explainer drafts, engine data fixed) → astronaut reviews → queued
  → Ground (A12) shows countdown to arrival
  → Ground Station: Reports Inbox (C2) → Report Detail (C4) → reply / plan change (C5)
  → Note arrives at crew after delay → Inbox (A12) → plan applied as an event
  → If waiting is unsafe: app keeps showing onboard Urgent protocol meanwhile
```

### W5. Low-confidence reading
```
Camera pulse or suspicious sensor value
  → Status "Unknown / low confidence" (grey)
  → Prompt: confirm with proper device
  → Confirmed reading → normal W3 path
```

### W6. Understand something
```
Any ⓘ or "Why?" → explainer card with sources
  → "Ask more" → Ask (A8) → answer with source chips
  → Not covered → "Contact flight surgeon" → W4
```

### W7. Behavioral check-in
```
Daily prompt → Check-in (A6) → mood/stress trend updates
  → Pattern detected (with sleep and workload) → Watch flag on "mind" region
  → Suggestions + option for private ground call (W4)
```

### W8. Environment switch
```
Settings → change environment (e.g., Station → Mars transit)
  → Preview of what changes: ranges, delay, ladder wording, kit preset
  → Confirm → new readings judged by new ranges; history keeps its original environment
```

### W9. Safe mode fallback
```
Rich UI fails (or demo toggle)
  → Safe-Mode Console (B) with same engine state
  → Quick entry still logs readings → events sync back when rich UI returns
```

### W10. Gamification loop
```
Completed tasks / honest reports / follow-ups → XP, streak, badges
  → Crew score → weekly leaderboard
  → Illness or rest → streak freeze, no penalty
```

### W11. Earth Mode
Same as W1–W10 with Earth clinic kit, real-world connectivity in place of space delay, and wording adapted for community health workers.

---

## 8. Screen inventory checklist (for the design pass)

**Crew App:** A0 Onboarding · A1 Body · A2 Region · A3 Indicator · A4 Today · A5 Measure · A6 Check-in · A7 Guidance · A8 Ask · A9 Trends · A10 Crew · A11 Kit · A12 Ground · A13 Library · A14 Settings

**Safe Mode:** B1 Status board · B2 Quick entry · B3 Protocol card

**Ground Station:** C1 Overview · C2 Inbox · C3 Crew member · C4 Report · C5 Compose note · C6 Timeline

**Mobile:** D1 Home · D2 Camera pulse · D3 Wearable sync · D4 Quick check-in · D5 Notifications

**States to design for each key screen:** nominal · flagged · urgent · cold start · stale data · low confidence · offline/blackout · AI off

**Demo-critical screens (design these first):** A1, A2, A5, A7, A10, A12, B1, C2/C4
