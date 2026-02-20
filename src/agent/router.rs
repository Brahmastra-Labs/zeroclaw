/// Multi-agent routing — brahmastra-fork.
///
/// `AgentRouter` resolves inbound messages to named agents based on channel + sender
/// binding rules defined in `RoutingConfig`.  In `RoutingMode::Single` the router
/// always returns the configured default agent name (zero behavior change).

use crate::config::schema::{BindingConfig, RoutingConfig, RoutingMode};

/// Lightweight routing table — resolves (channel, sender) → agent name.
///
/// Does NOT hold provider/memory handles; the gateway looks up those by agent
/// name from its own AppState. This keeps the router cheaply cloneable and
/// avoids coupling the config layer to async runtime types.
#[derive(Debug, Clone)]
pub struct AgentRouter {
    mode: RoutingMode,
    bindings: Vec<BindingConfig>,
    default_agent: String,
}

impl AgentRouter {
    /// Build a router from a `RoutingConfig`.
    ///
    /// `default_agent`:
    /// - If `cfg.default_agent` is set, use it.
    /// - Otherwise in Named mode use the first agent's name (if any).
    /// - Otherwise use "default".
    pub fn from_routing_config(cfg: &RoutingConfig) -> Self {
        let default_agent = cfg
            .default_agent
            .clone()
            .or_else(|| cfg.agents.first().map(|a| a.name.clone()))
            .unwrap_or_else(|| "default".to_string());

        Self {
            mode: cfg.mode.clone(),
            bindings: cfg.bindings.clone(),
            default_agent,
        }
    }

    /// Resolve the agent name for an inbound message.
    ///
    /// Single mode: always returns the default agent.
    /// Named mode: evaluates bindings in order, first match wins; falls back to default.
    ///
    /// Binding match rules:
    /// - `channel` field: exact match, or `"*"` / `None` matches any.
    /// - `account` field: exact match, or `"*"` / `None` matches any.
    pub fn resolve_agent<'a>(&'a self, channel: &str, sender: &str) -> &'a str {
        if self.mode == RoutingMode::Single {
            return &self.default_agent;
        }

        for binding in &self.bindings {
            let ch_match = match &binding.channel {
                None => true,
                Some(c) => c == "*" || c == channel,
            };
            let acc_match = match &binding.account {
                None => true,
                Some(a) => a == "*" || a == sender,
            };
            if ch_match && acc_match {
                return &binding.agent;
            }
        }

        &self.default_agent
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::schema::{BindingConfig, NamedAgentConfig, RoutingConfig, RoutingMode};

    fn make_agent(name: &str) -> NamedAgentConfig {
        NamedAgentConfig {
            name: name.to_string(),
            provider: "mock".to_string(),
            model: "mock-model".to_string(),
            system_prompt: None,
            api_key: None,
            api_key_env: None,
            workspace_dir: None,
            session_store: None,
            temperature: None,
        }
    }

    #[test]
    fn single_mode_always_returns_default() {
        let cfg = RoutingConfig {
            mode: RoutingMode::Single,
            agents: vec![make_agent("only")],
            bindings: vec![],
            default_agent: Some("only".to_string()),
        };
        let router = AgentRouter::from_routing_config(&cfg);
        assert_eq!(router.resolve_agent("imessage", "+1234567890"), "only");
        assert_eq!(router.resolve_agent("discord", "user-abc"), "only");
    }

    #[test]
    fn named_mode_routes_by_binding() {
        let cfg = RoutingConfig {
            mode: RoutingMode::Named,
            agents: vec![make_agent("a"), make_agent("b")],
            bindings: vec![
                BindingConfig {
                    channel: Some("imessage".to_string()),
                    account: Some("+11111".to_string()),
                    agent: "a".to_string(),
                },
                BindingConfig {
                    channel: Some("imessage".to_string()),
                    account: Some("+22222".to_string()),
                    agent: "b".to_string(),
                },
            ],
            default_agent: Some("a".to_string()),
        };
        let router = AgentRouter::from_routing_config(&cfg);
        assert_eq!(router.resolve_agent("imessage", "+11111"), "a");
        assert_eq!(router.resolve_agent("imessage", "+22222"), "b");
    }

    #[test]
    fn named_mode_wildcard_fallback() {
        let cfg = RoutingConfig {
            mode: RoutingMode::Named,
            agents: vec![make_agent("catch-all")],
            bindings: vec![BindingConfig {
                channel: Some("*".to_string()),
                account: Some("*".to_string()),
                agent: "catch-all".to_string(),
            }],
            default_agent: None,
        };
        let router = AgentRouter::from_routing_config(&cfg);
        assert_eq!(router.resolve_agent("slack", "user-xyz"), "catch-all");
        assert_eq!(router.resolve_agent("discord", "other"), "catch-all");
    }

    #[test]
    fn named_mode_falls_back_to_default_when_no_binding_matches() {
        let cfg = RoutingConfig {
            mode: RoutingMode::Named,
            agents: vec![make_agent("primary"), make_agent("secondary")],
            bindings: vec![BindingConfig {
                channel: Some("imessage".to_string()),
                account: Some("+99999".to_string()),
                agent: "secondary".to_string(),
            }],
            default_agent: Some("primary".to_string()),
        };
        let router = AgentRouter::from_routing_config(&cfg);
        // Unknown sender → falls back to primary
        assert_eq!(router.resolve_agent("imessage", "+00000"), "primary");
        // Unknown channel → falls back to primary
        assert_eq!(router.resolve_agent("telegram", "+99999"), "primary");
    }
}
