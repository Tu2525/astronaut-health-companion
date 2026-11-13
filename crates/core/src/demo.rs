//! Seeded demo history: Mars transit, mission day 143. SIMULATED data, never call it NASA data.
//! Deterministic so every face shows the same story.

use crate::types::*;

pub const CREW_ID: &str = "cdr";
/// Mission day 143, 06:12.
pub const NOW_MET_S: f64 = 143.0 * 86400.0 + 6.0 * 3600.0 + 12.0 * 60.0;
/// Earth time at MET 0 (2026-06-01T00:00:00Z), unix ms.
pub const LAUNCH_EARTH_MS: f64 = 1_780_272_000_000.0;

/// The same simulated opening state is served by HTTP and WASM.
#[derive(serde::Serialize)]
pub struct Demo {
    pub crew_id: &'static str,
    pub now_met_s: f64,
    pub launch_earth_ms: f64,
    pub events: Vec<Event>,
    pub kit: Vec<KitItem>,
}

pub fn seed() -> Demo {
    Demo {
        crew_id: CREW_ID,
        now_met_s: NOW_MET_S,
        launch_earth_ms: LAUNCH_EARTH_MS,
        events: events(),
        kit: kit(),
    }
}

pub fn kit() -> Vec<KitItem> {
    [
        ("electrolyte_drink", 6),
        ("glucose_tabs", 4),
        ("cold_pack", 2),
        ("sleep_mask", 1),
    ]
    .map(|(item, qty)| KitItem {
        item: item.into(),
        qty,
    })
    .to_vec()
}

pub fn events() -> Vec<Event> {
    let mut out = Vec::new();
    let mut push = |day: u32, hour: f64, kind: EventKind| {
        let met_s = day as f64 * 86400.0 + hour * 3600.0;
        out.push(Event {
            id: format!("demo-{}", out.len()),
            crew_id: CREW_ID.into(),
            met_s,
            earth_ms: LAUNCH_EARTH_MS + met_s * 1000.0,
            environment: Environment::Transit,
            author: CREW_ID.into(),
            kind,
        });
    };
    let reading = |id: &str, value: f64, device: &str| EventKind::Reading {
        indicator_id: id.into(),
        value: (value * 10.0).round() / 10.0,
        device: device.into(),
        confidence: Confidence::High,
    };
    // small deterministic wobble in [-1, 1]
    let wob = |day: u32, k: u32| ((day * 37 + k * 17) % 7) as f64 / 3.0 - 1.0;

    for (id, v) in [
        ("heart_rate", 60.0),
        ("bp_systolic", 116.0),
        ("bp_diastolic", 74.0),
    ] {
        push(
            0,
            0.0,
            EventKind::Baseline {
                indicator_id: id.into(),
                value: v,
            },
        );
    }
    for day in 129..=143u32 {
        let late = day.saturating_sub(139) as f64; // the story: last 4 days drift
        push(
            day,
            5.0,
            reading(
                "heart_rate",
                62.0 + wob(day, 1) * 2.0 + late * 2.5,
                "chest strap",
            ),
        );
        push(
            day,
            5.0,
            reading(
                "sleep_hours",
                7.3 + wob(day, 2) * 0.3 - late * 0.35,
                "sleep log",
            ),
        );
        push(
            day,
            5.1,
            reading("body_temp", 36.7 + wob(day, 3) * 0.1, "thermometer"),
        );
        push(
            day,
            5.1,
            reading("spo2", 98.0 + wob(day, 4).round(), "pulse oximeter"),
        );
        push(
            day,
            5.2,
            reading(
                "radiation_dose",
                1.8 + wob(day, 5) * 0.2,
                "simulated dosimeter",
            ),
        );
        // BP and glucose every other day; none on the last two days, so the scheduler has a reason to prompt.
        if day % 2 == 1 && day < 142 {
            push(
                day,
                5.3,
                reading("bp_systolic", 118.0 + wob(day, 6) * 3.0, "cuff"),
            );
            push(
                day,
                5.3,
                reading("bp_diastolic", 76.0 + wob(day, 7) * 2.0, "cuff"),
            );
            push(
                day,
                5.4,
                reading("blood_glucose", 92.0 + wob(day, 8) * 4.0, "glucometer"),
            );
        }
        let mood = if day >= 142 { 3 } else { 4 };
        push(
            day,
            5.5,
            EventKind::CheckIn {
                mood,
                stress: mood,
                energy: mood,
                sleep_quality: mood,
                note: None,
            },
        );
    }
    out
}
