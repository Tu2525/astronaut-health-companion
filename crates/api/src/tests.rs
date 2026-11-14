use axum::{
    body::{to_bytes, Body},
    http::{
        header::{
            ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
            ACCESS_CONTROL_ALLOW_ORIGIN, ACCESS_CONTROL_REQUEST_HEADERS,
            ACCESS_CONTROL_REQUEST_METHOD, ORIGIN,
        },
        Request,
    },
    response::Response,
};
use tower::ServiceExt;

use super::*;

async fn read_json(response: Response) -> Value {
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()[CONTENT_TYPE], "application/json");
    let bytes = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

async fn post_assessment(body: String) -> Response {
    router()
        .oneshot(
            Request::post("/api/assess")
                .header(CONTENT_TYPE, "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap()
}

#[tokio::test]
async fn serves_health_and_registry() {
    let registry_json = serde_json::to_string(&ahc_core::registry()).unwrap();
    for (path, expected) in [
        ("/api/health", json!({ "ok": true })),
        (
            "/api/registry",
            serde_json::from_str(&registry_json).unwrap(),
        ),
    ] {
        let response = router()
            .oneshot(Request::get(path).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(read_json(response).await, expected);
    }
}

#[tokio::test]
async fn assesses_the_served_demo_using_the_shared_engine() {
    let response = router()
        .oneshot(Request::get("/api/demo").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let demo = read_json(response).await;
    let seed_json = serde_json::to_string(&demo::seed()).unwrap();
    assert_eq!(demo, serde_json::from_str::<Value>(&seed_json).unwrap());

    let input = json!({
        "crew_id": demo["crew_id"],
        "now_met_s": demo["now_met_s"],
        "events": demo["events"],
        "kit": demo["kit"],
        "registry": null,
    })
    .to_string();
    let expected: Value = serde_json::from_str(&ahc_core::assess_json(&input).unwrap()).unwrap();
    let actual = read_json(post_assessment(input).await).await;
    assert_eq!(actual, expected);
    assert_eq!(actual["overall"], "watch");
    assert_eq!(actual["escalation"], "monitor");
}

#[tokio::test]
async fn rejects_malformed_requests_and_indicators_without_ranges() {
    let response = post_assessment("{".to_owned()).await;
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let mut registry = ahc_core::registry();
    registry[0].ranges.clear();
    let input = json!({
        "crew_id": "cdr",
        "now_met_s": 0,
        "events": [],
        "kit": [],
        "registry": registry,
    });
    let response = post_assessment(input.to_string()).await;
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn allows_json_requests_from_a_browser_origin() {
    let response = router()
        .oneshot(
            Request::builder()
                .method(Method::OPTIONS)
                .uri("/api/assess")
                .header(ORIGIN, "https://crew.example")
                .header(ACCESS_CONTROL_REQUEST_METHOD, "POST")
                .header(ACCESS_CONTROL_REQUEST_HEADERS, "content-type")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert!(response.status().is_success());
    assert_eq!(response.headers()[ACCESS_CONTROL_ALLOW_ORIGIN], "*");
    assert!(response.headers()[ACCESS_CONTROL_ALLOW_METHODS]
        .to_str()
        .unwrap()
        .split(',')
        .any(|method| method.trim() == "POST"));
    assert_eq!(
        response.headers()[ACCESS_CONTROL_ALLOW_HEADERS],
        "content-type"
    );
}
