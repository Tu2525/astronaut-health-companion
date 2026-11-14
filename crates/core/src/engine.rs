//! The one place status is decided. Deterministic, no I/O, no clock: `now` comes in the input.

use crate::types::*;

struct Obs {
    met_s: f64,
    value: f64,
    confidence: Confidence,
    env: Environment,
}

const ALL_REGIONS: [Region; 10] = [
    Region::Head,
    Region::Eyes,
    Region::Heart,
    Region::Lungs,
    Region::Bones,
    Region::Muscles,
    Region::Gut,
    Region::Immune,
    Region::Skin,
    Region::Mind,
];
/// Baseline needs this many earlier trusted readings; before that we are in cold start.
const BASELINE_MIN: usize = 5;

pub fn assess(input: &AssessInput) -> Assessment {
    let built_in;
    let registry = match &input.registry {
        Some(registry) => registry.as_slice(),
        None => {
            built_in = crate::registry();
            &built_in
        }
    };
    assess_with_registry(input, registry)
}

pub(crate) fn assess_with_registry(input: &AssessInput, registry: &[Indicator]) -> Assessment {
    let mut events: Vec<&Event> = input
        .events
        .iter()
        .filter(|e| e.crew_id == input.crew_id && e.met_s <= input.now_met_s)
        .collect();
    events.sort_by(|a, b| a.met_s.total_cmp(&b.met_s));

    let mut kit = input.kit.clone();
    for e in &events {
        if let EventKind::KitUse { item, qty } = &e.kind {
            if let Some(k) = kit.iter_mut().find(|k| &k.item == item) {
                k.qty = k.qty.saturating_sub(*qty);
            }
        }
    }

    let mut indicators = Vec::new();
    let mut flags = Vec::new();
    for ind in registry {
        let (st, escalation) = assess_indicator(ind, &events, input.now_met_s);
        if st.status >= Status::Watch {
            flags.push(Flag {
                indicator_id: ind.id.clone(),
                status: st.status,
                escalation,
                summary: format!("{}: {}", ind.name, st.reason),
                next_step: next_step(escalation, st.status).to_string(),
                actions: ind
                    .actions
                    .iter()
                    .map(|a| FlagAction {
                        available: a
                            .kit_item
                            .as_ref()
                            .is_none_or(|item| kit.iter().any(|k| &k.item == item && k.qty > 0)),
                        action: a.clone(),
                    })
                    .collect(),
            });
        }
        indicators.push(st);
    }
    flags.sort_by(|a, b| {
        b.status
            .cmp(&a.status)
            .then(b.escalation.cmp(&a.escalation))
    });

    let regions = ALL_REGIONS
        .iter()
        .map(|&region| RegionStatus {
            region,
            status: registry
                .iter()
                .zip(&indicators)
                .filter(|(i, _)| i.region == region)
                .map(|(_, s)| s.status)
                .max()
                .unwrap_or(Status::Unknown),
        })
        .collect();

    Assessment {
        overall: indicators
            .iter()
            .map(|s| s.status)
            .max()
            .unwrap_or(Status::Unknown),
        escalation: flags
            .iter()
            .map(|f| f.escalation)
            .max()
            .unwrap_or(Escalation::Monitor),
        indicators,
        regions,
        flags,
        kit,
    }
}

fn assess_indicator(ind: &Indicator, events: &[&Event], now: f64) -> (IndicatorStatus, Escalation) {
    let mut obs = Vec::new();
    let mut preflight = None;
    let mut over = None;
    for e in events {
        let mut push = |value: f64, confidence| {
            obs.push(Obs {
                met_s: e.met_s,
                value,
                confidence,
                env: e.environment,
            })
        };
        match &e.kind {
            EventKind::Reading {
                indicator_id,
                value,
                confidence,
                ..
            } if *indicator_id == ind.id => push(*value, *confidence),
            EventKind::CheckIn {
                mood,
                stress,
                energy,
                sleep_quality,
                ..
            } => {
                let v = match ind.id.as_str() {
                    "mood" => Some(mood),
                    "stress" => Some(stress),
                    "energy" => Some(energy),
                    "sleep_quality" => Some(sleep_quality),
                    _ => None,
                };
                if let Some(v) = v {
                    push(*v as f64, Confidence::Medium)
                }
            }
            EventKind::Baseline {
                indicator_id,
                value,
            } if *indicator_id == ind.id => preflight = Some(*value),
            EventKind::Override {
                indicator_id,
                bands,
                ..
            } if *indicator_id == ind.id => over = Some(*bands),
            _ => {}
        }
    }

    let default_bands = ind
        .ranges
        .iter()
        .find(|r| r.env.is_none())
        .or(ind.ranges.first())
        .map(|r| r.bands);
    let bands_for = |env: Environment| {
        over.or(ind
            .ranges
            .iter()
            .find(|r| r.env == Some(env))
            .map(|r| r.bands))
            .or(default_bands)
            .expect("registry: indicator without ranges")
    };

    let mut st = IndicatorStatus {
        indicator_id: ind.id.clone(),
        status: Status::Unknown,
        value: None,
        last_met_s: None,
        confidence: None,
        baseline: None,
        preflight,
        drift_pct: None,
        reason: "No data yet.".into(),
        bands: over
            .or(default_bands)
            .expect("registry: indicator without ranges"),
        source_status: ind.source.status,
    };
    let Some(last) = obs.last() else {
        return (st, Escalation::Monitor);
    };

    let b = bands_for(last.env); // judged by the environment the reading was taken in
    let prior: Vec<f64> = obs[..obs.len() - 1]
        .iter()
        .filter(|o| o.confidence != Confidence::Low)
        .map(|o| o.value)
        .collect();
    let baseline =
        (prior.len() >= BASELINE_MIN).then(|| prior.iter().sum::<f64>() / prior.len() as f64);
    let drift = |from: f64| (from != 0.0).then(|| (last.value - from) / from * 100.0);
    let base_drift = baseline.and_then(drift);
    let pre_drift = preflight.and_then(drift);

    st.value = Some(last.value);
    st.last_met_s = Some(last.met_s);
    st.confidence = Some(last.confidence);
    st.baseline = baseline;
    st.drift_pct = base_drift.or(pre_drift);
    st.bands = b;

    let age_h = (now - last.met_s) / 3600.0;
    let v = last.value;
    let outside = |lo: f64, hi: f64, x: f64| x < lo || x > hi;
    (st.status, st.reason) = if reading_is_stale(ind, now, last.met_s) {
        (
            Status::Unknown,
            format!("Stale: last measured {age_h:.0} h ago."),
        )
    } else if last.confidence == Confidence::Low {
        (
            Status::Unknown,
            "Low-confidence reading. Confirm with a proper device.".into(),
        )
    } else if outside(b.caution_low, b.caution_high, v) {
        (
            Status::Urgent,
            format!(
                "{v} {} is far outside the safe range ({}-{}).",
                ind.unit, b.caution_low, b.caution_high
            ),
        )
    } else if outside(b.nominal_low, b.nominal_high, v) {
        (
            Status::Caution,
            format!(
                "{v} {} is outside the normal range ({}-{}).",
                ind.unit, b.nominal_low, b.nominal_high
            ),
        )
    } else if let Some(d) = base_drift.filter(|d| d.abs() > ind.watch_drift_pct) {
        (
            Status::Watch,
            format!("{d:+.0}% from your in-flight baseline."),
        )
    } else if let Some(d) = pre_drift.filter(|d| d.abs() > ind.watch_drift_pct) {
        (
            Status::Watch,
            format!("{d:+.0}% from your preflight baseline."),
        )
    } else {
        (Status::Nominal, "Within your normal range.".into())
    };

    // Two trusted out-of-range readings in a row => involve ground.
    let persistent = obs
        .iter()
        .rev()
        .filter(|o| o.confidence != Confidence::Low)
        .take_while(|o| {
            let b = bands_for(o.env);
            outside(b.nominal_low, b.nominal_high, o.value)
        })
        .count()
        >= 2;
    let escalation = match st.status {
        Status::Urgent => Escalation::Urgent,
        Status::Caution if persistent => Escalation::ConsultGround,
        Status::Caution => Escalation::SelfCare,
        _ => Escalation::Monitor,
    };
    (st, escalation)
}

pub(crate) fn reading_is_stale(indicator: &Indicator, now_met_s: f64, last_met_s: f64) -> bool {
    (now_met_s - last_met_s) / 3600.0 > indicator.stale_after_h
}

fn next_step(e: Escalation, s: Status) -> &'static str {
    match (e, s) {
        (Escalation::Urgent, _) => "Follow the onboard urgent protocol now. Do not wait for Earth.",
        (Escalation::ConsultGround, _) => {
            "Send a report to ground and keep following the action plan."
        }
        (Escalation::SelfCare, _) => "Do the recommended action, then recheck.",
        _ => "Recheck at your next routine slot.",
    }
}
