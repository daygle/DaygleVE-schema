//! Durable control-plane operation records.

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::common::{ResourceId, Timestamp};

/// Lifecycle state of a durable host operation.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OperationStatus {
    Queued,
    Running,
    Succeeded,
    Failed,
    NeedsReview,
    Cancelled,
}

/// Requested reconciliation behavior.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReconciliationMode {
    /// Inspect host and persisted state without changing either.
    DryRun,
    /// Apply only explicitly supported, non-destructive repairs after approval.
    Repair,
}

/// Body for starting a reconciliation pass.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReconcileRequest {
    pub mode: ReconciliationMode,
    /// A completed dry-run operation ID authorizing a repair pass.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<ResourceId>,
    /// Preserve unmanaged resources in a quarantine record rather than adopting
    /// or deleting them. This must be true for repair requests.
    #[serde(default = "default_true")]
    pub quarantine_unmanaged: bool,
}

fn default_true() -> bool {
    true
}

/// Classification of a reconciliation finding.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReconciliationFindingKind {
    MissingFromHost,
    UnmanagedHost,
    StateDrift,
}

/// Lifecycle state of an unmanaged host resource held out of automatic adoption.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QuarantineStatus {
    Pending,
    Adopted,
    Released,
}

/// Explicit operator decision for a quarantined host resource.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QuarantineDecision {
    Adopt,
    Release,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QuarantineDecisionRequest {
    pub decision: QuarantineDecision,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// A durable quarantine record for an unmanaged host resource.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReconciliationQuarantineRecord {
    pub id: ResourceId,
    pub resource_type: String,
    pub host_id: String,
    pub message: String,
    pub status: QuarantineStatus,
    pub created_at: Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decided_at: Option<Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decided_by: Option<ResourceId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decision_message: Option<String>,
}

/// One persisted drift finding. Findings are informational during a dry run and
/// become the explicit input to an approved repair operation.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReconciliationFinding {
    pub resource_type: String,
    pub resource_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_id: Option<String>,
    pub kind: ReconciliationFindingKind,
    pub message: String,
    pub repairable: bool,
    pub destructive: bool,
}

/// A persisted record describing a host mutation or recovery event.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OperationRecord {
    pub id: ResourceId,
    /// Stable operation name, e.g. `vm.create` or `network.create_bridge`.
    pub kind: String,
    pub status: OperationStatus,
    /// Reconciliation mode, when this record represents a host reconciliation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reconciliation_mode: Option<ReconciliationMode>,
    /// Optional progress percentage for asynchronous jobs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_pct: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<ResourceId>,
    /// ID of the resource created or modified by this operation, populated
    /// once the operation reaches a terminal state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_id: Option<ResourceId>,
    pub created_at: Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<Timestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Discrepancies found during a reconciliation scan, e.g. orphaned records
    /// or stale states. Populated when the operation is a `host.reconcile` scan.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drift: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub findings: Option<Vec<ReconciliationFinding>>,
}
