//! Thin JSON bridge for the browser. All logic lives in ahc-core.
//! JS: `JSON.parse(assess(JSON.stringify(input)))`, typed with apps/web/src/contract/*.ts

use wasm_bindgen::prelude::*;

fn json<T: serde::Serialize>(v: &T) -> String {
    serde_json::to_string(v).expect("contract types always serialize")
}

/// AssessInput JSON -> Assessment JSON.
#[wasm_bindgen]
pub fn assess(input_json: &str) -> Result<String, JsError> {
    ahc_core::assess_json(input_json).map_err(|e| JsError::new(&e))
}

/// Indicator[] JSON.
#[wasm_bindgen]
pub fn registry() -> String {
    json(&ahc_core::registry())
}

/// Assessment and Indicator[] JSON -> Sbar JSON.
#[wasm_bindgen]
pub fn draft_sbar(assessment_json: &str, registry_json: &str) -> Result<String, JsError> {
    let assessment =
        serde_json::from_str(assessment_json).map_err(|e| JsError::new(&e.to_string()))?;
    let registry: Vec<ahc_core::Indicator> =
        serde_json::from_str(registry_json).map_err(|e| JsError::new(&e.to_string()))?;
    Ok(json(&ahc_core::draft_sbar(&assessment, &registry)))
}

/// Seeded demo state with the active crew's log and activity-only crew scores.
#[wasm_bindgen]
pub fn demo() -> String {
    json(&ahc_core::demo::seed())
}

/// Event[] JSON + crew id -> Score JSON. Goes through the value-stripping projection.
#[wasm_bindgen]
pub fn score(crew_id: &str, events_json: &str) -> Result<String, JsError> {
    let events: Vec<ahc_core::Event> =
        serde_json::from_str(events_json).map_err(|e| JsError::new(&e.to_string()))?;
    Ok(json(&ahc_core::gamify::score(
        crew_id,
        &ahc_core::gamify::activities(&events),
    )))
}
