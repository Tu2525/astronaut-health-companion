//! ahc-core: shared contract + assessment engine. See docs/contract.md.

pub mod demo;
pub mod engine;
pub mod gamify;
pub mod types;

pub use engine::assess;
pub use types::*;

/// Built-in indicator registry (rules as data). Edit registry.json, not code.
pub fn registry() -> Vec<Indicator> {
    serde_json::from_str(include_str!("../registry.json")).expect("registry.json is invalid")
}

/// JSON boundary used by WASM, FFI and the API: `AssessInput` in, `Assessment` out.
pub fn assess_json(input: &str) -> Result<String, String> {
    let input: AssessInput = serde_json::from_str(input).map_err(|e| e.to_string())?;
    serde_json::to_string(&assess(&input)).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn demo_input() -> AssessInput {
        AssessInput {
            crew_id: demo::CREW_ID.into(),
            events: demo::events(),
            now_met_s: demo::NOW_MET_S,
            kit: demo::kit(),
            registry: None,
        }
    }
    fn status(a: &Assessment, id: &str) -> Status {
        a.indicators
            .iter()
            .find(|i| i.indicator_id == id)
            .unwrap()
            .status
    }
    fn read(id: &str, value: f64, confidence: Confidence, dt: f64) -> Event {
        Event {
            id: format!("t-{id}-{dt}"),
            crew_id: demo::CREW_ID.into(),
            met_s: demo::NOW_MET_S + dt,
            earth_ms: 0.0,
            environment: Environment::Transit,
            author: demo::CREW_ID.into(),
            kind: EventKind::Reading {
                indicator_id: id.into(),
                value,
                device: "test".into(),
                confidence,
            },
        }
    }

    #[test]
    fn registry_is_valid() {
        let r = registry();
        assert!(r.len() >= 8);
        for i in &r {
            assert!(!i.ranges.is_empty(), "{} has no ranges", i.id);
        }
    }

    #[test]
    fn demo_story_opening_state() {
        let a = assess(&demo_input());
        assert_eq!(status(&a, "heart_rate"), Status::Watch); // creeping above baseline
        assert_eq!(status(&a, "sleep_hours"), Status::Watch); // sleep debt
        assert_eq!(status(&a, "mood"), Status::Watch); // mood dip
        assert_eq!(status(&a, "bp_systolic"), Status::Unknown); // stale -> scheduler prompts a BP check
        assert_eq!(status(&a, "body_temp"), Status::Nominal);
        assert_eq!(a.overall, Status::Watch);
        assert_eq!(a.escalation, Escalation::Monitor);
    }

    #[test]
    fn bp_recheck_escalates_then_consults_ground() {
        let mut input = demo_input();
        input
            .events
            .push(read("bp_systolic", 138.0, Confidence::High, 60.0));
        input.now_met_s += 120.0;
        let a = assess(&input);
        assert_eq!(status(&a, "bp_systolic"), Status::Caution);
        assert_eq!(a.flags[0].indicator_id, "bp_systolic"); // worst first
        assert_eq!(a.flags[0].escalation, Escalation::SelfCare);

        input
            .events
            .push(read("bp_systolic", 139.0, Confidence::High, 7200.0));
        input.now_met_s += 7200.0;
        assert_eq!(assess(&input).escalation, Escalation::ConsultGround); // persistent
    }

    #[test]
    fn low_confidence_never_flags() {
        let mut input = demo_input();
        input
            .events
            .push(read("heart_rate", 150.0, Confidence::Low, 60.0)); // camera pulse
        input.now_met_s += 120.0;
        assert_eq!(status(&assess(&input), "heart_rate"), Status::Unknown);
    }

    #[test]
    fn kit_runs_out() {
        let mut input = demo_input();
        input
            .events
            .push(read("bp_systolic", 138.0, Confidence::High, 60.0));
        let mut e = read("x", 0.0, Confidence::High, 61.0);
        e.kind = EventKind::KitUse {
            item: "electrolyte_drink".into(),
            qty: 6,
        };
        input.events.push(e);
        input.now_met_s += 120.0;
        let a = assess(&input);
        let f = a
            .flags
            .iter()
            .find(|f| f.indicator_id == "bp_systolic")
            .unwrap();
        assert!(f
            .actions
            .iter()
            .any(|x| x.action.kit_item.is_some() && !x.available));
    }

    #[test]
    fn gamification_sees_no_values_and_scores() {
        let acts = gamify::activities(&demo::events());
        let s = gamify::score(demo::CREW_ID, &acts);
        assert_eq!(s.streak_days, 15);
        assert!(s.xp > 0);
    }

    #[test]
    fn json_boundary_round_trips() {
        let out = assess_json(&serde_json::to_string(&demo_input()).unwrap()).unwrap();
        let _: Assessment = serde_json::from_str(&out).unwrap();
    }
}
