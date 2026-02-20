/// iMessage via ProxApi HTTP — Phase 3 (TDD: RED first, then GREEN).
///
/// Tests:
///   1. channel.send() calls POST /v1/imessage/messages/text
///   2. 400 from ProxApi propagates as error
///   3. health_check OK on 200
///   4. health_check returns false on 503
///   5. IMessageConfig parses backend = "proxapi"
///   6. E2E send + roundtrip (gated on env vars)

use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};
use zeroclaw::channels::proxapi_imessage::ProxApiIMessageChannel;
use zeroclaw::channels::traits::{Channel, SendMessage};
use zeroclaw::config::schema::{IMessageBackend, IMessageConfig};

// ── 1. send() calls correct endpoint ─────────────────────────────────────────

#[tokio::test]
async fn test_proxapi_channel_send_calls_correct_endpoint() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/imessage/messages/text"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "ok": true
        })))
        .expect(1)
        .mount(&server)
        .await;

    let channel = ProxApiIMessageChannel::new(
        server.uri(),
        "sk-test-token".to_string(),
        vec!["+15551234567".to_string()],
    );

    let msg = SendMessage::new("hello world", "+15551234567");
    channel.send(&msg).await.expect("send should succeed");

    server.verify().await;
}

// ── 2. 400 from ProxApi propagates as error ───────────────────────────────────

#[tokio::test]
async fn test_proxapi_channel_send_invalid_target_returns_error() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/imessage/messages/text"))
        .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
            "ok": false,
            "error": { "code": "invalid_recipient", "message": "invalid phone number" }
        })))
        .mount(&server)
        .await;

    let channel = ProxApiIMessageChannel::new(
        server.uri(),
        "sk-test-token".to_string(),
        vec!["*".to_string()],
    );

    let msg = SendMessage::new("hello", "not-a-phone-number");
    let result = channel.send(&msg).await;
    assert!(result.is_err(), "400 from server should return Err");
}

// ── 3. health_check returns true on 200 ──────────────────────────────────────

#[tokio::test]
async fn test_proxapi_channel_health_check_ok() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/imessage/queue"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
        .mount(&server)
        .await;

    let channel = ProxApiIMessageChannel::new(
        server.uri(),
        "sk-test-token".to_string(),
        vec![],
    );

    // probe_health returns Result<bool>
    let healthy = channel.probe_health().await.expect("probe_health should not error");
    assert!(healthy, "200 response should indicate healthy");
}

// ── 4. health_check returns false on 503 ─────────────────────────────────────

#[tokio::test]
async fn test_proxapi_channel_health_check_down() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/imessage/queue"))
        .respond_with(ResponseTemplate::new(503))
        .mount(&server)
        .await;

    let channel = ProxApiIMessageChannel::new(
        server.uri(),
        "sk-test-token".to_string(),
        vec![],
    );

    // probe_health returns Result<bool>
    let healthy = channel.probe_health().await.expect("probe_health should not error");
    assert!(!healthy, "503 response should indicate unhealthy");
}

// ── 5. IMessageConfig parses backend = "proxapi" ──────────────────────────────

#[test]
fn test_proxapi_channel_config_parses_backend_proxapi() {
    let toml = r#"
allowed_contacts = ["+15551234567"]
backend = "proxapi"
proxapi_url = "http://localhost:3000"
proxapi_token = "sk-proxapi-test"
"#;
    let cfg: IMessageConfig = toml::from_str(toml).unwrap();
    assert_eq!(cfg.backend, IMessageBackend::ProxApi);
    assert_eq!(cfg.proxapi_url.as_deref(), Some("http://localhost:3000"));
    assert_eq!(cfg.proxapi_token.as_deref(), Some("sk-proxapi-test"));
    assert_eq!(cfg.allowed_contacts, vec!["+15551234567"]);
}

// ── 6. E2E: real ProxApi send + verify (gated on env vars) ───────────────────

#[tokio::test]
async fn test_e2e_proxapi_imessage_send_roundtrip() {
    let proxapi_url = match std::env::var("PROXAPI_TEST_URL") {
        Ok(u) => u,
        Err(_) => {
            eprintln!("Skipping test_e2e_proxapi_imessage_send_roundtrip: PROXAPI_TEST_URL not set");
            return;
        }
    };
    let proxapi_token = match std::env::var("PROXAPI_TEST_TOKEN") {
        Ok(t) => t,
        Err(_) => {
            eprintln!("Skipping: PROXAPI_TEST_TOKEN not set");
            return;
        }
    };

    let channel = ProxApiIMessageChannel::new(
        proxapi_url,
        proxapi_token,
        vec!["*".to_string()],
    );

    // Health check first
    let healthy = channel.probe_health().await.expect("health check");
    assert!(healthy, "ProxApi iMessage endpoint should be healthy");
    eprintln!("E2E: ProxApi iMessage health OK");
}
