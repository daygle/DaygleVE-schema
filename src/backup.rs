//! Backup and restore contracts.

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::common::{ResourceId, Timestamp};

/// Resource kind captured by a backup plan.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BackupSourceType {
    Vm,
    Container,
    Dataset,
}

/// A durable scheduled backup policy.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BackupPlan {
    pub id: ResourceId,
    pub name: String,
    pub source_type: BackupSourceType,
    /// VM/container resource id, or a validated ZFS dataset name.
    pub source_id: String,
    /// Relative directory below the configured backup root.
    pub destination: String,
    /// Interval in seconds. `None` means manual-only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval_secs: Option<u64>,
    /// Keep the newest N successful artifacts.
    pub retention_count: u32,
    pub verify: bool,
    pub enabled: bool,
    pub created_at: Timestamp,
    pub updated_at: Option<Timestamp>,
    pub last_run_at: Option<Timestamp>,
    pub next_run_at: Option<Timestamp>,
}

/// Body for `POST /api/v1/backups/plans`.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateBackupPlanRequest {
    pub name: String,
    pub source_type: BackupSourceType,
    pub source_id: String,
    /// Relative destination below `DAYGLEVE_BACKUP_DIR`.
    #[serde(default = "default_destination")]
    pub destination: String,
    /// Minimum interval is one minute when supplied.
    pub interval_secs: Option<u64>,
    /// Must be at least one; defaults to seven artifacts.
    #[serde(default = "default_retention")]
    pub retention_count: u32,
    #[serde(default = "default_true")]
    pub verify: bool,
    #[serde(default = "default_true")]
    pub enabled: bool,
}

/// Body for `PATCH /api/v1/backups/plans/{id}`.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateBackupPlanRequest {
    pub interval_secs: Option<u64>,
    pub retention_count: Option<u32>,
    pub verify: Option<bool>,
    pub enabled: Option<bool>,
}

/// One ZFS send stream in a backup artifact.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BackupFile {
    pub dataset: String,
    pub snapshot: String,
    /// Absolute host path is returned for operator visibility; it is never
    /// accepted as a command argument from a client.
    pub path: String,
    pub size_bytes: u64,
    pub sha256: String,
}

/// A completed, verified-or-explicitly-unverified backup.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BackupArtifact {
    pub id: ResourceId,
    pub plan_id: ResourceId,
    pub source_type: BackupSourceType,
    pub source_id: String,
    pub created_at: Timestamp,
    pub files: Vec<BackupFile>,
    pub total_size_bytes: u64,
    pub verified: bool,
}

/// Body for `POST /api/v1/backups/artifacts/{id}/restore`.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RestoreBackupRequest {
    /// Required for dataset backups; for guests this must match the source
    /// resource id unless a future import workflow is added.
    pub target_id: Option<String>,
    /// Required because restore can replace existing ZFS state.
    pub force: bool,
}

fn default_destination() -> String {
    "default".to_string()
}

fn default_retention() -> u32 {
    7
}

fn default_true() -> bool {
    true
}
