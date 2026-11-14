//! Event log and settings storage (SAFE-3), same JSON shapes on both builds
//! (`docs/contract.md` section 8):
//! - web: `localStorage` keys `ahc.events.v1` and `ahc.settings.v1`, shared with the
//!   web app because `/safe/` is served from the same origin;
//! - native: `events.json` and `settings.json` in the app data dir
//!   (override with `AHC_SAFE_DATA_DIR`).
//!
//! The log is append-only and owned by the crew: this module never edits, deletes or
//! overwrites stored events. Unreadable data is left untouched and reported.

use ahc_core::{demo, Environment, Event};

pub const EVENTS_KEY: &str = "ahc.events.v1";
pub const SETTINGS_KEY: &str = "ahc.settings.v1";
/// Wall-clock ms of the first load, written by whichever face loads first
/// (`apps/web/src/clock.ts`). Shared so both faces show the same mission time.
pub const MISSION_START_KEY: &str = "ahc.mission_start.v1";

/// Demo default from `docs/architecture.md` ("Comms delay"); see `Q-SAFE-2`.
pub const DEMO_ONE_WAY_DELAY_S: f64 = 20.0;

/// The fields of `ahc.settings.v1` that safe mode uses. Missing or unreadable
/// settings fall back to the demo defaults.
#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    pub crew_id: String,
    pub one_way_delay_s: f64,
    pub met_offset_s: f64,
    /// `None` when unset or not a known environment.
    pub environment: Option<Environment>,
}

impl Settings {
    pub fn parse(stored: Option<&str>) -> Self {
        let v: serde_json::Value = stored
            .and_then(|s| serde_json::from_str(s).ok())
            .unwrap_or_default();
        Self {
            crew_id: v["crew_id"].as_str().unwrap_or(demo::CREW_ID).to_string(),
            one_way_delay_s: v["one_way_delay_s"]
                .as_f64()
                .unwrap_or(DEMO_ONE_WAY_DELAY_S),
            met_offset_s: v["met_offset_s"].as_f64().unwrap_or(0.0),
            environment: serde_json::from_value(v["environment"].clone()).ok(),
        }
    }
}

/// What the stored log text means.
#[derive(Debug)]
pub enum Stored {
    /// A log, possibly empty. An empty `[]` is kept as it is, as the web app does.
    Events(Vec<Event>),
    /// No log yet (key missing): seed from the demo.
    Missing,
    /// Present but not an `Event[]`. Never overwritten.
    Unreadable(String),
}

pub fn parse_log(stored: Option<&str>) -> Stored {
    let Some(text) = stored else {
        return Stored::Missing;
    };
    match serde_json::from_str::<Vec<Event>>(text) {
        Ok(events) => Stored::Events(events),
        Err(e) => Stored::Unreadable(e.to_string()),
    }
}

/// Wall-clock time in ms since the Unix epoch (same scale as the web app's clock).
pub fn wall_ms() -> f64 {
    #[cfg(target_arch = "wasm32")]
    {
        js_sys::Date::now()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0.0, |d| d.as_secs_f64() * 1000.0)
    }
}

/// The shared first-load time, written now if no face has loaded yet. Call once at
/// startup; later re-reads use [`stored_mission_start_ms`] so a failed write never
/// restarts the clock.
pub fn mission_start_ms() -> f64 {
    mission_start_with(&backend::Backend::new(), wall_ms())
}

/// The shared first-load time if one is stored (for re-reads after a storage change).
pub fn stored_mission_start_ms() -> Option<f64> {
    stored_mission_start(&backend::Backend::new())
}

fn stored_mission_start(b: &backend::Backend) -> Option<f64> {
    b.read(MISSION_START_KEY)
        .ok()
        .flatten()
        .and_then(|s| s.trim().parse::<f64>().ok())
        .filter(|ms| ms.is_finite())
}

fn mission_start_with(b: &backend::Backend, now_ms: f64) -> f64 {
    stored_mission_start(b).unwrap_or_else(|| {
        // Only written when nothing is stored. If the write fails the clock still runs
        // from this load; the caller keeps the value.
        if matches!(b.read(MISSION_START_KEY), Ok(None)) {
            let _ = b.write(MISSION_START_KEY, &now_ms.to_string());
        }
        now_ms
    })
}

/// `docs/architecture.md` "Clock": demo time + seconds since first load + `met_offset_s`,
/// never earlier than the newest event (so a reading just logged is never in the future).
/// Mirrors `apps/web/src/store.ts` so both faces assess at the same instant.
pub fn now_met_s(start_ms: f64, wall_ms: f64, settings: &Settings, events: &[Event]) -> f64 {
    let clock = demo::NOW_MET_S + (wall_ms - start_ms).max(0.0) / 1000.0 + settings.met_offset_s;
    events.iter().map(|e| e.met_s).fold(clock, f64::max)
}

/// The log as loaded, plus a message for the crew when something went wrong.
pub struct Loaded {
    /// `None` when the stored log could not be read: callers keep what they had.
    pub events: Option<Vec<Event>>,
    pub notice: Option<String>,
}

/// Read the log, seeding it from the demo the first time (as the web app does).
pub fn load_events() -> Loaded {
    load_events_with(&backend::Backend::new())
}

pub fn load_settings() -> Settings {
    let stored = backend::Backend::new().read(SETTINGS_KEY).ok().flatten();
    Settings::parse(stored.as_deref())
}

/// Where the log lives, for the top bar.
pub fn location() -> String {
    backend::Backend::new().describe()
}

/// Appends one event: re-reads the stored log, adds the event and writes the whole
/// array back (`docs/contract.md` section 8), so a write from the other face is never
/// lost. A missing log is seeded first, as the web app does. An unreadable log is
/// never written: the event is refused and the reason returned. Returns the new log.
pub fn append_event(event: Event) -> Result<Vec<Event>, String> {
    append_event_with(&backend::Backend::new(), event)
}

fn append_event_with(b: &backend::Backend, event: Event) -> Result<Vec<Event>, String> {
    let text = b
        .read(EVENTS_KEY)
        .map_err(|e| format!("Not logged: the stored log could not be read ({e})."))?;
    let not_logged = |e: serde_json::Error| format!("Not logged: {e}");
    // The typed parse checks the log; the stored objects themselves are written back
    // as they were, so a field this build does not know is never dropped.
    let (mut events, mut raw) = match parse_log(text.as_deref()) {
        Stored::Events(events) => {
            let raw: Vec<serde_json::Value> =
                serde_json::from_str(text.as_deref().unwrap_or("[]")).map_err(not_logged)?;
            (events, raw)
        }
        Stored::Missing => {
            let events = demo::events();
            let raw = serde_json::to_value(&events)
                .ok()
                .and_then(|v| v.as_array().cloned())
                .unwrap_or_default();
            (events, raw)
        }
        Stored::Unreadable(e) => {
            return Err(format!(
                "Not logged: the stored log ({EVENTS_KEY}) could not be read and was left untouched ({e})."
            ))
        }
    };
    // Same guard as the web app: an id already in the log is not added twice.
    if events.iter().any(|e| e.id == event.id) {
        return Ok(events);
    }
    raw.push(serde_json::to_value(&event).map_err(not_logged)?);
    let json = serde_json::to_string(&raw).map_err(not_logged)?;
    b.write(EVENTS_KEY, &json)
        .map_err(|e| format!("Not logged: {e}"))?;
    events.push(event);
    Ok(events)
}

fn load_events_with(b: &backend::Backend) -> Loaded {
    let text = match b.read(EVENTS_KEY) {
        Ok(text) => text,
        Err(e) => {
            return Loaded {
                events: None,
                notice: Some(format!(
                    "The stored log ({EVENTS_KEY}) could not be read and was left untouched: {e}"
                )),
            }
        }
    };
    match parse_log(text.as_deref()) {
        Stored::Events(events) => Loaded {
            events: Some(events),
            notice: None,
        },
        Stored::Missing => {
            let events = demo::events();
            let notice = serde_json::to_string(&events)
                .map_err(|e| e.to_string())
                .and_then(|json| b.write(EVENTS_KEY, &json))
                .err()
                .map(|e| format!("Demo log shown but not saved: {e}"));
            Loaded {
                events: Some(events),
                notice,
            }
        }
        Stored::Unreadable(e) => Loaded {
            events: None,
            notice: Some(format!(
                "The stored log ({EVENTS_KEY}) could not be read and was left untouched: {e}"
            )),
        },
    }
}

#[cfg(target_arch = "wasm32")]
mod backend {
    pub struct Backend(Option<web_sys::Storage>);

    impl Backend {
        pub fn new() -> Self {
            Self(web_sys::window().and_then(|w| w.local_storage().ok().flatten()))
        }
        /// `Ok(None)` only when the key is absent; any failure to read is an `Err`.
        pub fn read(&self, key: &str) -> Result<Option<String>, String> {
            let storage = self.0.as_ref().ok_or("browser storage is unavailable")?;
            storage
                .get_item(key)
                .map_err(|e| format!("browser storage refused the read: {e:?}"))
        }
        pub fn write(&self, key: &str, value: &str) -> Result<(), String> {
            let storage = self.0.as_ref().ok_or("browser storage is unavailable")?;
            storage
                .set_item(key, value)
                .map_err(|e| format!("browser storage refused the write: {e:?}"))
        }
        pub fn describe(&self) -> String {
            format!(
                "browser storage ({}), shared with the web app",
                super::EVENTS_KEY
            )
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod backend {
    use std::path::PathBuf;

    pub struct Backend(Option<PathBuf>);

    impl Backend {
        pub fn new() -> Self {
            let dir = std::env::var_os("AHC_SAFE_DATA_DIR")
                .map(PathBuf::from)
                .or_else(|| {
                    directories::ProjectDirs::from("", "", "ahc-safe")
                        .map(|d| d.data_dir().to_path_buf())
                });
            Self(dir)
        }
        #[cfg(test)]
        pub fn at(dir: PathBuf) -> Self {
            Self(Some(dir))
        }
        fn path(&self, key: &str) -> Option<PathBuf> {
            let file = match key {
                super::EVENTS_KEY => "events.json",
                super::SETTINGS_KEY => "settings.json",
                super::MISSION_START_KEY => "mission_start.json",
                _ => return None,
            };
            Some(self.0.as_ref()?.join(file))
        }
        /// `Ok(None)` only when the file does not exist; any other failure (permissions,
        /// a sharing violation, invalid UTF-8) is an `Err`, so it is never mistaken for
        /// "no log yet" and seeded over.
        pub fn read(&self, key: &str) -> Result<Option<String>, String> {
            let path = self.path(key).ok_or("no app data directory")?;
            match std::fs::read_to_string(&path) {
                Ok(text) => Ok(Some(text)),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
                Err(e) => Err(format!("{}: {e}", path.display())),
            }
        }
        /// Writes through a temporary file and a rename, so a crash never leaves half a log.
        pub fn write(&self, key: &str, value: &str) -> Result<(), String> {
            let path = self.path(key).ok_or("no app data directory")?;
            let tmp = path.with_extension("json.tmp");
            if let Some(dir) = path.parent() {
                std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
            }
            std::fs::write(&tmp, value).map_err(|e| format!("{}: {e}", tmp.display()))?;
            std::fs::rename(&tmp, &path).map_err(|e| format!("{}: {e}", path.display()))
        }
        pub fn describe(&self) -> String {
            self.path(super::EVENTS_KEY)
                .map_or("not saved (no app data directory)".into(), |p| {
                    p.display().to_string()
                })
        }
    }
}

/// Tells the app when another tab or the web app changed the shared log.
/// Native has no other writer, so it never fires there (focus still triggers a re-read).
pub struct Changes(#[allow(dead_code)] std::rc::Rc<std::cell::Cell<bool>>);

impl Changes {
    #[cfg(target_arch = "wasm32")]
    pub fn watch(ctx: &eframe::egui::Context) -> Self {
        use wasm_bindgen::{closure::Closure, JsCast};
        let flag = std::rc::Rc::new(std::cell::Cell::new(false));
        let (f, ctx) = (flag.clone(), ctx.clone());
        let on_storage =
            Closure::<dyn FnMut(web_sys::StorageEvent)>::new(move |e: web_sys::StorageEvent| {
                // `key` is None when the other tab cleared storage.
                if matches!(
                    e.key().as_deref(),
                    None | Some(EVENTS_KEY) | Some(SETTINGS_KEY) | Some(MISSION_START_KEY)
                ) {
                    f.set(true);
                    ctx.request_repaint();
                }
            });
        if let Some(w) = web_sys::window() {
            let _ =
                w.add_event_listener_with_callback("storage", on_storage.as_ref().unchecked_ref());
        }
        on_storage.forget(); // lives as long as the page
        Self(flag)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn watch(_ctx: &eframe::egui::Context) -> Self {
        Self(Default::default())
    }

    /// True once after each change.
    pub fn take(&self) -> bool {
        self.0.replace(false)
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!("ahc-safe-test-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        d
    }

    #[test]
    fn stored_log_is_classified() {
        assert!(matches!(parse_log(None), Stored::Missing));
        assert!(matches!(parse_log(Some("[]")), Stored::Events(e) if e.is_empty()));
        assert!(matches!(parse_log(Some("not json")), Stored::Unreadable(_)));
        assert!(matches!(parse_log(Some("{}")), Stored::Unreadable(_)));
        let demo_json = serde_json::to_string(&demo::events()).unwrap();
        match parse_log(Some(&demo_json)) {
            Stored::Events(e) => assert_eq!(e.len(), demo::events().len()),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn settings_fall_back_to_demo_defaults() {
        let d = Settings::parse(None);
        assert_eq!(d.crew_id, demo::CREW_ID);
        assert_eq!(d.one_way_delay_s, DEMO_ONE_WAY_DELAY_S);
        assert_eq!(d.met_offset_s, 0.0);
        assert_eq!(Settings::parse(Some("garbage")), d);
        let s = Settings::parse(Some(r#"{"one_way_delay_s": 1200, "met_offset_s": 7200}"#));
        assert_eq!((s.one_way_delay_s, s.met_offset_s), (1200.0, 7200.0));
        assert_eq!(s.crew_id, demo::CREW_ID);
    }

    #[test]
    fn first_load_seeds_the_demo_log_and_later_loads_read_it_back() {
        let dir = temp_dir("seed");
        let b = backend::Backend::at(dir.clone());
        let first = load_events_with(&b);
        assert!(first.notice.is_none(), "{:?}", first.notice);
        assert_eq!(first.events.unwrap().len(), demo::events().len());
        assert!(dir.join("events.json").exists());
        let again = load_events_with(&b);
        assert_eq!(again.events.unwrap().len(), demo::events().len());
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn mission_start_is_written_once_and_shared() {
        let dir = temp_dir("start");
        let b = backend::Backend::at(dir.clone());
        assert_eq!(mission_start_with(&b, 1000.0), 1000.0);
        assert_eq!(
            mission_start_with(&b, 9000.0),
            1000.0,
            "later loads keep the first"
        );
        std::fs::write(dir.join("mission_start.json"), "1731500000000.5").unwrap();
        assert_eq!(
            mission_start_with(&b, 9000.0),
            1_731_500_000_000.5,
            "web-written value"
        );
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn clock_counts_from_first_load_and_never_trails_the_log() {
        let s = Settings::parse(Some(r#"{"met_offset_s": 7200}"#));
        let t = now_met_s(10_000.0, 40_000.0, &s, &[]);
        assert_eq!(t, demo::NOW_MET_S + 30.0 + 7200.0);
        assert_eq!(
            now_met_s(10_000.0, 5_000.0, &s, &[]),
            demo::NOW_MET_S + 7200.0,
            "a clock behind the start does not run backwards"
        );
        let mut late = demo::events().pop().unwrap();
        late.met_s = t + 500.0;
        assert_eq!(now_met_s(10_000.0, 40_000.0, &s, &[late]), t + 500.0);
    }

    #[test]
    fn unreadable_log_is_reported_and_never_overwritten() {
        let dir = temp_dir("corrupt");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("events.json"), "{ half a log").unwrap();
        let loaded = load_events_with(&backend::Backend::at(dir.clone()));
        assert!(loaded.events.is_none());
        assert!(loaded.notice.unwrap().contains("left untouched"));
        let still = std::fs::read_to_string(dir.join("events.json")).unwrap();
        assert_eq!(still, "{ half a log");
        let _ = std::fs::remove_dir_all(dir);
    }

    fn reading(id: &str) -> Event {
        let mut e = demo::events().pop().unwrap();
        e.id = id.into();
        e
    }

    #[test]
    fn append_adds_to_the_stored_log_including_other_writers() {
        let dir = temp_dir("append");
        let b = backend::Backend::at(dir.clone());
        // No log yet: seeded, then appended, like the web app.
        let first = append_event_with(&b, reading("a")).unwrap();
        assert_eq!(first.len(), demo::events().len() + 1);
        // Another face appends meanwhile: its event is kept.
        let mut other = first.clone();
        other.push(reading("from-web"));
        b.write(EVENTS_KEY, &serde_json::to_string(&other).unwrap())
            .unwrap();
        let after = append_event_with(&b, reading("b")).unwrap();
        let ids: Vec<_> = after.iter().rev().take(3).map(|e| e.id.as_str()).collect();
        assert_eq!(ids, ["b", "from-web", "a"]);
        // The same id twice is stored once.
        let again = append_event_with(&b, reading("b")).unwrap();
        assert_eq!(again.len(), after.len());
        let stored = load_events_with(&b).events.unwrap();
        assert_eq!(stored.len(), after.len());
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn append_keeps_fields_this_build_does_not_know() {
        let dir = temp_dir("append-extra");
        let b = backend::Backend::at(dir.clone());
        let mut stored: Vec<serde_json::Value> =
            serde_json::from_str(&serde_json::to_string(&demo::events()).unwrap()).unwrap();
        stored[0]["added_by_web_later"] = serde_json::json!({ "keep": true });
        b.write(EVENTS_KEY, &serde_json::to_string(&stored).unwrap())
            .unwrap();
        append_event_with(&b, reading("new")).unwrap();
        let after: Vec<serde_json::Value> =
            serde_json::from_str(&std::fs::read_to_string(dir.join("events.json")).unwrap())
                .unwrap();
        assert_eq!(after.len(), stored.len() + 1);
        assert_eq!(after[0]["added_by_web_later"]["keep"], true);
        assert_eq!(after.last().unwrap()["id"], "new");
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn append_refuses_an_unreadable_log_and_leaves_it() {
        let dir = temp_dir("append-corrupt");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("events.json"), "{ half a log").unwrap();
        let err = append_event_with(&backend::Backend::at(dir.clone()), reading("a"));
        assert!(err.unwrap_err().starts_with("Not logged"));
        assert_eq!(
            std::fs::read_to_string(dir.join("events.json")).unwrap(),
            "{ half a log"
        );
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn settings_read_the_environment() {
        let s = Settings::parse(Some(r#"{"environment": "mars_base"}"#));
        assert_eq!(s.environment, Some(Environment::MarsBase));
        assert_eq!(Settings::parse(None).environment, None);
    }

    #[test]
    fn a_log_that_exists_but_cannot_be_read_is_never_seeded_over() {
        // Invalid UTF-8 makes the read itself fail (like a permission or sharing error):
        // that must not look like "no log yet".
        let dir = temp_dir("io-error");
        std::fs::create_dir_all(&dir).unwrap();
        let bytes = [0xff, 0xfe, b'[', b']'];
        std::fs::write(dir.join("events.json"), bytes).unwrap();
        let b = backend::Backend::at(dir.clone());
        let loaded = load_events_with(&b);
        assert!(loaded.events.is_none());
        assert!(loaded.notice.unwrap().contains("left untouched"));
        assert_eq!(std::fs::read(dir.join("events.json")).unwrap(), bytes);
        let _ = std::fs::remove_dir_all(dir);
    }
}
