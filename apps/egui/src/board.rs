//! View model for the B1 status board: turns engine output into display data.
//! Presentation only. Every status, reason, flag and escalation step comes from
//! `ahc_core::assess`; nothing here decides or ranks anything.

use ahc_core::{Assessment, Environment, Escalation, Event, Indicator, SourceStatus, Status};

/// One-way comms delay until SAFE-3 reads `ahc.settings.v1`.
/// Demo default from `docs/architecture.md` ("Comms delay"); see `Q-SAFE-2`.
pub const DEMO_ONE_WAY_DELAY_S: f64 = 20.0;

/// UIs show the top one or two flags (`docs/contract.md` section 4).
const MAX_FLAGS: usize = 2;

/// The four-step ladder, in order, with the wording from
/// `docs/reference/ui_faces_and_workflows.md` ("Escalation ladder").
pub const LADDER: [(Escalation, &str); 4] = [
    (Escalation::Monitor, "Monitor"),
    (Escalation::SelfCare, "Self-care"),
    (Escalation::ConsultGround, "Consult ground"),
    (Escalation::Urgent, "Urgent"),
];

pub struct Row {
    /// Registry id. Tests look rows up by it; SAFE-4 quick entry will use it to pick an indicator.
    #[cfg_attr(not(test), allow(dead_code))]
    pub id: String,
    pub name: String,
    pub value: String,
    pub status: Status,
    pub reason: String,
    pub source: SourceStatus,
    pub last_measured: String,
}

pub struct FlagCard {
    pub name: String,
    pub status: Status,
    pub summary: String,
    pub next_step: String,
    /// Index into `LADDER`.
    pub step: usize,
}

pub struct Board {
    pub mission_day: u64,
    pub met: String,
    pub environment: &'static str,
    pub comms: String,
    pub overall: Status,
    /// Index into `LADDER`.
    pub escalation_step: usize,
    pub flags: Vec<FlagCard>,
    pub more_flags: usize,
    pub rows: Vec<Row>,
}

/// Position of an escalation on the ladder. Exhaustive, so a new variant fails to compile.
pub fn ladder_step(e: Escalation) -> usize {
    match e {
        Escalation::Monitor => 0,
        Escalation::SelfCare => 1,
        Escalation::ConsultGround => 2,
        Escalation::Urgent => 3,
    }
}

/// `71.3 bpm`, but `3.0/5` for scores.
fn format_value(v: f64, unit: &str) -> String {
    if unit.is_empty() || unit.starts_with('/') {
        format!("{v:.1}{unit}")
    } else {
        format!("{v:.1} {unit}")
    }
}

pub fn environment_label(e: Environment) -> &'static str {
    match e {
        Environment::Station => "Station",
        Environment::Transit => "Transit",
        Environment::LunarBase => "Lunar base",
        Environment::MarsBase => "Mars base",
        Environment::EarthClinic => "Earth clinic",
    }
}

pub fn source_word(s: SourceStatus) -> &'static str {
    match s {
        SourceStatus::Sourced => "sourced",
        SourceStatus::Adapted => "adapted",
        SourceStatus::Placeholder => "placeholder",
    }
}

/// Mission elapsed time as `143d 06:12`.
pub fn format_met(met_s: f64) -> String {
    let total_min = (met_s.max(0.0) / 60.0).floor() as u64;
    let (days, h, m) = (total_min / 1440, total_min / 60 % 24, total_min % 60);
    format!("{days}d {h:02}:{m:02}")
}

impl Board {
    pub fn build(
        a: &Assessment,
        registry: &[Indicator],
        crew_id: &str,
        events: &[Event],
        now_met_s: f64,
        one_way_delay_s: f64,
    ) -> Self {
        let describe = |id: &str| {
            registry
                .iter()
                .find(|i| i.id == id)
                .map_or((id.to_string(), String::new()), |i| {
                    (i.name.clone(), i.unit.clone())
                })
        };
        // The environment the crew is in now is the one stored on its latest event,
        // using the same events the engine looks at (this crew, not in the future).
        let environment = events
            .iter()
            .filter(|e| e.crew_id == crew_id && e.met_s <= now_met_s)
            .max_by(|x, y| x.met_s.total_cmp(&y.met_s))
            .map_or("Unknown", |e| environment_label(e.environment));

        let rows = a
            .indicators
            .iter()
            .map(|i| {
                let (name, unit) = describe(&i.indicator_id);
                Row {
                    id: i.indicator_id.clone(),
                    name,
                    value: i.value.map_or("-".into(), |v| format_value(v, &unit)),
                    status: i.status,
                    reason: i.reason.clone(),
                    source: i.source_status,
                    last_measured: i.last_met_s.map_or("never".into(), |s| {
                        format!("{:.1} h ago", (now_met_s - s) / 3600.0)
                    }),
                }
            })
            .collect();

        let flags = a
            .flags
            .iter()
            .take(MAX_FLAGS)
            .map(|f| FlagCard {
                name: describe(&f.indicator_id).0,
                status: f.status,
                summary: f.summary.clone(),
                next_step: f.next_step.clone(),
                step: ladder_step(f.escalation),
            })
            .collect();

        Board {
            mission_day: (now_met_s.max(0.0) / 86_400.0).floor() as u64,
            met: format_met(now_met_s),
            environment,
            comms: format!("{one_way_delay_s:.0} s one-way (compressed demo time)"),
            overall: a.overall,
            escalation_step: ladder_step(a.escalation),
            flags,
            more_flags: a.flags.len().saturating_sub(MAX_FLAGS),
            rows,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ahc_core::{assess, demo, registry, AssessInput, Confidence, EventKind};

    fn input(events: Vec<Event>) -> AssessInput {
        AssessInput {
            crew_id: demo::CREW_ID.into(),
            events,
            now_met_s: demo::NOW_MET_S,
            kit: demo::kit(),
            registry: None,
        }
    }

    fn board_for(input: &AssessInput, registry: &[Indicator]) -> (Assessment, Board) {
        let a = assess(input);
        let b = Board::build(
            &a,
            registry,
            &input.crew_id,
            &input.events,
            input.now_met_s,
            DEMO_ONE_WAY_DELAY_S,
        );
        (a, b)
    }

    fn demo_board() -> (Assessment, Board) {
        board_for(&input(demo::events()), &registry())
    }

    fn row<'a>(b: &'a Board, id: &str) -> &'a Row {
        b.rows.iter().find(|r| r.id == id).expect(id)
    }

    fn reading_at(crew: &str, met_s: f64, environment: Environment) -> Event {
        Event {
            id: format!("test-{crew}-{met_s}"),
            crew_id: crew.into(),
            met_s,
            earth_ms: demo::LAUNCH_EARTH_MS + met_s * 1000.0,
            environment,
            author: crew.into(),
            kind: EventKind::Reading {
                indicator_id: "heart_rate".into(),
                value: 64.0,
                device: "test".into(),
                confidence: Confidence::High,
            },
        }
    }

    /// The demo opening state from docs/contract.md section 7, as the board shows it.
    /// Glucose is left out on purpose: see Q-SAFE-1.
    #[test]
    fn board_shows_the_demo_opening_state() {
        let (_, b) = demo_board();
        for id in ["heart_rate", "sleep_hours", "mood", "stress"] {
            assert_eq!(row(&b, id).status, Status::Watch, "{id}");
        }
        for id in ["bp_systolic", "bp_diastolic"] {
            assert_eq!(row(&b, id).status, Status::Unknown, "{id}");
            assert!(row(&b, id).reason.starts_with("Stale"), "{id}");
        }
        assert_eq!(row(&b, "body_temp").status, Status::Nominal);
        assert_eq!(b.overall, Status::Watch);
        assert_eq!(LADDER[b.escalation_step].1, "Monitor");
        assert_eq!((b.flags.len(), b.more_flags), (2, 2));
    }

    #[test]
    fn board_mirrors_the_engine_row_for_row_and_flag_for_flag() {
        let (a, b) = demo_board();
        assert_eq!(b.rows.len(), a.indicators.len());
        for (r, ind) in b.rows.iter().zip(&a.indicators) {
            assert_eq!(r.id, ind.indicator_id);
            assert_eq!((r.status, &r.reason), (ind.status, &ind.reason));
            assert_eq!(r.source, ind.source_status);
        }
        for (card, flag) in b.flags.iter().zip(&a.flags) {
            assert_eq!(card.status, flag.status);
            assert_eq!(card.summary, flag.summary);
            assert_eq!(card.next_step, flag.next_step);
            assert_eq!(LADDER[card.step].0, flag.escalation);
        }
    }

    #[test]
    fn ladder_table_and_ladder_step_agree() {
        for (i, (e, _)) in LADDER.iter().enumerate() {
            assert_eq!(ladder_step(*e), i);
        }
    }

    #[test]
    fn top_bar_matches_the_demo_story() {
        let (_, b) = demo_board();
        assert_eq!(b.mission_day, 143);
        assert_eq!(b.met, "143d 06:12");
        assert_eq!(b.environment, "Transit");
        assert_eq!(b.comms, "20 s one-way (compressed demo time)");
    }

    #[test]
    fn met_formatting_edges() {
        assert_eq!(format_met(0.0), "0d 00:00");
        assert_eq!(format_met(59.0), "0d 00:00");
        assert_eq!(format_met(86_399.0), "0d 23:59");
        assert_eq!(format_met(86_400.0), "1d 00:00");
        assert_eq!(format_met(-5.0), "0d 00:00");
        assert_eq!(format_met(f64::NAN), "0d 00:00");
    }

    #[test]
    fn values_carry_their_unit() {
        assert_eq!(format_value(71.34, "bpm"), "71.3 bpm");
        assert_eq!(format_value(3.0, "/5"), "3.0/5");
        assert_eq!(format_value(2.0, ""), "2.0");
        let (_, b) = demo_board();
        assert!(row(&b, "heart_rate").value.ends_with(" bpm"));
        assert!(row(&b, "mood").value.ends_with("/5"));
    }

    #[test]
    fn environment_ignores_other_crews_and_future_events() {
        let mut events = demo::events();
        events.push(reading_at("fe", demo::NOW_MET_S, Environment::Station));
        events.push(reading_at(
            demo::CREW_ID,
            demo::NOW_MET_S + 3600.0,
            Environment::LunarBase,
        ));
        let (_, b) = board_for(&input(events), &registry());
        assert_eq!(b.environment, "Transit");
    }

    #[test]
    fn empty_log_and_unknown_registry_ids_degrade_honestly() {
        let (_, b) = board_for(&input(Vec::new()), &[]);
        assert_eq!(b.environment, "Unknown");
        assert!(b.flags.is_empty());
        assert_eq!(b.more_flags, 0);
        for r in &b.rows {
            assert_eq!(r.name, r.id, "registry miss falls back to the id");
            assert_eq!(r.status, Status::Unknown);
            assert_eq!((r.value.as_str(), r.last_measured.as_str()), ("-", "never"));
        }
    }
}
