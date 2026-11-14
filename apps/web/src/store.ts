import type { Environment } from "./contract/Environment";
import type { Event } from "./contract/Event";
import { createClock, MISSION_START_KEY, missionStart } from "./clock";
import { assess, demo, registry } from "./engine";

const EVENTS_KEY = "ahc.events.v1";
const SETTINGS_KEY = "ahc.settings.v1";

interface Settings {
  crew_id: string;
  environment: Environment;
  one_way_delay_s: number;
  role: "astronaut" | "flight_surgeon";
  ai_enabled: boolean;
  met_offset_s: number;
}

export function createStore() {
  const seed = demo();
  const indicators = registry();
  const defaults: Settings = {
    crew_id: seed.crew_id, environment: "transit", one_way_delay_s: 20,
    role: "astronaut", ai_enabled: false, met_offset_s: 0,
  };
  // Corrupt storage must not brick the offline app: fall back to defaults and the seed.
  const parse = <T>(raw: string | null): T | null => {
    try { return raw === null ? null : (JSON.parse(raw) as T); } catch { return null; }
  };
  const readSettings = (): Settings => ({
    ...defaults, ...parse<Partial<Settings>>(localStorage.getItem(SETTINGS_KEY)),
  });
  const readEvents = (): Event[] => {
    const saved = parse<Event[]>(localStorage.getItem(EVENTS_KEY));
    if (Array.isArray(saved)) return saved;
    localStorage.setItem(EVENTS_KEY, JSON.stringify(seed.events));
    return seed.events;
  };
  let settings = readSettings();
  let events = readEvents();
  let clock = createClock(seed.now_met_s, seed.launch_earth_ms, missionStart());
  const listeners = new Set<() => void>();

  function snapshot() {
    const nowMet = events.reduce((met, event) => Math.max(met, event.met_s), clock.now(settings.met_offset_s));
    return {
      settings, events, nowMet, earthMs: clock.earthMs(nowMet), indicators,
      assessment: assess({
        crew_id: settings.crew_id, events, now_met_s: nowMet, kit: seed.kit, registry: null,
      }),
    };
  }
  let current = snapshot();

  function publish() {
    current = snapshot();
    listeners.forEach((listener) => listener());
  }

  function reload(event: StorageEvent) {
    if (event.storageArea !== localStorage) return;
    if (event.key === EVENTS_KEY) events = readEvents();
    else if (event.key === SETTINGS_KEY) settings = readSettings();
    else if (event.key === MISSION_START_KEY) clock = createClock(seed.now_met_s, seed.launch_earth_ms, missionStart());
    else if (event.key === null) {
      events = readEvents();
      settings = readSettings();
      clock = createClock(seed.now_met_s, seed.launch_earth_ms, missionStart());
    } else return;
    publish();
  }

  window.addEventListener("storage", reload);
  const timer = window.setInterval(publish, 1000);

  return {
    getSnapshot: () => current,
    subscribe(listener: () => void) {
      listeners.add(listener);
      return () => { listeners.delete(listener); };
    },
    append(event: Event) {
      const latest = readEvents();
      if (latest.some(({ id }) => id === event.id)) return;
      const updated = [...latest, event];
      localStorage.setItem(EVENTS_KEY, JSON.stringify(updated));
      events = updated;
      publish();
    },
    updateSettings(patch: Partial<Settings>) {
      settings = { ...readSettings(), ...patch };
      localStorage.setItem(SETTINGS_KEY, JSON.stringify(settings));
      publish();
    },
    skipTime(seconds: number) {
      const saved = readSettings();
      settings = { ...saved, met_offset_s: saved.met_offset_s + seconds };
      localStorage.setItem(SETTINGS_KEY, JSON.stringify(settings));
      publish();
    },
    dispose() {
      window.clearInterval(timer);
      window.removeEventListener("storage", reload);
    },
  };
}

export type CrewStore = ReturnType<typeof createStore>;
