//! B2 Quick entry (SAFE-4): turns what the crew typed into one `reading` event.
//! Decides nothing about the value: the engine judges it once it is in the log.
//! The rules for the event are in `apps/egui/AGENTS.md`: `device: "safe-mode"`,
//! `confidence: "high"`, the current environment and both clocks. Never edits or deletes.

use ahc_core::{demo, Confidence, Environment, Event, EventKind, Indicator};

use crate::storage::Settings;

pub const DEVICE: &str = "safe-mode";

/// What the crew has typed so far.
#[derive(Debug, Default)]
pub struct Draft {
    /// Index into the registry.
    pub selected: usize,
    pub text: String,
}

impl Draft {
    /// Moves the indicator choice up or down the registry, wrapping around.
    pub fn step(&mut self, delta: isize, len: usize) {
        if len > 0 {
            self.selected = (self.selected as isize + delta).rem_euclid(len as isize) as usize;
        }
    }
}

/// Reads a number as typed. A decimal comma is accepted. Only checks that it is a
/// number: whether it is in range is the engine's call.
pub fn parse_value(text: &str) -> Result<f64, String> {
    let t = text.trim().replace(',', ".");
    if t.is_empty() {
        return Err("Type a value first.".into());
    }
    match t.parse::<f64>() {
        Ok(v) if v.is_finite() => Ok(v),
        _ => Err(format!("\"{}\" is not a number.", text.trim())),
    }
}

/// The environment a new event is stamped with: the one in settings, else the web
/// app's default (`apps/web/src/store.ts`), so both faces stamp the same value.
pub fn current_environment(settings: &Settings) -> Environment {
    settings.environment.unwrap_or(Environment::Transit)
}

/// One reading by the crew member in settings, at `now_met_s`, in the indicator's
/// canonical unit. Earth time follows `docs/architecture.md` "Clock".
pub fn reading_event(
    id: String,
    indicator: &Indicator,
    value: f64,
    settings: &Settings,
    now_met_s: f64,
) -> Event {
    Event {
        id,
        crew_id: settings.crew_id.clone(),
        met_s: now_met_s,
        earth_ms: demo::LAUNCH_EARTH_MS + now_met_s * 1000.0,
        environment: current_environment(settings),
        author: settings.crew_id.clone(),
        kind: EventKind::Reading {
            indicator_id: indicator.id.clone(),
            value,
            device: DEVICE.into(),
            confidence: Confidence::High,
        },
    }
}

/// A random (v4) UUID, as the contract asks of every client.
pub fn new_event_id() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        // crypto.getRandomValues; uuid's own wasm backend would need a build flag.
        let mut bytes = [0u8; 16];
        let filled = web_sys::window()
            .and_then(|w| w.crypto().ok())
            .is_some_and(|c| c.get_random_values_with_u8_array(&mut bytes).is_ok());
        if !filled {
            for b in &mut bytes {
                *b = (js_sys::Math::random() * 256.0) as u8;
            }
        }
        uuid::Builder::from_random_bytes(bytes)
            .into_uuid()
            .to_string()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        uuid::Uuid::new_v4().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ahc_core::registry;

    #[test]
    fn values_parse_as_typed() {
        assert_eq!(parse_value(" 138 "), Ok(138.0));
        assert_eq!(parse_value("36,8"), Ok(36.8));
        assert_eq!(parse_value("-0.5"), Ok(-0.5));
        assert!(parse_value("").is_err());
        assert!(parse_value("12a").is_err());
        assert!(parse_value("NaN").is_err());
        assert!(parse_value("inf").is_err());
    }

    #[test]
    fn indicator_choice_wraps_both_ways() {
        let mut d = Draft::default();
        d.step(-1, 10);
        assert_eq!(d.selected, 9);
        d.step(1, 10);
        assert_eq!(d.selected, 0);
        d.step(1, 0);
        assert_eq!(d.selected, 0);
    }

    #[test]
    fn reading_follows_the_safe_mode_rules() {
        let reg = registry();
        let bp = reg.iter().find(|i| i.id == "bp_systolic").unwrap();
        let s = Settings::parse(Some(r#"{"crew_id": "fe1", "environment": "lunar_base"}"#));
        let e = reading_event("id-1".into(), bp, 138.0, &s, 1000.0);
        assert_eq!((e.crew_id.as_str(), e.author.as_str()), ("fe1", "fe1"));
        assert_eq!(e.met_s, 1000.0);
        assert_eq!(e.earth_ms, demo::LAUNCH_EARTH_MS + 1_000_000.0);
        assert_eq!(e.environment, Environment::LunarBase);
        match e.kind {
            EventKind::Reading {
                indicator_id,
                value,
                device,
                confidence,
            } => {
                assert_eq!(indicator_id, "bp_systolic");
                assert_eq!(value, 138.0);
                assert_eq!(device, "safe-mode");
                assert_eq!(confidence, Confidence::High);
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn environment_defaults_like_the_web_app() {
        assert_eq!(
            current_environment(&Settings::parse(None)),
            Environment::Transit
        );
        let s = Settings::parse(Some(r#"{"environment": "not_a_place"}"#));
        assert_eq!(current_environment(&s), Environment::Transit);
    }

    #[test]
    fn event_ids_are_uuids_and_differ() {
        let (a, b) = (new_event_id(), new_event_id());
        assert_ne!(a, b);
        assert_eq!(uuid::Uuid::parse_str(&a).unwrap().get_version_num(), 4);
    }

    #[test]
    fn the_demo_story_bp_reading_gives_caution() {
        // docs/contract.md section 7: log bp_systolic 138 -> caution / self_care.
        // Checks the event this module builds is one the engine reads.
        use ahc_core::{assess, AssessInput, Escalation, Status};
        let reg = registry();
        let bp = reg.iter().find(|i| i.id == "bp_systolic").unwrap();
        let s = Settings::parse(None);
        let mut events = demo::events();
        events.push(reading_event(
            new_event_id(),
            bp,
            138.0,
            &s,
            demo::NOW_MET_S,
        ));
        let a = assess(&AssessInput {
            crew_id: s.crew_id.clone(),
            events,
            now_met_s: demo::NOW_MET_S,
            kit: demo::kit(),
            registry: None,
        });
        let st = a
            .indicators
            .iter()
            .find(|i| i.indicator_id == "bp_systolic")
            .unwrap();
        assert_eq!(st.status, Status::Caution);
        assert_eq!(a.escalation, Escalation::SelfCare);
    }
}
