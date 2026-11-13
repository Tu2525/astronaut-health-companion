//! THE CONTRACT. Every face (web, egui, api, mobile) speaks these types as JSON.
//! Only the core owner edits this file. Everyone else opens a `contract:` issue/PR request.
//! Times are f64 on purpose (ts-rs maps u64/i64 to bigint, which JSON cannot carry).

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Ordered by severity: `max()` of statuses is the worst one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum Status {
    Unknown,
    Nominal,
    Watch,
    Caution,
    Urgent,
}

/// The four-step ladder, same everywhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum Escalation {
    Monitor,
    SelfCare,
    ConsultGround,
    Urgent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum Environment {
    Station,
    Transit,
    LunarBase,
    MarsBase,
    EarthClinic,
}

/// NASA HRP five hazards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum Hazard {
    Radiation,
    Isolation,
    Distance,
    Gravity,
    ClosedEnvironment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum Region {
    Head,
    Eyes,
    Heart,
    Lungs,
    Bones,
    Muscles,
    Gut,
    Immune,
    Skin,
    Mind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum SourceStatus {
    Sourced,
    Adapted,
    Placeholder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum Confidence {
    Low,
    Medium,
    High,
}

// ---------- Registry (rules as data, see registry.json) ----------

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Indicator {
    pub id: String,
    pub name: String,
    pub unit: String,
    /// display = canonical * factor + offset
    pub alt_units: Vec<AltUnit>,
    pub region: Region,
    pub hazards: Vec<Hazard>,
    pub what: String,
    pub why_in_space: String,
    pub how_to_measure: Vec<String>,
    /// Adaptation that is normal in flight and must not alarm.
    pub expected_change: Option<String>,
    /// First entry with `env == None` is the default; env-specific entries win.
    pub ranges: Vec<EnvRange>,
    /// |drift from baseline| above this (percent) => Watch.
    pub watch_drift_pct: f64,
    /// Older than this => Unknown (stale).
    pub stale_after_h: f64,
    pub source: Source,
    pub actions: Vec<Action>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AltUnit {
    pub unit: String,
    pub factor: f64,
    pub offset: f64,
}

/// Inside nominal => Nominal. Outside nominal but inside caution => Caution. Outside caution => Urgent.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Bands {
    pub nominal_low: f64,
    pub nominal_high: f64,
    pub caution_low: f64,
    pub caution_high: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct EnvRange {
    pub env: Option<Environment>,
    pub bands: Bands,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Source {
    pub citation: String,
    pub url: Option<String>,
    pub status: SourceStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Action {
    pub id: String,
    pub text: String,
    /// Kit item consumed by this action, if any. Never a medication dose.
    pub kit_item: Option<String>,
}

// ---------- Event log (append-only) ----------

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Event {
    /// Unique. Clients use `crypto.randomUUID()` or equivalent.
    pub id: String,
    pub crew_id: String,
    /// Mission elapsed time, seconds. Streaks and sleep debt use this clock.
    pub met_s: f64,
    /// Earth time, unix milliseconds.
    pub earth_ms: f64,
    /// Environment AT THE TIME of the event. Never rewritten.
    pub environment: Environment,
    /// Who wrote it: a crew id, "ground:<name>", or "device:<name>".
    pub author: String,
    pub kind: EventKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(tag = "type", rename_all = "snake_case")]
#[ts(export)]
pub enum EventKind {
    Reading {
        indicator_id: String,
        value: f64,
        device: String,
        confidence: Confidence,
    },
    /// Preflight baseline for one indicator.
    Baseline {
        indicator_id: String,
        value: f64,
    },
    Symptom {
        text: String,
        region: Option<Region>,
    },
    /// Each field 1 (bad) to 5 (good). Engine reads these as indicators mood/stress/energy/sleep_quality.
    CheckIn {
        mood: u8,
        stress: u8,
        energy: u8,
        sleep_quality: u8,
        note: Option<String>,
    },
    KitUse {
        item: String,
        qty: u32,
    },
    ActionTaken {
        action_id: String,
        indicator_id: String,
    },
    ReportSent {
        report_id: String,
        deliver_at_earth_ms: f64,
    },
    GroundNote {
        text: String,
        sent_earth_ms: f64,
        in_reply_to: Option<String>,
    },
    /// Flight surgeon changes a threshold for this crew member.
    Override {
        indicator_id: String,
        bands: Bands,
        reason: String,
    },
}

// ---------- Engine input / output ----------

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct KitItem {
    pub item: String,
    pub qty: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AssessInput {
    pub crew_id: String,
    pub events: Vec<Event>,
    pub now_met_s: f64,
    /// Starting kit (preset). The engine subtracts KitUse events.
    pub kit: Vec<KitItem>,
    /// None => built-in registry.
    pub registry: Option<Vec<Indicator>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Assessment {
    pub overall: Status,
    pub escalation: Escalation,
    pub indicators: Vec<IndicatorStatus>,
    pub regions: Vec<RegionStatus>,
    /// Ranked, worst first. UIs show the top 1-2.
    pub flags: Vec<Flag>,
    pub kit: Vec<KitItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct IndicatorStatus {
    pub indicator_id: String,
    pub status: Status,
    pub value: Option<f64>,
    pub last_met_s: Option<f64>,
    pub confidence: Option<Confidence>,
    /// Mean of earlier trusted in-flight readings. None during cold start (< 5 readings).
    pub baseline: Option<f64>,
    pub preflight: Option<f64>,
    pub drift_pct: Option<f64>,
    /// Plain sentence explaining the status. Deterministic; the AI may rephrase but never change it.
    pub reason: String,
    pub bands: Bands,
    pub source_status: SourceStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct RegionStatus {
    pub region: Region,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Flag {
    pub indicator_id: String,
    pub status: Status,
    pub escalation: Escalation,
    pub summary: String,
    pub next_step: String,
    pub actions: Vec<FlagAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct FlagAction {
    pub action: Action,
    /// false => kit item used up; UI shows "contact ground" fallback.
    pub available: bool,
}

// ---------- Gamification (activity only: these types cannot hold a health value) ----------

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum ActivityKind {
    Measurement,
    CheckIn,
    SymptomReported,
    FollowUp,
    ReportSent,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Activity {
    pub crew_id: String,
    pub met_s: f64,
    pub kind: ActivityKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Score {
    pub crew_id: String,
    pub xp: u32,
    pub level: u32,
    pub streak_days: u32,
}
