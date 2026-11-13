use std::borrow::Cow;

use crate::{
    engine::{assess_with_registry, reading_is_stale},
    AssessInput, Confidence, EventKind, Status, Task, TaskKind,
};

const DAY_S: f64 = 86_400.0;
const HOUR_S: f64 = 3_600.0;
const MORNING_VITALS: [&str; 3] = ["heart_rate", "body_temp", "spo2"];

/// Return today's routine slots and current optional follow-ups in MET order.
pub fn due_tasks(input: &AssessInput) -> Vec<Task> {
    let day = (input.now_met_s / DAY_S).floor();
    let day_start = day * DAY_S;
    let registry = input
        .registry
        .as_deref()
        .map_or_else(|| Cow::Owned(crate::registry()), Cow::Borrowed);
    let today: Vec<_> = input
        .events
        .iter()
        .filter(|event| {
            event.crew_id == input.crew_id
                && event.met_s >= day_start
                && event.met_s <= input.now_met_s
        })
        .collect();

    let mut tasks = Vec::new();
    for (id, hour) in
        std::iter::once(("sleep_hours", 5.0)).chain(MORNING_VITALS.map(|id| (id, 6.0)))
    {
        let Some(indicator) = registry.iter().find(|indicator| indicator.id == id) else {
            continue;
        };
        let completed = today.iter().any(|event| {
            matches!(
                &event.kind,
                EventKind::Reading { indicator_id, confidence, .. }
                    if indicator_id == id && *confidence != Confidence::Low
            )
        });
        tasks.push(Task {
            id: format!("routine-{day}-{id}"),
            kind: TaskKind::Routine,
            label: indicator.name.clone(),
            indicator_id: Some(id.into()),
            due_met_s: day_start + hour * HOUR_S,
            completed,
            reason: None,
        });
    }
    tasks.push(Task {
        id: format!("routine-{day}-check-in"),
        kind: TaskKind::Routine,
        label: "Crew check-in".into(),
        indicator_id: None,
        due_met_s: day_start + 18.0 * HOUR_S,
        completed: today
            .iter()
            .any(|event| matches!(event.kind, EventKind::CheckIn { .. })),
        reason: None,
    });

    let assessment = assess_with_registry(input, &registry);
    let mut check_in_prompt = false;
    let mut blood_pressure_prompt = false;
    for (indicator, reading) in registry.iter().zip(assessment.indicators) {
        let Some(last_met_s) = reading.last_met_s else {
            continue;
        };
        // The assessment already decides drift. Apply the same age rule to identify stale data.
        let due_met_s = match reading.status {
            Status::Watch => last_met_s,
            Status::Unknown if reading_is_stale(indicator, input.now_met_s, last_met_s) => {
                last_met_s + indicator.stale_after_h * HOUR_S
            }
            _ => continue,
        };
        let check_in = matches!(
            indicator.id.as_str(),
            "mood" | "stress" | "energy" | "sleep_quality"
        );
        let blood_pressure = matches!(indicator.id.as_str(), "bp_systolic" | "bp_diastolic");
        if blood_pressure {
            if blood_pressure_prompt {
                continue;
            }
            blood_pressure_prompt = true;
        }
        if check_in {
            if check_in_prompt {
                continue;
            }
            check_in_prompt = true;
        }
        let label = if check_in {
            "Extra crew check-in".into()
        } else if blood_pressure {
            "Recheck blood pressure".into()
        } else {
            format!("Recheck {}", indicator.name)
        };
        tasks.push(Task {
            id: format!("bonus-{}-{last_met_s}", indicator.id),
            kind: TaskKind::Bonus,
            label,
            indicator_id: (!check_in).then(|| indicator.id.clone()),
            due_met_s,
            completed: false,
            reason: Some(reading.reason),
        });
    }

    tasks.sort_by(|left, right| left.due_met_s.total_cmp(&right.due_met_s));
    tasks
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{demo, Environment, Event};

    fn demo_input() -> AssessInput {
        AssessInput {
            crew_id: demo::CREW_ID.into(),
            events: demo::events(),
            now_met_s: demo::NOW_MET_S,
            kit: demo::kit(),
            registry: None,
        }
    }

    #[test]
    fn demo_has_an_optional_blood_pressure_prompt() {
        let input = demo_input();
        let tasks = due_tasks(&input);
        let bp = tasks
            .iter()
            .find(|task| task.indicator_id.as_deref() == Some("bp_systolic"))
            .unwrap();
        assert_eq!(bp.kind, TaskKind::Bonus);
        assert!(!bp.completed);
        assert!(bp.due_met_s < input.now_met_s);
        assert!(bp.reason.as_deref().unwrap().starts_with("Stale:"));
        assert_eq!(
            tasks
                .iter()
                .filter(|task| task.label == "Recheck blood pressure")
                .count(),
            1
        );
        assert!(tasks.iter().any(|task| task.kind == TaskKind::Bonus
            && task.indicator_id.as_deref() == Some("heart_rate")));
        assert_eq!(
            tasks
                .iter()
                .filter(|task| task.kind == TaskKind::Bonus && task.indicator_id.is_none())
                .count(),
            1
        );
        assert!(tasks
            .iter()
            .filter(|task| task.kind == TaskKind::Routine)
            .all(|task| task.completed));
    }

    #[test]
    fn a_fresh_blood_pressure_reading_clears_the_stale_prompt() {
        let mut input = demo_input();
        input.events.push(Event {
            id: "new-bp".into(),
            crew_id: input.crew_id.clone(),
            met_s: input.now_met_s,
            earth_ms: demo::LAUNCH_EARTH_MS + input.now_met_s * 1000.0,
            environment: Environment::Transit,
            author: input.crew_id.clone(),
            kind: EventKind::Reading {
                indicator_id: "bp_systolic".into(),
                value: 118.0,
                device: "cuff".into(),
                confidence: Confidence::High,
            },
        });

        let tasks = due_tasks(&input);
        assert!(tasks
            .iter()
            .all(|task| task.indicator_id.as_deref() != Some("bp_systolic")));
        assert!(tasks
            .iter()
            .any(|task| task.indicator_id.as_deref() == Some("bp_diastolic")));
        assert_eq!(
            crate::gamify::score(&input.crew_id, &crate::gamify::activities(&input.events))
                .streak_days,
            15
        );
    }
}
