//! KVM/QEMU virtual machine types: summaries, full detail, lifecycle state and
//! the create/update request bodies.

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::common::{ResourceId, Timestamp};
use crate::gpu::GpuAssignment;

/// Lifecycle state of a virtual machine, as reported by the hypervisor.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VmState {
    Running,
    Stopped,
    Paused,
    /// A lifecycle transition (start/stop/reboot/migrate-in) is in progress.
    Transitioning,
    /// The hypervisor reports the domain in an error state.
    Error,
}

/// Power actions accepted by `POST /api/v1/vms/{id}/power`.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VmPowerAction {
    Start,
    /// Graceful ACPI shutdown.
    Shutdown,
    /// Force power-off.
    Stop,
    Reboot,
    Reset,
    Pause,
    Resume,
}

/// Firmware / boot mode for a VM.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Firmware {
    /// Legacy SeaBIOS.
    Bios,
    /// UEFI (OVMF).
    Uefi,
}

/// A virtual disk attached to a VM, backed by a ZFS zvol.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VmDisk {
    /// Backing ZFS dataset path, e.g. `tank/vms/web01-disk0`.
    pub dataset: String,
    pub size_gib: u64,
    /// Bus the disk is exposed on inside the guest.
    pub bus: DiskBus,
}

/// Guest-visible disk bus.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DiskBus {
    Virtio,
    Scsi,
    Sata,
}

/// A virtual NIC attached to a VM.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VmNic {
    /// Host bridge this NIC is attached to, e.g. `vmbr0`.
    pub bridge: String,
    /// Optional VLAN tag.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vlan: Option<u16>,
    /// MAC address; assigned by the backend when omitted on create.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    pub model: NicModel,
}

/// Guest-visible NIC model.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NicModel {
    Virtio,
    E1000,
    Rtl8139,
}

/// An installer/live ISO available on the host that can be attached to a VM as
/// virtual install media. Enumerated by `GET /api/v1/vms/iso-images` from the
/// node's ISO library; a VM's `cdrom` field holds the chosen image's `path`.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IsoImage {
    /// File name, e.g. `debian-13.0-amd64-netinst.iso`.
    pub name: String,
    /// Absolute host path, e.g. `/var/lib/daygleve/isos/debian-13.0-amd64-netinst.iso`.
    /// This is the value to send as a VM's `cdrom`.
    pub path: String,
    pub size_bytes: u64,
    /// Storage this ISO was found on: `local` for the node's built-in library,
    /// otherwise the name of the network share it lives on.
    pub storage: String,
}

/// Compact VM record for list views.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VmSummary {
    pub id: ResourceId,
    pub name: String,
    pub state: VmState,
    pub vcpus: u32,
    pub memory_mib: u64,
    pub created_at: Timestamp,
}

/// Full VM detail for `GET /api/v1/vms/{id}`.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Vm {
    pub id: ResourceId,
    pub name: String,
    pub state: VmState,
    pub vcpus: u32,
    pub memory_mib: u64,
    pub firmware: Firmware,
    pub disks: Vec<VmDisk>,
    pub nics: Vec<VmNic>,
    /// GPUs passed through to this VM, if any.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub gpus: Vec<GpuAssignment>,
    /// Host path of an install ISO attached as a virtual CD-ROM, if any. When
    /// set, the VM boots from the CD-ROM first (so a guest OS can be installed)
    /// and falls back to disk; eject it once the OS is installed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdrom: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub created_at: Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
}

/// Body for `POST /api/v1/vms` — the desired spec of a new VM.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateVmRequest {
    pub name: String,
    pub vcpus: u32,
    pub memory_mib: u64,
    pub firmware: Firmware,
    pub disks: Vec<VmDisk>,
    pub nics: Vec<VmNic>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub gpus: Vec<GpuAssignment>,
    /// Host path of an install ISO to attach as a virtual CD-ROM. Must be one
    /// of the images returned by `GET /api/v1/vms/iso-images`. When set the VM
    /// boots from it first so a guest OS can be installed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdrom: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Start the VM immediately after creation.
    #[serde(default)]
    pub start: bool,
}

/// Body for `PATCH /api/v1/vms/{id}` — all fields optional; only present
/// fields are applied. Most changes require the VM to be stopped.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateVmRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vcpus: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_mib: Option<u64>,
    /// Attach or replace the install ISO with this host path (one of the images
    /// from `GET /api/v1/vms/iso-images`). Ignored when `eject_cdrom` is true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdrom: Option<String>,
    /// Eject any attached install ISO. Takes precedence over `cdrom`.
    #[serde(default)]
    pub eject_cdrom: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Body for `POST /api/v1/vms/{id}/power`.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VmPowerRequest {
    pub action: VmPowerAction,
}

/// Response from `POST /api/v1/vms/{id}/console` — a short-lived noVNC ticket.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConsoleTicket {
    /// WebSocket path the frontend's noVNC client connects to.
    pub websocket_path: String,
    /// One-time ticket authorising the console connection.
    pub ticket: String,
    pub expires_at: Timestamp,
}
