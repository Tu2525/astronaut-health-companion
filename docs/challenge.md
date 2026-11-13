# The challenge (summary)

Source: https://www.spaceappschallenge.org/2026/challenges/health-monitoring-software/?tab=details
(Read on 2026-11-13. Check the Resources tab too; more may be added.)

**Create Health Monitoring Software for Astronauts on Space Missions.** Difficulty: intermediate. Subjects: human exploration, software, space exploration.

Build a **visual tool** that gathers health indicators and lets astronauts **assess their own health and act on it**, with guidance on treatments and improvements.

What the brief names, and where we answer it:

| The brief says | Our answer |
|---|---|
| Five HRP hazards: radiation, isolation and confinement, distance from Earth, gravity fields, hostile closed environments | Hazard lens on the body view; `hazards` on every indicator |
| Indicators such as blood pressure, heart rate, blood sugar, temperature, sleep hours, mood | `crates/core/registry.json` |
| Any input: keyboard, mouse, camera | Guided keypad entry, camera pulse on mobile |
| Which measurements can be done in space? Why is an exam needed? | Guided measurement cards (`how_to_measure`, `why_in_space`) |
| How do hazards affect the indicators? | Region detail + library, with sources |
| When does a result need the crew or flight surgeons? | The four-step escalation ladder and the delayed ground loop |
| What actions prevent a critical condition? | Guidance screen, actions filtered by the kit |
| A clickable human figure showing function and status | Body view (the hero screen) |
| Different environments (spacecraft, station, rover, base) | `Environment` on every reading, per-environment ranges |
| Prompts as part of the mission routine | Today screen and prompt scheduler |
| Gamification | Crew screen, activity-only scoring |
| Dynamic charts over time | Indicator detail and trends |
| Colour coding at a glance | Status vocabulary: colour + icon + word |
| Do not assume astronauts know how to assess their health | Plain-language text everywhere, "Why?" on every number |
