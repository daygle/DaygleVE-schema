//! Scheduled snapshots with retention.
//!
//! A [`SnapshotSchedule`] captures a snapshot of one guest (VM or container) on
//! a cron timetable and prunes the oldest automatic snapshots so at most `keep`
//! of them are retained. Scheduled snapshots are named with a recognizable
//! `auto-<timestamp>` prefix so retention only ever removes snapshots this
//! schedule created — manual snapshots are never touched. Cron expressions are
//! standard 5-field and evaluated in UTC.

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::common::{ResourceId, Timestamp};
use crate::schedule::ScheduleTarget;

/// A cron-scheduled, retention-pruned snapshot task for one guest.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SnapshotSchedule {
    pub id: ResourceId,
    pub target_kind: ScheduleTarget,
    pub target_id: ResourceId,
    /// Standard 5-field cron expression, evaluated in UTC.
    pub cron: String,
    /// Maximum number of automatic snapshots to retain for this guest; older
    /// `auto-` snapshots beyond this count are pruned after each capture. `0`
    /// keeps every automatic snapshot (no pruning).
    pub keep: u32,
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_run_at: Option<Timestamp>,
    /// Outcome of the most recent firing: `ok`, `ok (pruned N)`, or an error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_result: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_run_at: Option<Timestamp>,
    pub created_at: Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
}

/// Body for `POST /api/v1/snapshot-schedules`. The target guest is fixed at
/// creation.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateSnapshotScheduleRequest {
    pub target_kind: ScheduleTarget,
    pub target_id: ResourceId,
    pub cron: String,
    pub keep: u32,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

fn default_true() -> bool {
    true
}

/// Body for `PATCH /api/v1/snapshot-schedules/{id}`. The target guest is
/// immutable; only the timetable, retention, enabled flag, and description
/// change. `description: Some("")` clears it; omitted fields are unchanged.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateSnapshotScheduleRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cron: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keep: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
