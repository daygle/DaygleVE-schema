//! Host firewall types.
//!
//! DaygleVE's host firewall filters traffic destined for the **node itself** —
//! the management API, SSH, and any other host-local service — via an nftables
//! `input` hook. It is independent of the per-VM guest firewall
//! (`crate::vm::VmFirewall`), which filters *forwarded* guest traffic on the
//! bridge; the two operate on different hooks and never interfere.
//!
//! On a single node the "datacenter" and "host" firewall levels collapse into
//! this one node-scoped configuration. Loopback and established/related return
//! traffic are always accepted by the backend regardless of the rules below, so
//! a default-drop policy cannot sever existing connections or local sockets —
//! but an operator must still add a rule admitting the management port before
//! enabling a default-drop policy, or they will lock themselves out.

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// Default verdict for inbound host traffic that matches no rule.
#[typeshare]
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FirewallPolicy {
    /// Admit unmatched traffic (a permissive default; add `drop` rules to block).
    #[default]
    Accept,
    /// Reject unmatched traffic (a default-deny posture; add `accept` rules to
    /// admit the services you need, e.g. the management port).
    Drop,
}

/// Verdict applied when a host-firewall rule matches.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FirewallAction {
    /// Admit the matching traffic.
    Accept,
    /// Silently discard the matching traffic.
    Drop,
    /// Discard and send an ICMP/TCP rejection back to the sender.
    Reject,
}

/// Transport a host-firewall rule matches. `Any` ignores ports.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FirewallProtocol {
    Tcp,
    Udp,
    Icmp,
    Any,
}

/// One inbound host-firewall rule. Rules are evaluated in order; the first match
/// decides the verdict, and traffic matching no rule takes the
/// [`HostFirewall::default_input_policy`].
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HostFirewallRule {
    /// Verdict when this rule matches.
    pub action: FirewallAction,
    /// Transport this rule matches.
    pub protocol: FirewallProtocol,
    /// Source network the rule matches, in CIDR form (e.g. `10.0.0.0/24` or a
    /// single host `203.0.113.5/32`). Absent means any source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_cidr: Option<String>,
    /// Destination port on the host. Meaningful only for `tcp`/`udp`; ignored
    /// for `icmp`/`any`. Absent means any port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dest_port: Option<u16>,
    /// Optional human label for the rule (UI-only; not used by the host).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Host (node) firewall configuration. When `enabled` is false (the default),
/// no host-level filtering is applied and the node accepts traffic as if the
/// firewall did not exist.
#[typeshare]
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct HostFirewall {
    /// Whether host-level filtering is active.
    pub enabled: bool,
    /// Verdict for inbound traffic matching no rule.
    #[serde(default)]
    pub default_input_policy: FirewallPolicy,
    /// Ordered inbound rules. Loopback and established/related traffic are always
    /// accepted ahead of these, so they never need an explicit rule.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<HostFirewallRule>,
}

/// Body for `PUT /api/v1/network/firewall` — replace the entire host firewall
/// configuration. The complete desired state is submitted each time (there is
/// no per-rule endpoint), mirroring how the ruleset is applied atomically on the
/// host.
#[typeshare]
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateHostFirewallRequest {
    pub enabled: bool,
    #[serde(default)]
    pub default_input_policy: FirewallPolicy,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<HostFirewallRule>,
}
