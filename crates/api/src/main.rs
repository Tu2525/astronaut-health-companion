use std::env;

use ahc_core::{demo, AssessInput, Assessment};
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde_json::{json, Value};
use tokio::net::TcpListener;

mod comms;

#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    let bind = match env::var("AHC_BIND") {
        Ok(bind) => bind,
        Err(env::VarError::NotPresent) => "127.0.0.1:8080".into(),
        Err(error) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, error)),
    };
    let app = router().merge(comms::from_env()?).layer(comms::cors()?);
    let listener = TcpListener::bind(&bind).await?;
    eprintln!("ahc-api listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await
}

fn router() -> Router {
    Router::new()
        .route("/api/health", get(|| async { Json(json!({ "ok": true })) }))
        .route("/api/assess", post(assess))
        .route("/api/demo", get(|| async { Json(demo::seed()) }))
        .route(
            "/api/registry",
            get(|| async { Json(ahc_core::registry()) }),
        )
}

async fn assess(
    Json(input): Json<AssessInput>,
) -> Result<Json<Assessment>, (StatusCode, Json<Value>)> {
    ahc_core::validate(&input).map_err(|error| {
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({ "error": error })),
        )
    })?;

    Ok(Json(ahc_core::assess(&input)))
}

#[cfg(test)]
mod tests;
