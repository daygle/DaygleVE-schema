//! Linux networking types: bridges and VLANs.

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::common::{ResourceId, Timestamp};

/// Operational state of a network interface.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LinkState {
    Up,
    Down,
    Unknown,
}

/// A Linux bridge that VM/LXC NICs attach to.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Bridge {
    pub id: ResourceId,
    /// Bridge name, e.g. `vmbr0`.
    pub name: String,
    pub state: LinkState,
    /// Physical/bond ports enslaved to the bridge.
    pub ports: Vec<String>,
    /// Whether 802.1Q VLAN filtering is enabled on the bridge.
    pub vlan_aware: bool,
    /// Optional management address in CIDR form.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    pub mtu: u32,
    pub created_at: Timestamp,
}

/// A VLAN configured on a VLAN-aware bridge.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Vlan {
    pub id: ResourceId,
    /// Parent bridge name.
    pub bridge: String,
    /// 802.1Q tag, 1–4094.
    pub tag: u16,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Body for `POST /api/v1/network/bridges`.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateBridgeRequest {
    pub name: String,
    #[serde(default)]
    pub ports: Vec<String>,
    #[serde(default)]
    pub vlan_aware: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mtu: Option<u32>,
}

/// Body for `POST /api/v1/network/vlans`.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateVlanRequest {
    pub bridge: String,
    pub tag: u16,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
