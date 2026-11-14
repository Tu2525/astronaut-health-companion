# Crew home v4 (3D body): expected outcomes

Reference for `crew-home-proposal-v4-3d.html`. This is a demo mock: all data is simulated, status comes from a hard-coded snapshot that stands in for the engine output. The model is `assets/body.glb` (5.9 MB, about 287k polygons). Credit is in `assets/CREDITS.md`; it is not shown on the page.

Run: `npx serve design -l 8765` (or the `.claude/launch.json` entry), open `/references/crew-home-proposal-v4-3d.html`.

## What you should see

### On load
- "Loading body model" shows, then fades as an X-ray body appears: faint skin and muscle, organs inside.
- The body slowly turns on its own (not with reduced motion).
- Beside the model, the **Body systems** legend lists all 10 systems. Each row shows the name, what is off (a short reading) and a status chip with icon and word. On phones the legend sits under the model.
- Only flagged organs take full colour and pulse. Nominal organs stay near-silver (calm by default).

### Interaction
| Do | Expect |
|---|---|
| Drag the body | It turns, with momentum on release. Auto-turn stops. The hint fades. |
| Tap an organ on the model | It becomes the selected region: the camera flies in, the rest of the body steps back, the organ shines. The right panel shows its detail. |
| Tap a legend row, pin or callout | Same as tapping the organ. |
| Tap the selected organ again | The camera re-zooms to it. |
| Tap empty space or "Whole body" | The camera returns to the full body. |
| Tap a callout with guidance | The guidance sheet opens. Esc or the scrim closes it. |
| Hover an organ (mouse) | It lifts slightly, the cursor becomes a pointer. |

### Mock states (bottom bar)
| State | Headline | Model | Legend |
|---|---|---|---|
| Flagged | Two things worth a look | Heart amber (caution) and head blue (watch), both pulse, with callouts | Heart: Caution, Mind: Watch, Eyes: Unknown |
| Nominal | All clear | All near-silver, no callouts | All Nominal |
| Cold start | Learning your normal | Dashed unknown pin, "Baseline in progress, day 3 of 7" | Unknown where no baseline |
| Stale | Some readings are old | Heart unknown, "Heart data is 26 hours old" | Stale rows read Unknown |

### Hazard lenses
Radiation, Isolation and confinement, Distance from Earth, Gravity fields, Closed environment. Pressing one dims every system it does not affect, on the model and in the legend. Press again to clear.

### Rules the page keeps
- Status is never colour alone: chips carry an icon and a word, the legend is the keyboard and screen-reader path.
- "Simulated data" badge stays visible. No medication doses.
- Reduced motion: no auto-turn, no pulse, camera moves jump instead of flying.
- No WebGL or model load failure: the page falls back to a static figure (the mock uses a local photo that is not committed; the apps should fall back to the legend alone).
- 375 px: no sideways scroll, callouts hidden (pins and legend carry the info).

## Apple-design review (done)
Holds up: camera moves are critically damped and start from the current position, so they can be interrupted; drag carries momentum; tap vs drag uses hysteresis; reduced motion and higher contrast are handled; heading tracking is size-specific.

Gaps found and fixed:
1. An organ lights up on pointer-down; selection still commits on release, and dragging away cancels it.
2. Tilting past the comfortable angle is allowed a little, then springs back on release (rubber-band) instead of a hard stop.
3. "Whole body" fades and scales in from its corner, and out the same way.
4. Callouts grow out toward their pin, and only when they change (not on every tap).
5. `prefers-reduced-transparency` makes the blurred bars, sheet, callouts and buttons solid.
6. Pulse stays at about 0.4 Hz. Do not slow it toward 0.2 Hz (vestibular discomfort).

Also fixed: the body turns around its spine, not its bounding box, so it stays centred while turning.

### Icons
Custom inline SVG line icons, 24 grid, `currentColor`: one per system (brain, eye, moon, lungs, heart with pulse, gut coil, shield, skin layers, muscle, bone) and one per status with a distinct shape (check circle, half circle, triangle, octagon, dashed circle). Flagged rows tint their icon tile in the status colour; nominal rows stay grey.

## Not yet done (more UI we could add)
- Organ detail drill-down: a trend sparkline for the selected organ, inside the 3D view.
- Compare to baseline: ghost the body at the last nominal state.
- Crew view: small bodies for each crew member, tap to open theirs.
- Time scrub: drag through the last 7 days and watch organs change status.
- Blackout / comms-loss state and error state (not in the mock bar yet).
- Bundle three.js and the model offline (CDN for the mock only).

## Handoff: building this in React (web) and React Native (mobile)

This page is the reference. Match what it shows; do not copy its mock logic.

1. **Owners.** React goes in `apps/web` (WEB). React Native goes in `apps/mobile` (MOBILE). Each agent edits only its own folder (root `AGENTS.md`).
2. **Status comes from the engine.** The `STATES` object in the page is a stand-in for engine output: per-region status, flags with text and next step, meta lines, lens membership. Ask ENGINE for that shape with a `contract:` issue if it is missing. Until it lands, mock the data, not the logic. No thresholds or status rules in TypeScript.
3. **Mesh names are the keys.** `assets/body.glb` holds 10 meshes named `head eyes mind lungs heart gut immune skin muscles bones` (Blender may suffix `.001`; strip it). These must match the engine's region IDs.
4. **Bundle offline.** Ship three.js and `body.glb` inside each app; no CDN at runtime. Copy `assets/body.glb` and keep the credit from `assets/CREDITS.md` in an About or Licences screen.
5. **Web 3D.** `three` plus `@react-three/fiber` and `@react-three/drei` (`useGLTF`, `OrbitControls`). Port the X-ray shader, the critically damped camera focus, the press highlight, the rubber-band tilt and the height-only camera fit as they are.
6. **Mobile 3D is the risk.** Use `@react-three/fiber/native` with `expo-gl` and test on a real phone early (287k polygons). If it stutters, decimate the GLB in Blender or ship the legend alone. The legend already carries all the information, so the screen works without the model.
7. **Legend is required, not optional.** It is the keyboard, screen-reader and non-colour path: icon, name, status chip (shape + word), and what is off.
8. **Icons.** The 10 system icons and 5 status icons are inline SVG in the page (`ICON`, `GLYPH`). Lift them as components (`react-native-svg` on mobile). Status icons have different shapes on purpose.
9. **Colours.** Only tokens from `design/tokens.css`, or each app's single mirror file. The mock still uses a few raw values (`#9aa3ad` neutral organ, `#15181c` row divider, `#000` mixes); add tokens for them, do not copy the hex.
10. **Accessibility and motion.** Keep reduced motion (no auto-turn, no pulse, jump instead of fly), reduced transparency (solid surfaces), 44 px touch targets, and status as colour + icon + word.
11. **PR screenshots.** Desktop and about 390 px, in Flagged, Nominal, Cold start and Stale, next to this page (root `AGENTS.md`, "Screenshots in PRs").
