use crate::{Assessment, Escalation, Indicator, Sbar, Status};

/// Draft a report from the highest-ranked flag and the engine's existing guidance.
pub fn draft_sbar(assessment: &Assessment, registry: &[Indicator]) -> Sbar {
    let overall = match assessment.overall {
        Status::Unknown => "Unknown",
        Status::Nominal => "Nominal",
        Status::Watch => "Watch",
        Status::Caution => "Caution",
        Status::Urgent => "Urgent",
    };
    let escalation = match assessment.escalation {
        Escalation::Monitor => "Monitor",
        Escalation::SelfCare => "Self-care",
        Escalation::ConsultGround => "Consult ground",
        Escalation::Urgent => "Urgent",
    };
    let assessment_text = format!("Overall status: {overall}. Escalation: {escalation}.");

    let Some(flag) = assessment.flags.first() else {
        return Sbar {
            situation: "No active indicator flags.".into(),
            background: "No flagged readings to summarize.".into(),
            assessment: assessment_text,
            recommendation: "Share any additional concerns with ground.".into(),
        };
    };

    let reading = assessment
        .indicators
        .iter()
        .find(|reading| reading.indicator_id == flag.indicator_id);
    let unit = registry
        .iter()
        .find(|indicator| indicator.id == flag.indicator_id)
        .map(|indicator| indicator.unit.as_str())
        .filter(|unit| !unit.is_empty());
    let with_unit = |value: f64| match unit {
        Some(unit) => format!("{} {unit}", (value * 10.0).round() / 10.0),
        None => ((value * 10.0).round() / 10.0).to_string(),
    };
    let mut background = Vec::new();
    if let Some(reading) = reading {
        if let Some(value) = reading.value {
            background.push(format!("Latest reading: {}.", with_unit(value)));
        }
        if let Some(value) = reading.baseline {
            background.push(format!("In-flight baseline: {}.", with_unit(value)));
        }
        if let Some(value) = reading.preflight {
            background.push(format!("Preflight baseline: {}.", with_unit(value)));
        }
    }

    Sbar {
        situation: flag.summary.clone(),
        background: if background.is_empty() {
            "No reading or baseline is available for the flagged indicator.".into()
        } else {
            background.join(" ")
        },
        assessment: assessment_text,
        recommendation: flag.next_step.clone(),
    }
}
