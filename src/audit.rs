//! Audit-log types: a persisted, append-only record of security-relevant
//! actions on the control plane.
//!
//! Events are emitted by the backend at the API boundary (where the acting
//! caller is known) for authentication, account and role changes, API-token and
//! TLS/ACME operations, and other sensitive mutations. The log is read-only
//! over the API — there is no endpoint to create or alter entries — and is
//! surfaced newest-first to administrators.

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::common::{ResourceId, Timestamp};

/// Whether the audited action succeeded or was rejected/failed.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AuditOutcome {
    Success,
    Failure,
}

/// One recorded action in the audit log.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuditEvent {
    pub id: ResourceId,
    /// When the action occurred.
    pub at: Timestamp,
    /// The account that performed the action, by id. Absent for actions with no
    /// authenticated caller (e.g. a failed login, or a system-initiated task).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor_id: Option<ResourceId>,
    /// Display name of the actor: the username, the attempted username on a
    /// failed login, or `system`.
    pub actor: String,
    /// Stable, machine-readable action key, e.g. `auth.login`,
    /// `user.create`, `api_token.revoke`, `acme.config.update`.
    pub action: String,
    /// The kind of resource acted upon, when applicable (e.g. `user`, `vm`,
    /// `api_token`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The id or name of the resource acted upon, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    pub outcome: AuditOutcome,
    /// Human-readable detail, free of secrets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Client IP the action came from, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_ip: Option<String>,
}
