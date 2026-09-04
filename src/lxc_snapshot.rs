//! LXC container snapshot types.

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::common::{ResourceId, Timestamp};

/// A point-in-time snapshot of a container's ZFS-backed rootfs.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LxcSnapshot {
    pub id: ResourceId,
    pub name: String,
    /// The container this snapshot belongs to.
    pub container_id: ResourceId,
    /// Backing ZFS dataset that was snapshotted.
    pub dataset: String,
    pub used_bytes: u64,
    pub created_at: Timestamp,
}

/// Body for `POST /api/v1/containers/{id}/snapshots` — capture a snapshot.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateLxcSnapshotRequest {
    pub name: String,
}

/// Full snapshot record returned by the API (same shape as LxcSnapshot).
pub type LxcSnapshotRecord = LxcSnapshot;
