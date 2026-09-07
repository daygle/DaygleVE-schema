//! LXC container types: summaries, full detail, lifecycle state and
//! create/update request bodies.

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::common::{ResourceId, Timestamp};

/// Lifecycle state of an LXC container.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LxcState {
    Running,
    Stopped,
    Frozen,
    Transitioning,
    Error,
}

/// Power actions accepted by `POST /api/v1/containers/{id}/power`.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LxcPowerAction {
    Start,
    Stop,
    Restart,
    Freeze,
    Unfreeze,
}

/// A network interface attached to a container.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LxcNetwork {
    /// Host bridge, e.g. `vmbr0`.
    pub bridge: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vlan: Option<u16>,
    /// Static address in CIDR form, or `None` for DHCP.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

/// A host directory bind-mounted into the container. The host `source` is
/// exposed at the container-absolute `destination`. This is a privileged
/// capability: it can expose arbitrary host paths to the guest, so it is only
/// available to operators/admins who can create containers.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LxcMount {
    /// Absolute host path to expose (a directory).
    pub source: String,
    /// Absolute path inside the container where `source` appears.
    pub destination: String,
    /// Mount read-only. Defaults to true (the safer default).
    #[serde(default = "default_true")]
    pub read_only: bool,
}

/// Compact container record for list views.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LxcSummary {
    pub id: ResourceId,
    pub name: String,
    pub state: LxcState,
    pub vcpus: u32,
    pub memory_mib: u64,
    pub created_at: Timestamp,
}

/// Full container detail for `GET /api/v1/containers/{id}`.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Lxc {
    pub id: ResourceId,
    pub name: String,
    pub state: LxcState,
    /// OS template the rootfs was created from, e.g. `debian-12`.
    pub template: String,
    /// Backing ZFS dataset for the rootfs.
    pub rootfs_dataset: String,
    pub vcpus: u32,
    pub memory_mib: u64,
    pub networks: Vec<LxcNetwork>,
    /// Host directories bind-mounted into the container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mounts: Vec<LxcMount>,
    /// Run unprivileged (user-namespaced) when `true`.
    pub unprivileged: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub created_at: Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
}

/// Body for `POST /api/v1/containers`.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateLxcRequest {
    pub name: String,
    pub template: String,
    pub vcpus: u32,
    pub memory_mib: u64,
    pub rootfs_size_gib: u64,
    pub networks: Vec<LxcNetwork>,
    /// Host directories to bind-mount into the container.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mounts: Vec<LxcMount>,
    #[serde(default = "default_true")]
    pub unprivileged: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub start: bool,
}

fn default_true() -> bool {
    true
}

/// Body for `PATCH /api/v1/containers/{id}`.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateLxcRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vcpus: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_mib: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Body for `POST /api/v1/containers/{id}/power`.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LxcPowerRequest {
    pub action: LxcPowerAction,
}
