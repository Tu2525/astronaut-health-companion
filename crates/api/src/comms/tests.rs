use std::sync::atomic::{AtomicU64, Ordering};

use axum::body::{to_bytes, Body};
use serde_json::{json, Value};
use tower::ServiceExt;

use super::*;

struct TestDirectory(PathBuf);

impl TestDirectory {
    fn new() -> Self {
        Self(env::temp_dir().join(format!("ahc-comms-{}", Uuid::new_v4())))
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn test_router(directory: &TestDirectory, clock: &Arc<AtomicU64>) -> Router {
    let clock = Arc::clone(clock);
    let credentials = Credentials {
        user: "ground".into(),
        password: "test:password".into(),
    };
    router(&directory.0, Some(credentials), move || {
        clock.load(Ordering::Relaxed) as f64
    })
    .unwrap()
}

async fn request(
    app: &Router,
    method: Method,
    path: &str,
    body: Option<&Value>,
    auth: Option<&str>,
) -> Response {
    let mut request = Request::builder().method(method).uri(path);
    if let Some(auth) = auth {
        request = request.header(AUTHORIZATION, format!("Basic {}", STANDARD.encode(auth)));
    }
    let body = match body {
        Some(body) => {
            request = request.header(CONTENT_TYPE, "application/json");
            Body::from(body.to_string())
        }
        None => Body::empty(),
    };
    app.clone()
        .oneshot(request.body(body).unwrap())
        .await
        .unwrap()
}

async fn json_body(response: Response) -> Value {
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

#[tokio::test]
async fn reports_and_notes_survive_restart_and_arrive_on_schedule() {
    let directory = TestDirectory::new();
    let clock = Arc::new(AtomicU64::new(1_000_000));
    let app = test_router(&directory, &clock);
    let auth = Some("ground:test:password");
    let seed = demo::seed();
    let events: Vec<_> = seed
        .events
        .into_iter()
        .filter(|event| seed.now_met_s - event.met_s <= 48.0 * 3600.0)
        .collect();
    let assessment = ahc_core::assess(&ahc_core::AssessInput {
        crew_id: seed.crew_id.into(),
        events: demo::events(),
        now_met_s: seed.now_met_s,
        kit: seed.kit,
        registry: None,
    });
    let report = json!({
        "id": "report-1", "crew_id": "cdr", "sent_met_s": seed.now_met_s,
        "sent_earth_ms": 1_000_000.0, "deliver_at_earth_ms": 1_020_000.0,
        "sbar": ahc_core::draft_sbar(&assessment, &ahc_core::registry()),
        "assessment": assessment, "events": events
    });
    let note = json!({ "crew_id": "cdr", "text": "Acknowledged.", "in_reply_to": "report-1", "delay_s": 20 });
    let mut wrong_crew = note.clone();
    wrong_crew["crew_id"] = json!("other");
    assert_eq!(
        request(&app, Method::POST, "/api/reports", Some(&report), None)
            .await
            .status(),
        StatusCode::CREATED
    );
    let unauthorized = request(
        &app,
        Method::GET,
        "/api/reports",
        None,
        Some("ground:wrong"),
    )
    .await;
    assert_eq!(unauthorized.status(), StatusCode::UNAUTHORIZED);
    assert!(unauthorized.headers().contains_key(WWW_AUTHENTICATE));
    assert_eq!(
        request(&app, Method::POST, "/api/notes", Some(&note), None)
            .await
            .status(),
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        request(&app, Method::POST, "/api/notes", Some(&note), auth)
            .await
            .status(),
        StatusCode::NOT_FOUND
    );

    drop(app);
    let app = test_router(&directory, &clock);
    assert_eq!(
        request(&app, Method::POST, "/api/reports", Some(&report), None)
            .await
            .status(),
        StatusCode::CREATED
    );
    let mut conflict = report.clone();
    conflict["deliver_at_earth_ms"] = json!(1_030_000);
    assert_eq!(
        request(&app, Method::POST, "/api/reports", Some(&conflict), None)
            .await
            .status(),
        StatusCode::CONFLICT
    );
    clock.store(1_019_999, Ordering::Relaxed);
    assert_eq!(
        json_body(request(&app, Method::GET, "/api/reports", None, auth).await).await,
        json!([])
    );
    clock.store(1_020_000, Ordering::Relaxed);
    let delivered = json_body(request(&app, Method::GET, "/api/reports", None, auth).await).await;
    assert_eq!(delivered, json!([report]));
    assert_eq!(
        request(&app, Method::POST, "/api/notes", Some(&wrong_crew), auth)
            .await
            .status(),
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        request(&app, Method::POST, "/api/notes", Some(&note), auth)
            .await
            .status(),
        StatusCode::CREATED
    );

    drop(app);
    let app = test_router(&directory, &clock);
    let inbox = "/api/notes?crew_id=cdr";
    clock.store(1_039_999, Ordering::Relaxed);
    assert_eq!(
        json_body(request(&app, Method::GET, inbox, None, None).await).await,
        json!([])
    );
    clock.store(1_040_000, Ordering::Relaxed);
    assert_eq!(
        json_body(request(&app, Method::GET, "/api/notes?crew_id=other", None, None).await).await,
        json!([])
    );
    let delivered = json_body(request(&app, Method::GET, inbox, None, None).await).await;
    let notes: Vec<Event> = serde_json::from_value(delivered.clone()).unwrap();
    assert_eq!(notes.len(), 1);
    assert!(!notes[0].id.is_empty());
    assert_eq!(notes[0].crew_id, "cdr");
    assert_eq!(notes[0].author, "ground:ground");
    assert_eq!(notes[0].met_s, seed.now_met_s + 40.0);
    assert_eq!(
        notes[0].earth_ms,
        demo::LAUNCH_EARTH_MS + notes[0].met_s * 1000.0
    );
    assert!(matches!(
        &notes[0].kind,
        EventKind::GroundNote { text, sent_earth_ms, in_reply_to }
            if text == "Acknowledged." && *sent_earth_ms == 1_020_000.0
                && in_reply_to.as_deref() == Some("report-1")
    ));
    assert_eq!(
        json_body(request(&app, Method::GET, inbox, None, None).await).await,
        delivered
    );
}

#[test]
fn reopens_after_an_interrupted_append() {
    let directory = TestDirectory::new();
    fs::create_dir(&directory.0).unwrap();
    let path = directory.0.join("messages.jsonl");
    fs::write(&path, b"1\n{\"interrupted\":").unwrap();
    let mut log = Log::<u32>::open(&path).unwrap();
    assert_eq!(log.entries, vec![1]);
    log.append(2).unwrap();
    drop(log);
    assert_eq!(Log::<u32>::open(&path).unwrap().entries, vec![1, 2]);
}

#[test]
fn data_directory_has_one_writer() {
    let directory = TestDirectory::new();
    let app = router(&directory.0, None, || 0.0).unwrap();
    assert!(router(&directory.0, None, || 0.0).is_err());
    drop(app);
    assert!(router(&directory.0, None, || 0.0).is_ok());
}
