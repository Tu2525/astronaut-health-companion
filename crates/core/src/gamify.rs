//! Gamification sees `Activity` only. `Activity` has no value field, so it cannot rank health.

use crate::types::*;

/// The privacy projection: strips every health value from the log.
pub fn activities(events: &[Event]) -> Vec<Activity> {
    events
        .iter()
        .filter_map(|e| {
            let kind = match e.kind {
                EventKind::Reading { .. } => ActivityKind::Measurement,
                EventKind::CheckIn { .. } => ActivityKind::CheckIn,
                EventKind::Symptom { .. } => ActivityKind::SymptomReported,
                EventKind::ActionTaken { .. } => ActivityKind::FollowUp,
                EventKind::ReportSent { .. } => ActivityKind::ReportSent,
                _ => return None,
            };
            Some(Activity {
                crew_id: e.crew_id.clone(),
                met_s: e.met_s,
                kind,
            })
        })
        .collect()
}

pub fn score(crew_id: &str, acts: &[Activity]) -> Score {
    let mine: Vec<&Activity> = acts.iter().filter(|a| a.crew_id == crew_id).collect();
    let xp: u32 = mine
        .iter()
        .map(|a| match a.kind {
            ActivityKind::Measurement | ActivityKind::CheckIn | ActivityKind::ReportSent => 10,
            ActivityKind::FollowUp => 15,
            ActivityKind::SymptomReported => 30, // early-report bonus
        })
        .sum();
    // Streak in MISSION days, counted back from the latest active day.
    let mut days: Vec<i64> = mine.iter().map(|a| (a.met_s / 86400.0) as i64).collect();
    days.sort_unstable();
    days.dedup();
    let last = days.last().copied().unwrap_or(0);
    let streak = days
        .iter()
        .rev()
        .enumerate()
        .take_while(|(i, d)| **d == last - *i as i64)
        .count() as u32;
    Score {
        crew_id: crew_id.into(),
        xp,
        level: xp / 500 + 1,
        streak_days: streak,
    }
}
