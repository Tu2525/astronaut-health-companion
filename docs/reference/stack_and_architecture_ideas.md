# Stack & Architecture Ideas: Astronaut Health Companion

NASA Space Apps 2026 · *Create Health Monitoring Software for Astronauts on Space Missions*

> Ideas and decisions only. This is **not** a build plan: no tasks, schedule or implementation steps.
> Companion to `space-apps-2026-winning-playbook.md`.

---

## 1. Stack at a glance

| Piece | Choice | Role in the story |
|---|---|---|
| **Hero experience** | Web UI (Svelte or React) | Body figure, hazard lens, trends, guided measurements, explainer. The link judges click. |
| **Desktop shell** | Tauri (webview) | Same web UI as a Linux/Windows app for the "onboard" setting |
| **Safe-mode console** | egui (native + WASM build) | Minimal, always-on fallback that keeps working if the rich layer fails |
| **Doctor dashboard** | Same web UI, "flight surgeon" role | SBAR reports, trends, crew overview, notes sent back with delay |
| **Mobile** | Jetpack Compose (Android) | Only for things a phone does best: camera pulse, Health Connect, notifications. Can embed the shared body view. |
| **Backend** | Normal backend on the Oracle VM, framed as **"Ground Station / Earth"** | Receives store-and-forward reports with a simulated delay. Not required for the astronaut side to work. |
| **Assessment engine** | One shared Rust crate | Native on desktop, WASM in the web UI, also in the backend. **Single source of truth.** |
| **Local AI** | Optional, grounded explainer | Explains and drafts; never decides |

**Not doing:** iOS (cut), GPUI as the main UI (see section 6).

---

## 2. Core idea: one UI, one engine

- **Build the body once.** The web UI serves the hero demo, the desktop app, the doctor dashboard (as a role) and the mobile body view.
- **Compute status once.** If each client calculates status itself, they'll disagree (yellow on the phone, green on the dashboard), and it will happen in the demo. One engine avoids that.
- **Offline-first astronaut side.** The ground backend is a feature, not a dependency.

---

## 3. The spine every feature reads from

### Indicator registry (rules as data)

One record per indicator:

- name, unit, allowed units with conversions (mg/dL ↔ mmol/L, °C ↔ °F)
- body region
- related NASA hazards (radiation, isolation and confinement, distance from Earth, gravity fields, hostile closed environments)
- how to measure it in space (guided-card content)
- **expected in-flight change** (adaptation is not an alarm)
- ranges per environment (station, transit, lunar base, Mars base)
- source citation + source status: **sourced**, **adapted** or **placeholder**
- recommended actions + kit items each action needs
- escalation level

The registry makes the science visible to judges, and thresholds can change without code.

### Append-only event log

Every reading, symptom, check-in, kit use, flag, ground message and override is an event, with:

value · unit · source device · confidence · environment · mission time · Earth time · author

---

## 4. How the features connect

```
Inputs: manual entry · guided cards · camera pulse · Health Connect · simulated dosimeter · cabin sensors
   ↓
Observation events → local event log
   ↓
Assessment engine (registry + personal baseline + preflight baseline + environment + confidence)
   ↓
Status per indicator and body region · ranked flags · escalation level
   ├─ Body figure + hazard lens        (status + registry hazards)
   ├─ Trends                           (log + baseline band + mission annotations)
   ├─ Prompt scheduler                 (drifting flags → more frequent measurements)
   ├─ Action planner                   (registry actions ∩ kit inventory)
   ├─ Grounded explainer (local AI)    (status + registry sources; cannot change status)
   ├─ SBAR generator → sync queue (simulated delay) → Ground backend → Doctor dashboard
   ├─ Gamification                     (reads activity events ONLY, never values)
   └─ Safe-mode console (egui)         (same engine, minimal view)
   ↑
Ground notes and plan changes return as events, also delayed
```

### Guardrails enforced by the architecture

- **Gamification can't see health values.** It only receives events like "measurement completed", "symptom reported" or "follow-up done", so it *can't* rank health outcomes.
- **The AI can't change status.** It receives the engine's result and fills a constrained explanation.
- **Views by role:**

| Data | Astronaut (self) | Crewmates | Leaderboard | Flight surgeon |
|---|---|---|---|---|
| Own readings and mood | ✅ | ❌ | ❌ | ✅ |
| Flags and escalation | ✅ | Opt-in | ❌ | ✅ |
| Scores, streaks, badges | ✅ | ✅ | ✅ | ✅ |
| Buddy-check notes | Decide | Author only | ❌ | ✅ |

---

## 5. Caveats where features collide

**Baselines and data**
- **Cold start.** Personal baselines need history. Fall back to population ranges until enough data exists, and seed the demo with a realistic mission history.
- **Adapting baselines can hide decline.** Slow bone or cardiovascular loss becomes "normal". Anchor to a preflight baseline plus sourced hard limits, and flag drift from preflight separately.
- **Expected adaptations.** Some in-flight changes are normal. Use the registry's "expected change" field to avoid false alarms.

**Inputs and alerts**
- **Low-confidence inputs** (camera pulse, suspected sensor fault) never flag on their own. They trigger a "confirm with a proper device" prompt.
- **Alarm fatigue.** Many indicators plus compound rules can give five yellows at once. Rank flags and surface the top one or two with a clear next step.
- **Environment switching.** Store the environment on each reading, or switching mode rewrites history.

**Actions and kit**
- **Kit state.** Using an item decrements the inventory, so an action can become unavailable. The fallback is "contact ground", which ties back to the delay.
- **No medication doses.** Point to the kit protocol card and the flight surgeon.

**Sync and time**
- **Conflicts.** Ground may change a plan while the crew is offline. Append-only, authored events make this explainable ("ground note arrived 40 min after your action"). Show it rather than hide it.
- **Two clocks.** Mission elapsed time and Earth time differ, and sleep schedules shift. Streak and sleep-debt logic use mission time.
- **Delay in both directions,** or the distance story breaks.

**Behavioral health**
- **Buddy checks can hurt crew trust.** NASA lists teamwork as a risk. Make them opt-in and support-framed, and decide visibility deliberately.

**Gamification**
- **Extra prompts** from drifting values are bonus opportunities, never streak-breakers.

**AI**
- **Contradiction risk.** The model's text could contradict the engine. Show engine status first and constrain the wording.
- **Disclose AI** used inside the product as well as AI used to build it.

**Data honesty**
- Individual astronaut vitals are largely not public, and much of NASA's Open Science Data Repository is omics and animal studies. Simulated vitals are likely. Use NASA material to shape the **direction and size** of simulated effects and cite it. **Never call simulated data NASA data.** Verify what OSDR actually offers before writing the "NASA data used" section.
- **Radiation:** there's no personal sensor, so use a simulated dosimeter. Take limits from a cited NASA standard or label them placeholders.

**Hosting**
- The hero demo must work as a **static, offline-capable build**, with the backend optional.
- The VM must stay up until the January 2027 winner announcements.
- Put basic auth on the doctor view, even with fake data.

---

## 6. Desktop UI decision: why egui for safe mode, not GPUI

**Safe mode as a design feature.** Spacecraft are designed for redundancy and graceful degradation. The rich web UI is normal mode. A native console keeps critical status, active flags, the escalation step, comms delay and reading entry working if the webview, AI or sync layer fails. Both use the same engine.

> *"If the rich interface fails, the crew still has a native health console."*

| | egui | GPUI | Tauri webview |
|---|---|---|---|
| Judges can open it in a browser | ✅ (WASM) | ❌ | ✅ (as the web app) |
| Rich body figure and animation | Hard | Hard | Easy |
| Dense native status console | Easy | Good | Fine |
| Story value | Same console native and in browser | Maintainer credibility | Shared UI everywhere |

- **egui wins for safe mode.** It's mature on Linux and Windows, has few dependencies, and the **WASM build lets judges click the degraded mode** instead of watching a video.
- **egui caveats:** it looks like a developer tool by default (style it deliberately); custom visuals are harder (keep the body on the web); accessibility is weaker than the web's.
- **GPUI** only if maintainer credibility matters more than judges being able to try it.
- **Don't** build the main experience in GPUI with an embedded webview: two rendering stacks, a seam for input and communication, and mismatched styling, with no benefit judges see.
- **Webview reality:** Tauri uses WebView2 on Windows and WebKitGTK on Linux, so the same page can behave differently. WebKitGTK is weaker with WebGL and heavy animation, so prefer an **SVG/2D body** over a 3D one.

---

## 7. Local AI: role and limits

**Use it for:**
- plain-language explanations of readings and why spaceflight causes them
- natural-language and voice logging into structured events (speech-to-text can also be local)
- drafting the SBAR handoff from structured data
- the "ask onboard now vs. ask Earth with delay" story

**Never use it for:**
- thresholds, status or urgency (the deterministic, sourced engine decides)
- free-form medical advice or doses
- opaque anomaly detection (baseline bands and simple statistics are more explainable)

**Make it defensible:**
- answers only from the evidence layer, with source chips; says "not covered, contact flight surgeon" otherwise
- **the app works fully with the AI off**; a stalled model must never break the demo
- disclosed in the AI use section

---

## 8. Mobile: earn its place

Judges won't install an APK, so mobile exists only for:
- **camera pulse estimate** (labeled as an estimate, with confidence)
- **Health Connect** as a stand-in for crew wearables (heart rate, sleep, activity)
- measurement **prompts and notifications**
- optionally the shared body view in a WebView

If none of these make the demo, cut mobile.

---

## 9. The scenario that proves the connections

One story that touches every module:

1. **Mars transit, day 143.** Sleep debt builds; mood check-in dips.
2. Heart rate creeps above baseline → the **prompt scheduler** asks for a blood pressure check.
3. A **guided measurement card** walks the astronaut through it.
4. The **engine** flags yellow; the **body figure** highlights the cardiovascular region; the **hazard lens** shows isolation and the closed environment as contributors.
5. The **explainer** gives a plain-language reason with source chips.
6. The **action planner** suggests rest, hydration and a recheck, using what's in the kit.
7. An **SBAR report** is queued and reaches **ground** 20 minutes later; the **flight surgeon's note** returns, also delayed.
8. **Comms blackout:** the web layer is "lost", and the **egui safe mode** still shows status and accepts the recheck.
9. The crew earns an **early-report bonus**; the **crew leaderboard** ticks up without showing any values.

**Rule:** if a feature doesn't appear in this story, it's a candidate to drop.
