//! Broker split contracts: the planned privilege-separation surface for the
//! DaygleVE control plane.
//!
//! These types are the stable target for the future root-owned broker. They
//! describe *what* the backend currently performs directly and *what* must
//! eventually be moved out of the API process. They do not, by themselves,
//! implement or enforce the split.

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::common::Timestamp;

/// The mode used for a subsystem in the current deployment.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BrokerMode {
    /// The API process still performs the operation directly.
    Direct,
    /// The operation has been moved behind the root-owned broker.
    Delegated,
    /// The subsystem is read-only / metadata-only and is not a broker blocker.
    Local,
}

/// Where a subsystem's host action currently runs.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum HostExecution {
    /// The API process itself.
    Api,
    /// A separate root-owned broker process.
    Broker,
    /// Not applicable / metadata only.
    None,
}

/// One subsystem's current security posture relative to the planned broker split.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BrokerSubsystem {
    /// Stable subsystem name, e.g. `kvm`, `zfs`, `lxc`, `gpu`, `network`,
    /// `share`, `backup`.
    pub subsystem: String,
    /// Current execution mode.
    pub mode: BrokerMode,
    /// Where the host action currently runs.
    pub execution: HostExecution,
    /// Whether this subsystem must move behind the broker before the control plane
    /// is considered hardened for untrusted tenants or a hostile network.
    pub broker_required: bool,
    /// Human-readable summary of the residual root-equivalent surface for this
    /// subsystem. This is for review and reporting, not runtime policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub residual_surface: Option<String>,
    /// Specific host actions currently performed directly by the API process.
    /// This is informational; it becomes enforceable only after delegation.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub current_actions: Vec<String>,
}

/// A snapshot of the current broker split posture for the whole platform.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BrokerSplitInventory {
    /// When the inventory was generated.
    pub generated_at: Timestamp,
    /// Programmatic marker that the backend is still the acting process for the
    /// privileged stack.
    pub current_execution: HostExecution,
    /// Per-subsystem posture.
    pub subsystems: Vec<BrokerSubsystem>,
    /// Whether any subsystem still requires the broker split.
    pub broker_split_incomplete: bool,
    /// Explicit note that the current inventory is informational until the broker
    /// exists and the direct paths are removed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl BrokerSplitInventory {
    /// Current posture: the API process still performs the privileged host actions.
    ///
    /// The schema crate stays types-only (no clock access), so the caller
    /// supplies the generation timestamp; the backend passes its own `now_ts()`.
    pub fn current(generated_at: Timestamp) -> Self {
        Self {
            generated_at,
            current_execution: HostExecution::Api,
            subsystems: broker_subsystems(),
            broker_split_incomplete: true,
            note: Some(
                "This inventory describes the planned broker split target. The broker itself is not deployed yet; the current values are informational, not enforced.".to_string(),
            ),
        }
    }
}

fn broker_subsystems() -> Vec<BrokerSubsystem> {
    vec![
        BrokerSubsystem {
            subsystem: "kvm".to_string(),
            mode: BrokerMode::Direct,
            execution: HostExecution::Api,
            broker_required: true,
            residual_surface: Some("libvirt system instance: VM define/start/destroy/undefine, nvram, console VNC, live state queries".to_string()),
            current_actions: vec![
                "virsh domstate / vncdisplay / dominate / domrename / undefine".to_string(),
                "qemu:///system libvirt operations".to_string(),
            ],
        },
        BrokerSubsystem {
            subsystem: "zfs".to_string(),
            mode: BrokerMode::Direct,
            execution: HostExecution::Api,
            broker_required: true,
            residual_surface: Some("ZFS dataset/snapshot/zvol mutation, send/receive, volume provisioning, pool listing".to_string()),
            current_actions: vec![
                "zfs create / snapshot / clone / receive / destroy".to_string(),
                "zpool list".to_string(),
            ],
        },
        BrokerSubsystem {
            subsystem: "lxc".to_string(),
            mode: BrokerMode::Direct,
            execution: HostExecution::Api,
            broker_required: true,
            residual_surface: Some("LXC create/start/stop/destroy, cgroup limit writes, config mutation, ZFS-backed rootfs writes".to_string()),
            current_actions: vec![
                "lxc-create / lxc-start / lxc-stop / lxc-destroy / lxc-info / lxc-cgroup".to_string(),
                "ZFS rootfs dataset writes".to_string(),
            ],
        },
        BrokerSubsystem {
            subsystem: "gpu".to_string(),
            mode: BrokerMode::Direct,
            execution: HostExecution::Api,
            broker_required: true,
            residual_surface: Some("PCI sysfs bind/unbind and vfio-pci driver overrides for IOMMU groups".to_string()),
            current_actions: vec![
                "sysfs unbind / driver_override / vfio-pci bind".to_string(),
            ],
        },
        BrokerSubsystem {
            subsystem: "network".to_string(),
            mode: BrokerMode::Direct,
            execution: HostExecution::Api,
            broker_required: true,
            residual_surface: Some("bridge/VLAN creation and modifications, mount/network device changes, namespace and cgroup usage".to_string()),
            current_actions: vec![
                "ip link / addr operations".to_string(),
                "bridge vlan add/del".to_string(),
            ],
        },
        BrokerSubsystem {
            subsystem: "share".to_string(),
            mode: BrokerMode::Direct,
            execution: HostExecution::Api,
            broker_required: true,
            residual_surface: Some("mount/umount for network shares and mount table mutation".to_string()),
            current_actions: vec![
                "mount / umount of network shares".to_string(),
            ],
        },
        BrokerSubsystem {
            subsystem: "backup".to_string(),
            mode: BrokerMode::Direct,
            execution: HostExecution::Api,
            broker_required: true,
            residual_surface: Some("long-running ZFS send/receive and restore target replacement, retention deletes".to_string()),
            current_actions: vec![
                "zfs send / receive during backup/restore".to_string(),
                "restore target destruction when forced".to_string(),
            ],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_inventory_marks_broker_split_incomplete() {
        let inventory = BrokerSplitInventory::current("2026-09-05T00:00:00Z".to_string());
        assert!(inventory.broker_split_incomplete);
        assert_eq!(inventory.current_execution, HostExecution::Api);
        assert_eq!(inventory.generated_at, "2026-09-05T00:00:00Z");
        assert!(!inventory.subsystems.is_empty());
        assert!(inventory.subsystems.iter().all(|s| s.broker_required));
    }

    #[test]
    fn all_broker_blocker_subsystems_are_listed() {
        let inventory = BrokerSplitInventory::current("2026-09-05T00:00:00Z".to_string());
        let names: Vec<_> = inventory
            .subsystems
            .iter()
            .map(|s| s.subsystem.as_str())
            .collect();
        assert!(names.contains(&"kvm"));
        assert!(names.contains(&"zfs"));
        assert!(names.contains(&"lxc"));
        assert!(names.contains(&"gpu"));
        assert!(names.contains(&"network"));
        assert!(names.contains(&"share"));
        assert!(names.contains(&"backup"));
    }
}
