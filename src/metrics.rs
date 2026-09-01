//! Metrics types: node-level and per-guest resource usage, plus the real-time
//! event shape streamed over SSE/WebSocket.

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::common::{ResourceId, Timestamp};

/// Instantaneous node-wide resource usage, from `GET /api/v1/metrics/node`.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NodeMetrics {
    pub timestamp: Timestamp,
    /// Aggregate CPU utilisation, 0.0–100.0.
    pub cpu_pct: f64,
    /// Number of logical CPUs.
    pub cpu_count: u32,
    /// 1/5/15-minute load averages.
    pub load_average: [f64; 3],
    pub memory_total_bytes: u64,
    pub memory_used_bytes: u64,
    pub swap_total_bytes: u64,
    pub swap_used_bytes: u64,
    /// Aggregate disk read/write throughput.
    pub disk_read_bps: u64,
    pub disk_write_bps: u64,
    /// Aggregate network throughput across host uplinks.
    pub net_rx_bps: u64,
    pub net_tx_bps: u64,
    pub uptime_seconds: u64,
}

/// Instantaneous per-guest (VM or container) resource usage.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GuestMetrics {
    pub id: ResourceId,
    pub timestamp: Timestamp,
    pub cpu_pct: f64,
    pub memory_used_bytes: u64,
    pub memory_max_bytes: u64,
    pub disk_read_bps: u64,
    pub disk_write_bps: u64,
    pub net_rx_bps: u64,
    pub net_tx_bps: u64,
}

/// Which resource kind a [`MetricsEvent`] describes.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MetricsScope {
    Node,
    Vm,
    Lxc,
}

/// A single real-time metrics frame pushed to the frontend over the
/// `GET /api/v1/metrics/stream` SSE endpoint.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetricsEvent {
    pub scope: MetricsScope,
    /// Node payload; present when `scope == Node`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<NodeMetrics>,
    /// Guest payload; present when `scope == Vm` or `Lxc`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guest: Option<GuestMetrics>,
}
