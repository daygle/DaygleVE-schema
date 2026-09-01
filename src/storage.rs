//! ZFS storage types: pools, datasets, snapshots, clones and send/receive.

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::common::{ResourceId, Timestamp};

/// Health of a ZFS pool, mirroring `zpool status`.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PoolHealth {
    Online,
    Degraded,
    Faulted,
    Offline,
    Unavail,
}

/// A ZFS storage pool.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Pool {
    pub name: String,
    pub health: PoolHealth,
    pub size_bytes: u64,
    pub allocated_bytes: u64,
    pub free_bytes: u64,
    /// Fragmentation percentage, 0–100.
    pub fragmentation_pct: u8,
}

/// Kind of ZFS dataset.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DatasetKind {
    /// POSIX filesystem dataset.
    Filesystem,
    /// Block device (zvol), typically backing a VM disk.
    Volume,
}

/// A ZFS dataset (filesystem or zvol).
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Dataset {
    pub id: ResourceId,
    /// Full ZFS path, e.g. `tank/vms/web01-disk0`.
    pub name: String,
    pub kind: DatasetKind,
    pub used_bytes: u64,
    pub available_bytes: u64,
    /// Mountpoint for filesystems; `None` for volumes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mountpoint: Option<String>,
    pub compression: String,
    pub created_at: Timestamp,
}

/// A point-in-time ZFS snapshot of a dataset.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Snapshot {
    pub id: ResourceId,
    /// Full snapshot name, e.g. `tank/vms/web01-disk0@daily-2026-09-01`.
    pub name: String,
    /// The dataset this snapshot belongs to.
    pub dataset: String,
    pub used_bytes: u64,
    pub created_at: Timestamp,
}

/// Body for `POST /api/v1/storage/datasets`.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateDatasetRequest {
    /// Full ZFS path to create.
    pub name: String,
    pub kind: DatasetKind,
    /// Required for volumes; ignored for filesystems.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_gib: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression: Option<String>,
}

/// Body for `POST /api/v1/storage/datasets/{id}/snapshots`.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateSnapshotRequest {
    /// Short snapshot name (after the `@`), e.g. `pre-upgrade`.
    pub name: String,
    /// Recurse into child datasets.
    #[serde(default)]
    pub recursive: bool,
}

/// Body for `POST /api/v1/storage/snapshots/{id}/clone`.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CloneSnapshotRequest {
    /// Full ZFS path of the new clone.
    pub target: String,
}
