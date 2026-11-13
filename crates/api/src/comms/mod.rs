mod log;

use std::{
    env, fs,
    fs::{File, OpenOptions},
    io,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

use ahc_core::{demo, Environment, Event, EventKind, Report};
use axum::{
    extract::{Query, Request, State},
    http::{
        header::{AUTHORIZATION, CACHE_CONTROL, CONTENT_TYPE, WWW_AUTHENTICATE},
        HeaderMap, HeaderValue, Method, StatusCode,
    },
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Json, Router,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use serde::{Deserialize, Serialize};
use subtle::ConstantTimeEq;
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

use self::log::Log;

#[derive(Clone)]
struct Comms {
    store: Arc<Mutex<Store>>,
    clock: Arc<dyn Fn() -> f64 + Send + Sync>,
}

struct Store {
    _lock: File,
    reports: Log<Report>,
    notes: Log<QueuedNote>,
}

#[derive(Serialize, Deserialize)]
struct QueuedNote {
    deliver_at_earth_ms: f64,
    event: Event,
}

#[derive(Deserialize)]
struct NoteInput {
    crew_id: String,
    text: String,
    in_reply_to: Option<String>,
    delay_s: f64,
}

#[derive(Deserialize)]
struct CrewQuery {
    crew_id: String,
}

#[derive(Clone)]
struct Credentials {
    user: String,
    password: String,
}

#[derive(Clone)]
struct GroundUser(String);

pub(super) fn from_env() -> io::Result<Router> {
    let data_dir =
        env::var_os("AHC_DATA_DIR").map_or_else(|| PathBuf::from("target/ahc-data"), PathBuf::from);
    let user = optional_env("AHC_GROUND_USER")?;
    let password = match optional_env("AHC_GROUND_PASS")? {
        Some(password) => Some(password),
        None => optional_env("AHC_GROUND_PASSWORD")?,
    };
    let credentials = user.zip(password).and_then(|(user, password)| {
        (!user.is_empty() && !user.contains(':') && !password.is_empty())
            .then_some(Credentials { user, password })
    });
    router(&data_dir, credentials, || {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64()
            * 1000.0
    })
}

fn optional_env(key: &str) -> io::Result<Option<String>> {
    match env::var(key) {
        Ok(value) => Ok(Some(value)),
        Err(env::VarError::NotPresent) => Ok(None),
        Err(error) => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{key}: {error}"),
        )),
    }
}

pub(super) fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::HEAD, Method::POST])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
}

pub(super) fn cors() -> io::Result<CorsLayer> {
    let Some(origins) = optional_env("AHC_CORS_ORIGINS")? else {
        return Ok(cors_layer());
    };
    let allowed = origins
        .split(',')
        .map(str::trim)
        .map(|origin| {
            if origin.is_empty() || origin == "*" {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "AHC_CORS_ORIGINS requires explicit origins",
                ));
            }
            origin.parse::<HeaderValue>().map_err(|error| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("AHC_CORS_ORIGINS: {error}"),
                )
            })
        })
        .collect::<io::Result<Vec<_>>>()?;
    Ok(cors_layer().allow_origin(allowed))
}

fn router(
    data_dir: &Path,
    credentials: Option<Credentials>,
    clock: impl Fn() -> f64 + Send + Sync + 'static,
) -> io::Result<Router> {
    fs::create_dir_all(data_dir)?;
    let lock = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(data_dir.join("messages.lock"))?;
    lock.try_lock().map_err(|error| {
        let error = io::Error::from(error);
        io::Error::new(
            error.kind(),
            format!(
                "Cannot lock API data directory {}: {error}",
                data_dir.display()
            ),
        )
    })?;
    let state = Comms {
        store: Arc::new(Mutex::new(Store {
            _lock: lock,
            reports: Log::open(&data_dir.join("reports.jsonl"))?,
            notes: Log::open(&data_dir.join("notes.jsonl"))?,
        })),
        clock: Arc::new(clock),
    };
    let ground = Router::new()
        .route("/api/reports", get(list_reports))
        .route("/api/notes", post(send_note))
        .route_layer(middleware::from_fn_with_state(credentials, require_ground));
    Ok(ground
        .route("/api/reports", post(send_report))
        .route("/api/notes", get(list_notes))
        .with_state(state)
        .layer(tower_http::set_header::SetResponseHeaderLayer::overriding(
            CACHE_CONTROL,
            HeaderValue::from_static("no-store"),
        )))
}

async fn require_ground(
    State(credentials): State<Option<Credentials>>,
    mut request: Request,
    next: Next,
) -> Response {
    if let Some(credentials) =
        credentials.filter(|credentials| credentials.matches(request.headers()))
    {
        request
            .extensions_mut()
            .insert(GroundUser(credentials.user));
        return next.run(request).await;
    }
    (
        StatusCode::UNAUTHORIZED,
        [(
            WWW_AUTHENTICATE,
            "Basic realm=\"Ground Station\", charset=\"UTF-8\"",
        )],
    )
        .into_response()
}

impl Credentials {
    fn matches(&self, headers: &HeaderMap) -> bool {
        let decoded = headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.split_once(' '))
            .filter(|(scheme, _)| scheme.eq_ignore_ascii_case("Basic"))
            .and_then(|(_, payload)| STANDARD.decode(payload).ok());
        decoded.is_some_and(|bytes| {
            let Some(colon) = bytes.iter().position(|&byte| byte == b':') else {
                return false;
            };
            bool::from(
                bytes[..colon].ct_eq(self.user.as_bytes())
                    & bytes[colon + 1..].ct_eq(self.password.as_bytes()),
            )
        })
    }
}

type ApiError = (StatusCode, &'static str);

const STORE_ERROR: ApiError = (
    StatusCode::INTERNAL_SERVER_ERROR,
    "Message storage unavailable.",
);

impl Comms {
    async fn with_store<T: Send + 'static>(
        self,
        operation: impl FnOnce(&mut Store, f64) -> Result<T, ApiError> + Send + 'static,
    ) -> Result<T, ApiError> {
        tokio::task::spawn_blocking(move || {
            let mut store = self.store.lock().map_err(|_| STORE_ERROR)?;
            operation(&mut store, (self.clock)())
        })
        .await
        .map_err(|_| STORE_ERROR)?
    }
}

async fn send_report(
    State(state): State<Comms>,
    Json(report): Json<Report>,
) -> Result<StatusCode, ApiError> {
    if report.id.trim().is_empty()
        || report.crew_id.trim().is_empty()
        || report.sent_met_s < 0.0
        || report.sent_earth_ms < 0.0
        || report.deliver_at_earth_ms < report.sent_earth_ms
        || report.events.iter().any(|event| {
            event.crew_id != report.crew_id
                || event.met_s > report.sent_met_s
                || report.sent_met_s - event.met_s > 48.0 * 3600.0
        })
    {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            "Invalid report ID, time, or crew events.",
        ));
    }

    state
        .with_store(move |store, _| {
            if let Some(existing) = store
                .reports
                .entries
                .iter()
                .find(|entry| entry.id == report.id)
            {
                let existing = serde_json::to_vec(existing).map_err(|_| STORE_ERROR)?;
                let received = serde_json::to_vec(&report).map_err(|_| STORE_ERROR)?;
                if existing != received {
                    return Err((StatusCode::CONFLICT, "Report ID already exists."));
                }
            } else {
                store.reports.append(report).map_err(|_| STORE_ERROR)?;
            }
            Ok(StatusCode::CREATED)
        })
        .await
}

async fn list_reports(State(state): State<Comms>) -> Result<Json<Vec<Report>>, ApiError> {
    state
        .with_store(|store, now| {
            Ok(Json(
                store
                    .reports
                    .entries
                    .iter()
                    .filter(|report| report.deliver_at_earth_ms <= now)
                    .cloned()
                    .collect(),
            ))
        })
        .await
}

async fn send_note(
    State(state): State<Comms>,
    Extension(user): Extension<GroundUser>,
    Json(input): Json<NoteInput>,
) -> Result<StatusCode, ApiError> {
    if input.crew_id.trim().is_empty() || input.text.trim().is_empty() || input.delay_s < 0.0 {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            "A crew ID, note text, and non-negative delay are required.",
        ));
    }

    state
        .with_store(move |store, now| {
            let mut delivered = store.reports.entries.iter().filter(|report| {
                report.crew_id == input.crew_id && report.deliver_at_earth_ms <= now
            });
            let report = match &input.in_reply_to {
                Some(id) => Some(delivered.find(|report| &report.id == id).ok_or((
                    StatusCode::NOT_FOUND,
                    "Delivered report for this crew was not found.",
                ))?),
                None => delivered
                    .max_by(|left, right| left.sent_earth_ms.total_cmp(&right.sent_earth_ms)),
            };
            let deliver_at_earth_ms = now + input.delay_s * 1000.0;
            let (met_s, environment) = note_context(report, deliver_at_earth_ms, input.delay_s);
            let earth_ms = demo::LAUNCH_EARTH_MS + met_s * 1000.0;
            if !deliver_at_earth_ms.is_finite() || !earth_ms.is_finite() {
                return Err((
                    StatusCode::UNPROCESSABLE_ENTITY,
                    "Note delivery time is out of range.",
                ));
            }
            let event = Event {
                id: Uuid::new_v4().to_string(),
                crew_id: input.crew_id,
                met_s,
                earth_ms,
                environment,
                author: format!("ground:{}", user.0),
                kind: EventKind::GroundNote {
                    text: input.text,
                    sent_earth_ms: now,
                    in_reply_to: input.in_reply_to,
                },
            };
            store
                .notes
                .append(QueuedNote {
                    deliver_at_earth_ms,
                    event,
                })
                .map_err(|_| STORE_ERROR)?;
            Ok(StatusCode::CREATED)
        })
        .await
}

fn note_context(report: Option<&Report>, delivery_ms: f64, delay_s: f64) -> (f64, Environment) {
    let Some(report) = report else {
        return (demo::NOW_MET_S + delay_s, Environment::Transit);
    };
    let environment = report
        .events
        .iter()
        .max_by(|left, right| left.met_s.total_cmp(&right.met_s))
        .map_or(Environment::Transit, |event| event.environment);
    (
        report.sent_met_s + (delivery_ms - report.sent_earth_ms) / 1000.0,
        environment,
    )
}

async fn list_notes(
    State(state): State<Comms>,
    Query(query): Query<CrewQuery>,
) -> Result<Json<Vec<Event>>, ApiError> {
    state
        .with_store(move |store, now| {
            Ok(Json(
                store
                    .notes
                    .entries
                    .iter()
                    .filter(|note| {
                        note.event.crew_id == query.crew_id && note.deliver_at_earth_ms <= now
                    })
                    .map(|note| note.event.clone())
                    .collect(),
            ))
        })
        .await
}

#[cfg(test)]
mod tests;
