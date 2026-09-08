//! Notification channels: deliver alerts about node events over email (SMTP)
//! or an outbound webhook.
//!
//! A channel subscribes to a set of [`NotificationEvent`]s and, when one fires,
//! the backend delivers a short message through the channel's transport. Secrets
//! (an SMTP password or a webhook signing secret) are write-only: they are set
//! through the create/update requests and never returned — a channel only
//! reports whether one is stored via `has_secret`.

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::common::{ResourceId, Timestamp};

/// Transport a channel delivers through.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NotificationChannelKind {
    Email,
    Webhook,
}

/// A node event a channel can subscribe to.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NotificationEvent {
    /// A backup completed successfully.
    BackupSucceeded,
    /// A backup run failed.
    BackupFailed,
    /// A scheduled snapshot failed to capture.
    SnapshotFailed,
    /// A scheduled power action failed to apply.
    PowerActionFailed,
    /// A manual test message (from the "send test" action).
    Test,
}

/// SMTP delivery settings (no password — that is the channel secret).
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EmailSettings {
    pub smtp_host: String,
    pub smtp_port: u16,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smtp_username: Option<String>,
    /// Envelope/from address.
    pub from_address: String,
    /// One or more recipient addresses.
    pub to_addresses: Vec<String>,
    /// Use STARTTLS on the SMTP connection.
    #[serde(default)]
    pub starttls: bool,
}

/// Outbound webhook settings (no secret — that is the channel secret).
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WebhookSettings {
    /// HTTPS endpoint that receives a JSON `POST` for each event.
    pub url: String,
}

/// A configured notification channel.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NotificationChannel {
    pub id: ResourceId,
    pub name: String,
    pub kind: NotificationChannelKind,
    pub enabled: bool,
    /// Events this channel is notified about.
    pub events: Vec<NotificationEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<EmailSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<WebhookSettings>,
    /// Whether a secret (SMTP password / webhook signing secret) is stored. The
    /// secret itself is never returned.
    pub has_secret: bool,
    pub created_at: Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
}

/// Body for `POST /api/v1/notifications`.
///
/// `Debug` is hand-written so the `secret` is never printed.
#[typeshare]
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateNotificationChannelRequest {
    pub name: String,
    pub kind: NotificationChannelKind,
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub events: Vec<NotificationEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<EmailSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<WebhookSettings>,
    /// SMTP password (email) or webhook HMAC signing secret (webhook).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

impl std::fmt::Debug for CreateNotificationChannelRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreateNotificationChannelRequest")
            .field("name", &self.name)
            .field("kind", &self.kind)
            .field("enabled", &self.enabled)
            .field("events", &self.events)
            .field("email", &self.email)
            .field("webhook", &self.webhook)
            .field("secret", &self.secret.as_ref().map(|_| "<redacted>"))
            .finish()
    }
}

fn default_true() -> bool {
    true
}

/// Body for `PATCH /api/v1/notifications/{id}`. Only present fields change; the
/// channel kind is immutable. `secret: Some("")` clears the stored secret.
///
/// `Debug` is hand-written so the `secret` is never printed.
#[typeshare]
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateNotificationChannelRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<NotificationEvent>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<EmailSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<WebhookSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}

impl std::fmt::Debug for UpdateNotificationChannelRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UpdateNotificationChannelRequest")
            .field("name", &self.name)
            .field("enabled", &self.enabled)
            .field("events", &self.events)
            .field("email", &self.email)
            .field("webhook", &self.webhook)
            .field("secret", &self.secret.as_ref().map(|_| "<redacted>"))
            .finish()
    }
}
