//! Notification channels: deliver alerts about node events over email (SMTP)
//! or an outbound webhook.
//!
//! A channel subscribes to a set of [`NotificationEvent`]s and, when one fires,
//! the backend delivers a short message through the channel's transport. Secrets
//! (an SMTP password or a webhook signing secret) are write-only: they are set
//! through the create/update requests and never returned - a channel only
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
    /// A metric-threshold alert rule fired.
    ThresholdBreached,
    /// A manual test message (from the "send test" action).
    Test,
}

/// SMTP delivery settings (no password - that is the channel secret).
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

/// Outbound webhook settings (no secret - that is the channel secret).
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

/// Metric a threshold alert rule watches.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AlertMetric {
    /// Guest or node CPU utilisation, 0-100.
    CpuPct,
    /// Guest memory usage relative to its limit, 0-100. Node rules compare
    /// against total physical memory.
    MemoryPct,
    /// Guest disk read throughput in MiB/s (aggregate across disks).
    DiskReadMibS,
    /// Guest disk write throughput in MiB/s (aggregate across disks).
    DiskWriteMibS,
    /// Guest network receive throughput in MiB/s.
    NetRxMibS,
    /// Guest network transmit throughput in MiB/s.
    NetTxMibS,
    /// ZFS pool allocated capacity, 0-100. Scope is the node; `guest_id`
    /// selects the pool name when set.
    PoolUsedPct,
}

/// Which sample source a rule evaluates against.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AlertScope {
    /// Node-wide metrics and pool capacity.
    Node,
    /// A single VM (all VMs when `guest_id` is unset).
    Vm,
    /// A single container (all containers when `guest_id` is unset).
    Lxc,
}

/// A metric-threshold alert rule: notify when a metric crosses a threshold and
/// stays there for a sustained window. Rules are evaluated on the background
/// metrics tick; a rule that fires re-notifies only after its cooldown passes.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AlertRule {
    pub id: ResourceId,
    pub name: String,
    pub enabled: bool,
    pub scope: AlertScope,
    /// Restrict the rule to one guest (VM/container id) or pool name. Unset
    /// means the rule applies to every guest of the scope.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guest_id: Option<ResourceId>,
    pub metric: AlertMetric,
    /// Fire when the observed value is `>=` this threshold. Units depend on
    /// `metric` (percent for `*_pct`, MiB/s for throughput).
    pub threshold: f64,
    /// The metric must stay at or above the threshold this many consecutive
    /// evaluation ticks before the rule fires. 0 or 1 fires on the first
    /// breach.
    #[serde(default)]
    pub sustain_ticks: u32,
    /// Minimum seconds between two notifications for the same rule (and guest).
    #[serde(default = "default_cooldown_secs")]
    pub cooldown_secs: u64,
    pub created_at: Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
}

fn default_cooldown_secs() -> u64 {
    300
}

/// Body for `POST /api/v1/alerts`.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateAlertRuleRequest {
    pub name: String,
    #[serde(default = "default_rule_enabled")]
    pub enabled: bool,
    pub scope: AlertScope,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guest_id: Option<ResourceId>,
    pub metric: AlertMetric,
    pub threshold: f64,
    #[serde(default)]
    pub sustain_ticks: u32,
    #[serde(default = "default_cooldown_secs")]
    pub cooldown_secs: u64,
}

fn default_rule_enabled() -> bool {
    true
}

/// Body for `PATCH /api/v1/alerts/{id}`. Only present fields change.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateAlertRuleRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guest_id: Option<Option<ResourceId>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric: Option<AlertMetric>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sustain_ticks: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cooldown_secs: Option<u64>,
}
