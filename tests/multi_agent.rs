/// Multi-agent routing tests — Phase 2 (TDD: RED first, then GREEN).
///
/// Tests:
///   1. Config: routing config parses named mode
///   2. Config: defaults to single mode (backward-compatible)
///   3. Config: binding resolves channel + sender
///   4. Config: wildcard fallback
///   5. AgentRouter: routes to correct agent
///   6. AgentRouter: single mode always dispatches to default
///   7. E2E: two agents have isolated memory (gated on ZEROCLAW_BINARY_PATH)

use zeroclaw::config::{
    schema::{BindingConfig, NamedAgentConfig, RoutingConfig, RoutingMode},
    Config,
};
use zeroclaw::agent::router::AgentRouter;

// ── 1. Config: named mode round-trips via serde ───────────────────────────────

#[test]
fn test_routing_config_parses_named_mode() {
    // Parse RoutingConfig directly (without the [routing] wrapper)
    let toml = r#"
mode = "named"

[[agents]]
name = "rocco"
provider = "openrouter"
model = "meta-llama/llama-3.1-8b-instruct"
workspace_dir = "/tmp/rocco"

[[agents]]
name = "jasmine"
provider = "openrouter"
model = "anthropic/claude-3-haiku"
workspace_dir = "/tmp/jasmine"
"#;

    let cfg: RoutingConfig = toml::from_str(toml).unwrap();
    assert_eq!(cfg.mode, RoutingMode::Named);
    assert_eq!(cfg.agents.len(), 2);
    assert_eq!(cfg.agents[0].name, "rocco");
    assert_eq!(cfg.agents[0].provider, "openrouter");
    assert_eq!(cfg.agents[0].model, "meta-llama/llama-3.1-8b-instruct");
    assert_eq!(cfg.agents[1].name, "jasmine");
    assert_eq!(cfg.agents[1].model, "anthropic/claude-3-haiku");
}

// ── 2. Config: empty → RoutingMode::Single (backward-compatible) ─────────────

#[test]
fn test_routing_config_defaults_to_single_mode() {
    // When no [routing] section is present, Config.routing should be None or Single.
    let toml = r#"
[agent]
name = "test"

[gateway]
port = 3000
host = "127.0.0.1"
"#;
    // Parse as a minimal config — routing should be absent / default to Single.
    let cfg: toml::Value = toml::from_str(toml).unwrap();
    assert!(
        cfg.get("routing").is_none(),
        "routing key should be absent from minimal config"
    );

    // Explicit Single mode round-trips.
    let routing_toml = r#"
mode = "single"
"#;
    let rc: RoutingConfig = toml::from_str(routing_toml).unwrap();
    assert_eq!(rc.mode, RoutingMode::Single);
    assert!(rc.agents.is_empty(), "single mode should have no named agents");
}

// ── 3. Config: binding resolves channel + sender ──────────────────────────────

#[test]
fn test_routing_binding_resolves_channel_and_sender() {
    let toml = r#"
mode = "named"

[[agents]]
name = "rocco"
provider = "openrouter"
model = "meta-llama/llama-3.1-8b-instruct"

[[agents]]
name = "jasmine"
provider = "openrouter"
model = "anthropic/claude-3-haiku"

[[bindings]]
channel = "imessage"
account = "+15551234567"
agent = "rocco"

[[bindings]]
channel = "imessage"
account = "+15559999999"
agent = "jasmine"
"#;
    let cfg: RoutingConfig = toml::from_str(toml).unwrap();
    assert_eq!(cfg.bindings.len(), 2);

    let b = &cfg.bindings[0];
    assert_eq!(b.channel.as_deref(), Some("imessage"));
    assert_eq!(b.account.as_deref(), Some("+15551234567"));
    assert_eq!(b.agent, "rocco");
}

// ── 4. Config: wildcard fallback binding ─────────────────────────────────────

#[test]
fn test_routing_binding_wildcard_fallback() {
    let toml = r#"
mode = "named"

[[agents]]
name = "default-agent"
provider = "openrouter"
model = "meta-llama/llama-3.1-8b-instruct"

[[bindings]]
channel = "*"
account = "*"
agent = "default-agent"
"#;
    let cfg: RoutingConfig = toml::from_str(toml).unwrap();
    assert_eq!(cfg.bindings.len(), 1);
    let b = &cfg.bindings[0];
    assert_eq!(b.channel.as_deref(), Some("*"));
    assert_eq!(b.account.as_deref(), Some("*"));
    assert_eq!(b.agent, "default-agent");
}

// ── 5. AgentRouter: routes to correct agent by channel + sender ───────────────

#[test]
fn test_agent_router_routes_to_correct_agent() {
    // Build a RoutingConfig with two agents and bindings.
    let cfg = RoutingConfig {
        mode: RoutingMode::Named,
        agents: vec![
            NamedAgentConfig {
                name: "agent-a".to_string(),
                provider: "mock".to_string(),
                model: "model-a".to_string(),
                system_prompt: None,
                api_key: None,
                api_key_env: None,
                workspace_dir: None,
                session_store: None,
                temperature: None,
            },
            NamedAgentConfig {
                name: "agent-b".to_string(),
                provider: "mock".to_string(),
                model: "model-b".to_string(),
                system_prompt: None,
                api_key: None,
                api_key_env: None,
                workspace_dir: None,
                session_store: None,
                temperature: None,
            },
        ],
        bindings: vec![
            BindingConfig {
                channel: Some("imessage".to_string()),
                account: Some("+11111111111".to_string()),
                agent: "agent-a".to_string(),
            },
            BindingConfig {
                channel: Some("imessage".to_string()),
                account: Some("+12222222222".to_string()),
                agent: "agent-b".to_string(),
            },
        ],
        default_agent: None,
    };

    let router = AgentRouter::from_routing_config(&cfg);

    let resolved_a = router.resolve_agent("imessage", "+11111111111");
    let resolved_b = router.resolve_agent("imessage", "+12222222222");

    assert_eq!(resolved_a, "agent-a");
    assert_eq!(resolved_b, "agent-b");
}

// ── 6. AgentRouter: single mode always resolves to the default ────────────────

#[test]
fn test_agent_router_single_mode_uses_default() {
    let cfg = RoutingConfig {
        mode: RoutingMode::Single,
        agents: vec![],
        bindings: vec![],
        default_agent: Some("sole-agent".to_string()),
    };

    let router = AgentRouter::from_routing_config(&cfg);

    // Single mode: any channel/sender resolves to the configured default.
    let r1 = router.resolve_agent("imessage", "+15551111111");
    let r2 = router.resolve_agent("discord", "user-xyz");
    let r3 = router.resolve_agent("*", "*");

    assert_eq!(r1, "sole-agent");
    assert_eq!(r2, "sole-agent");
    assert_eq!(r3, "sole-agent");
}

// ── 7. E2E: two agents have isolated memory ───────────────────────────────────
//
// Gated on ZEROCLAW_BINARY_PATH — skipped in CI unless set.

#[test]
fn test_e2e_two_agents_isolated_memory() {
    let binary = match std::env::var("ZEROCLAW_BINARY_PATH") {
        Ok(p) => p,
        Err(_) => {
            eprintln!("Skipping test_e2e_two_agents_isolated_memory: ZEROCLAW_BINARY_PATH not set");
            return;
        }
    };

    // Verify binary exists
    assert!(
        std::path::Path::new(&binary).exists(),
        "ZEROCLAW_BINARY_PATH={binary} does not exist"
    );

    // This test would:
    // 1. Create two temp dirs for agent workspaces
    // 2. Spawn zeroclaw with a named routing config pointing to both workspaces
    // 3. Send a message to agent-a, send a different message to agent-b
    // 4. Verify agent-a's memory doesn't contain agent-b's message and vice versa
    //
    // For now, this is a placeholder — implementation follows after GREEN.
    eprintln!("E2E multi-agent isolation test: binary found at {binary}");
    eprintln!("Full E2E implementation pending gateway integration (Phase 2 GREEN)");
}
