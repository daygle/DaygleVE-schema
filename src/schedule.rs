//! Scheduled power actions for guests.
//!
//! A power schedule fires a power action (start / graceful shutdown / reboot)
//! against one VM or container on a cron timetable. This is deliberately more
//! flexible than host-boot autostart: arbitrary recurring times, e.g. "shut the
//! lab VMs down at 22:00 on weekdays, start them at 07:00". Cron expressions are
//! standard 5-field and evaluated in UTC.

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::common::{ResourceId, Timestamp};

/// Which kind of guest a power schedule targets.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ScheduleTarget {
    Vm,
    Lxc,
}

/// A guest-agnostic power action a schedule performs. On execution it maps to
/// the guest-specific action: for containers `Shutdown` → stop and `Reboot` →
/// restart.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PowerScheduleAction {
    Start,
    Shutdown,
    Reboot,
}

/// A cron-scheduled power action against one guest.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PowerSchedule {
    pub id: ResourceId,
    pub target_kind: ScheduleTarget,
    pub target_id: ResourceId,
    pub action: PowerScheduleAction,
    /// Standard 5-field cron expression (`min hour day month weekday`),
    /// evaluated in UTC.
    pub cron: String,
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// When the schedule most recently fired.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_run_at: Option<Timestamp>,
    /// Outcome of the most recent firing: `ok` or a short error summary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_result: Option<String>,
    /// Next time the schedule is expected to fire (advisory; recomputed as it
    /// runs).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_run_at: Option<Timestamp>,
    pub created_at: Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
}

/// Body for `POST /api/v1/schedules`. The target guest is fixed at creation.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreatePowerScheduleRequest {
    pub target_kind: ScheduleTarget,
    pub target_id: ResourceId,
    pub action: PowerScheduleAction,
    pub cron: String,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

fn default_true() -> bool {
    true
}

/// Body for `PATCH /api/v1/schedules/{id}`. The target guest is immutable; only
/// the action, cron, enabled flag, and description can change. `description:
/// Some("")` clears it; omitted fields are left unchanged.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdatePowerScheduleRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<PowerScheduleAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cron: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
