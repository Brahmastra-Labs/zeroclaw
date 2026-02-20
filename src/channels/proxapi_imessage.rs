/// ProxApi iMessage channel — brahmastra-fork.
///
/// Delegates iMessage sending + listening to ProxApi's HTTP endpoints:
///   POST /v1/imessage/messages/text   — send a message
///   GET  /v1/imessage/messages/stream — SSE stream of incoming messages
///   GET  /v1/imessage/queue           — health check
///
/// This replaces the direct chat.db + AppleScript approach for deployments
/// where ProxApi manages the Mac's iMessage bridge.

use crate::channels::traits::{Channel, ChannelMessage, SendMessage};
use anyhow::{bail, Context, Result};
use async_trait::async_trait;
use futures_util::StreamExt;
use reqwest::Client;
use std::time::Duration;
use tokio::sync::mpsc;

/// iMessage channel that calls ProxApi HTTP endpoints instead of AppleScript.
///
/// brahmastra-fork: ProxApi-specific — not suitable for upstream.
#[derive(Clone)]
pub struct ProxApiIMessageChannel {
    /// Base URL of the ProxApi server (e.g. "http://localhost:3000")
    base_url: String,
    /// Bearer token for ProxApi authentication
    token: String,
    /// Allowed sender identities; "*" means any
    allowed_contacts: Vec<String>,
    client: Client,
}

impl ProxApiIMessageChannel {
    pub fn new(
        base_url: impl Into<String>,
        token: impl Into<String>,
        allowed_contacts: Vec<String>,
    ) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            token: token.into(),
            allowed_contacts,
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("reqwest client"),
        }
    }

    fn is_contact_allowed(&self, sender: &str) -> bool {
        self.allowed_contacts.iter().any(|c| c == "*" || c.eq_ignore_ascii_case(sender))
    }

    /// Probe the ProxApi iMessage queue endpoint.
    /// Returns `true` if the endpoint returns 2xx, `false` otherwise.
    /// Distinct from the Channel trait's `health_check` so callers can propagate errors.
    pub async fn probe_health(&self) -> Result<bool> {
        let url = format!("{}/v1/imessage/queue", self.base_url);
        match self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await
        {
            Ok(resp) => Ok(resp.status().is_success()),
            Err(e) => Err(e).context("ProxApi iMessage health probe failed"),
        }
    }
}

#[async_trait]
impl Channel for ProxApiIMessageChannel {
    fn name(&self) -> &str {
        "imessage-proxapi"
    }

    /// Send a message via ProxApi's POST /v1/imessage/messages/text endpoint.
    async fn send(&self, message: &SendMessage) -> Result<()> {
        let url = format!("{}/v1/imessage/messages/text", self.base_url);
        let body = serde_json::json!({
            "recipient": message.recipient,
            "text": message.content,
        });

        let resp = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&body)
            .send()
            .await
            .context("ProxApi iMessage send request failed")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            bail!("ProxApi iMessage send returned {status}: {text}");
        }

        Ok(())
    }

    /// Listen for incoming messages via ProxApi's SSE stream endpoint.
    ///
    /// Connects to GET /v1/imessage/messages/stream and forwards parsed events to
    /// the `tx` channel.  On disconnect: exponential backoff (1s → 2s → 4s → max 60s).
    async fn listen(&self, tx: mpsc::Sender<ChannelMessage>) -> Result<()> {
        let url = format!("{}/v1/imessage/messages/stream", self.base_url);
        let mut backoff_secs: u64 = 1;

        loop {
            let resp = match self
                .client
                .get(&url)
                .header("Authorization", format!("Bearer {}", self.token))
                .header("Accept", "text/event-stream")
                .send()
                .await
            {
                Ok(r) if r.status().is_success() => r,
                Ok(r) => {
                    tracing::warn!(
                        status = %r.status(),
                        "ProxApi iMessage SSE: non-2xx, backing off {}s", backoff_secs
                    );
                    tokio::time::sleep(Duration::from_secs(backoff_secs)).await;
                    backoff_secs = (backoff_secs * 2).min(60);
                    continue;
                }
                Err(e) => {
                    tracing::warn!(
                        error = %e,
                        "ProxApi iMessage SSE connect failed, backing off {}s", backoff_secs
                    );
                    tokio::time::sleep(Duration::from_secs(backoff_secs)).await;
                    backoff_secs = (backoff_secs * 2).min(60);
                    continue;
                }
            };

            // Reset backoff on successful connect
            backoff_secs = 1;

            let mut stream = resp.bytes_stream();
            let mut buffer = String::new();
            let mut event_data = String::new();

            while let Some(chunk) = stream.next().await {
                let bytes = match chunk {
                    Ok(b) => b,
                    Err(e) => {
                        tracing::warn!(error = %e, "ProxApi iMessage SSE stream error");
                        break;
                    }
                };
                buffer.push_str(&String::from_utf8_lossy(&bytes));

                // Process complete lines
                while let Some(pos) = buffer.find('\n') {
                    let line = buffer[..pos].trim_end_matches('\r').to_string();
                    buffer = buffer[pos + 1..].to_string();

                    if let Some(data) = line.strip_prefix("data:") {
                        event_data = data.trim().to_string();
                    } else if line.is_empty() && !event_data.is_empty() {
                        // End of SSE event — parse JSON payload
                        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&event_data) {
                            let sender = val["sender"].as_str().unwrap_or("").to_string();
                            let content = val["text"]
                                .as_str()
                                .or_else(|| val["content"].as_str())
                                .unwrap_or("")
                                .to_string();
                            let id = val["id"].as_str().unwrap_or("").to_string();

                            if !sender.is_empty()
                                && !content.is_empty()
                                && self.is_contact_allowed(&sender)
                            {
                                let msg = ChannelMessage {
                                    id,
                                    sender: sender.clone(),
                                    reply_target: sender,
                                    content,
                                    channel: "imessage".to_string(),
                                    timestamp: std::time::SystemTime::now()
                                        .duration_since(std::time::UNIX_EPOCH)
                                        .map(|d| d.as_secs())
                                        .unwrap_or(0),
                                };
                                if tx.send(msg).await.is_err() {
                                    return Ok(());
                                }
                            }
                        }
                        event_data.clear();
                    }
                }
            }

            tracing::info!(
                "ProxApi iMessage SSE disconnected, reconnecting in {}s", backoff_secs
            );
            tokio::time::sleep(Duration::from_secs(backoff_secs)).await;
            backoff_secs = (backoff_secs * 2).min(60);
        }
    }

    /// Check if ProxApi's iMessage endpoint is reachable.
    /// Returns `true` on 2xx, `false` on any error or non-2xx.
    async fn health_check(&self) -> bool {
        self.probe_health().await.unwrap_or(false)
    }
}
