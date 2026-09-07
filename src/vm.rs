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
    /// Graceful shutdown: use QEMU guest agent `shutdown` when the guest agent is
    /// enabled, otherwise fall back to ACPI button press. The guest agent path gives
    /// the guest OS a chance to quiesce (flush writes, stop services) before power
    /// off; ACPI-only shutdown does not.
    Shutdown,
    /// Force power-off (`virsh destroy`). No guest cooperation; equivalent to pulling
    /// the power plug. Running-VM snapshots taken after this are crash-consistent, not
    /// application-consistent.
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
    /// Storage this ISO was found on: the reserved value `local` for the node's
    /// built-in library, otherwise the name of the network share it lives on.
    /// The backend rejects `local` as a share name so this is unambiguous.
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
    /// True when this VM is a template (a clone-only golden image that cannot be
    /// powered on). Surfaced here so list views can badge it and hide power
    /// controls.
    #[serde(default)]
    pub template: bool,
    /// Start automatically on host boot.
    #[serde(default)]
    pub autostart: bool,
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
    /// Path to a cloud-init NoCloud seed ISO attached as a second CD-ROM, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cloud_init_iso: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the QEMU guest agent channel is enabled for this VM.
    #[serde(default)]
    pub guest_agent: bool,
    /// Host-side firewall configuration for this VM.
    #[serde(default, skip_serializing_if = "VmFirewall::is_empty")]
    pub firewall: VmFirewall,
    /// When true, this VM is a template: a golden image used only as a clone
    /// source. Templates cannot be powered on or autostarted.
    #[serde(default)]
    pub template: bool,
    /// Start this VM automatically when the host boots. Ignored for templates.
    #[serde(default)]
    pub autostart: bool,
    /// Ordering for host-boot autostart: lower numbers start first; VMs without
    /// an explicit order start last. Only meaningful when `autostart` is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub startup_order: Option<u32>,
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
    /// Start the VM immediately after creation. Rejected together with
    /// `template` (a template is never powered on).
    #[serde(default)]
    pub start: bool,
    /// Create this VM as a template (a clone-only golden image that cannot be
    /// powered on).
    #[serde(default)]
    pub template: bool,
    /// Start this VM automatically on host boot. Ignored for templates.
    #[serde(default)]
    pub autostart: bool,
    /// Autostart ordering: lower numbers start first.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub startup_order: Option<u32>,
    /// QEMU guest agent integration. When enabled, the VM gets a virtio-serial guest
    /// agent channel (the guest must also run `qemu-guest-agent` to use it).
    #[serde(default)]
    pub guest_agent: bool,
    /// Host-side firewall for the VM's bridge traffic (nftables).
    #[serde(default, skip_serializing_if = "VmFirewall::is_empty")]
    pub firewall: VmFirewall,
    /// Cloud-init/automated provisioning. When set, a NoCloud seed ISO is generated
    /// and attached as a second CD-ROM (`sdab`) so the guest can auto-configure
    /// hostname, network, authorized SSH keys, and default user on first boot.
    /// Requires the VM to be stopped when changing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cloud_init: Option<CloudInitRequest>,
}

/// Per-NIC firewall rules applied on the host (nftables) for the guest's traffic.
/// Only outbound/inbound filtering on the bridge is supported today; the host
/// itself is not affected. Rules are evaluated per-bridge in the guest's forward
/// chain.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VmFirewallRule {
    /// Direction of traffic this rule applies to.
    pub direction: VmFirewallDirection,
    /// Action when the rule matches.
    pub action: VmFirewallAction,
    /// CIDR the rule matches on (source for inbound, destination for outbound).
    /// Required unless the action is `accept_all` / `drop_all` style; advisory when
    /// absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
    /// Optional human label for the rule (UI-only; not used by the host).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VmFirewallDirection {
    In,
    Out,
}

#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VmFirewallAction {
    Accept,
    Drop,
    /// Convenience: accept all traffic in this direction (no CIDR required).
    AcceptAll,
    /// Convenience: drop all traffic in this direction (no CIDR required).
    DropAll,
}

/// Guest firewall configuration. When `enabled` is true, the host applies an
/// nftables per-VM chain (matched on the VM's NIC MAC addresses) that filters
/// the guest's forwarded traffic. When disabled (the default), the guest is
/// unrestricted on its bridge.
#[typeshare]
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct VmFirewall {
    /// Whether host-side filtering is active for this VM.
    pub enabled: bool,
    /// Ordered rules for the VM. With `enabled=true`, traffic that matches no
    /// rule is dropped in both directions (default deny); return traffic for
    /// established connections is always allowed.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<VmFirewallRule>,
}

impl VmFirewall {
    /// True when the firewall is disabled and carries no rules, so a default
    /// (all-zero) value can be omitted from serialized payloads.
    pub fn is_empty(&self) -> bool {
        !self.enabled && self.rules.is_empty()
    }
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
    /// Firmware/boot mode. Changing it requires the VM to be stopped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub firmware: Option<Firmware>,
    /// Replace the VM's disks with this set (declarative). New datasets are
    /// provisioned; removing a disk detaches it but never destroys its data.
    /// Requires the VM to be stopped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disks: Option<Vec<VmDisk>>,
    /// Replace the VM's NICs with this set (declarative). Requires the VM to be
    /// stopped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nics: Option<Vec<VmNic>>,
    /// Attach or replace the install ISO with this host path (one of the images
    /// from `GET /api/v1/vms/iso-images`). Ignored when `eject_cdrom` is true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdrom: Option<String>,
    /// Eject any attached install ISO (`Some(true)`). Takes precedence over
    /// `cdrom`. Optional like every other field — omitting it means "no change".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eject_cdrom: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// QEMU guest agent integration. When enabled, the VM gets a virtio-serial guest
    /// agent channel (the guest must also run `qemu-guest-agent` to use it).
    /// Requires the VM to be stopped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guest_agent: Option<bool>,
    /// Host-side firewall for the VM's bridge traffic (nftables). Requires the VM
    /// to be stopped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub firewall: Option<VmFirewall>,
    /// Cloud-init/automated provisioning. When set, a NoCloud seed ISO is generated
    /// and attached as a second CD-ROM (`sdab`) so the guest can auto-configure
    /// hostname, network, authorized SSH keys, and default user on first boot.
    /// Requires the VM to be stopped when changing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cloud_init: Option<CloudInitRequest>,
    /// Convert to/from a template. Marking a running VM as a template is rejected;
    /// setting it requires the VM to be stopped. Making a VM a template also
    /// clears its autostart flag.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<bool>,
    /// Start automatically on host boot. Ignored for templates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autostart: Option<bool>,
    /// Autostart ordering: lower numbers start first.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub startup_order: Option<u32>,
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

/// A point-in-time snapshot of a VM, taken across every one of the VM's backing
/// ZFS datasets under a single name. Enumerated by
/// `GET /api/v1/vms/{id}/snapshots`; created, rolled back to, and deleted via
/// the sibling endpoints. The snapshot `name` (the ZFS `@tag`) is unique within
/// a VM and identifies it in the path.
///
/// Two capture modes exist: disk-only ZFS snapshots (crash-consistent, the default,
/// available on running and stopped VMs) and full RAM-state snapshots (`ram`, only
/// available on running VMs, via `virsh save`). RAM-state snapshots are
/// application-consistent point-in-time captures that include guest memory; they can
/// be restored to resume the VM from exactly the captured state.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VmSnapshot {
    /// Short name shared by the underlying `dataset@name` snapshots.
    pub name: String,
    /// Space uniquely referenced by this snapshot, summed over the VM's disks.
    pub used_bytes: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub created_at: Timestamp,
    /// Capture method. `disk` is a ZFS snapshot of the backing datasets (crash-
    /// consistent). `ram` is a full VM memory/disk state save (application-
    /// consistent point-in-time) stored on the host, not a ZFS snapshot.
    #[serde(default)]
    pub snapshot_type: VmSnapshotType,
}

/// Body for `POST /api/v1/vms/{id}/snapshots` — capture a new VM snapshot.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateVmSnapshotRequest {
    /// Snapshot name; must be a valid ZFS snapshot tag — letters, digits, and
    /// the punctuation `_`, `-`, `.`, `:` (no spaces) — and unique within the VM.
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Capture method. Defaults to `disk`.
    #[serde(default)]
    pub snapshot_type: VmSnapshotType,
}

/// Body for `POST /api/v1/vms/{id}/disks/{index}/resize` — grow a VM disk's
/// backing zvol (and resize the running guest's block device when the VM is up).
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResizeVmDiskRequest {
    /// New size in GiB. Must be larger than the current size: shrinking a zvol
    /// is rejected because it destroys data beyond the new boundary.
    pub size_gib: u64,
}

/// Body for `POST /api/v1/vms/{id}/clone` — copy an existing VM into a new one.
/// The clone gets a fresh id and freshly-generated NIC MACs; its disks are ZFS
/// clones of the source's disks (taken from a snapshot of the source). Any GPU
/// passthrough and attached install ISO are dropped (they can't be shared). The
/// clone is created stopped.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CloneVmRequest {
    /// Name for the new VM; must be unique and host-safe (same rules as create).
    pub name: String,
    /// When `true`, the clone's disks are promoted so they no longer depend on
    /// the source's snapshot (an independent "full" clone). When `false`
    /// (default), the disks stay as linked clones — fast and space-efficient, but
    /// tied to the source until promoted.
    #[serde(default)]
    pub full: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// QEMU guest agent integration request / state.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GuestAgentConfig {
    /// Whether the guest agent channel is enabled for this VM.
    pub enabled: bool,
}

/// Cloud-init/automated provisioning request. When present on create/update, a NoCloud
/// seed ISO is generated and attached to the VM as a second CD-ROM (`sdab`) so the
/// guest can auto-configure hostname, network, authorized SSH keys, and the default
/// user on first boot.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CloudInitRequest {
    /// Desired hostname pushed to the guest via cloud-init.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// Network config for the guest: interface name, address (CIDR), gateway.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<CloudInitNetwork>,
    /// SSH public keys authorized for the default user.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ssh_keys: Vec<String>,
    /// Default user created by cloud-init. When omitted, a platform default (`debian`,
    /// `ubuntu`, etc.) is used depending on the guest OS.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_user: Option<String>,
    /// Whether to inject a root password (base64-encoded plaintext). Omitting it means
    /// no password is set and SSH-key-only auth is the default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root_password: Option<String>,
}

/// Network configuration for cloud-init.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CloudInitNetwork {
    /// Guest-side interface name (e.g. `eth0`).
    pub interface: String,
    /// IPv4 address in CIDR notation (e.g. `192.168.1.10/24`).
    pub address: String,
    /// IPv4 gateway.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    /// Optional DNS servers.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dns: Vec<String>,
}

/// Snapshot capture type.
#[typeshare]
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VmSnapshotType {
    /// ZFS snapshot of the backing datasets (crash-consistent).
    #[default]
    Disk,
    /// Full VM state save including memory (application-consistent point-in-time).
    Ram,
}

/// Body for `POST /api/v1/vms/{id}/power`.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VmPowerRequest {
    pub action: VmPowerAction,
}

/// Response from `POST /api/v1/vms/{id}/power` with extended guest info.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VmPowerResponse {
    /// The VM after the power action.
    pub vm: Vm,
    /// Guest IP addresses reported by the QEMU guest agent, when the agent is enabled
    /// and the guest is running. Absent when the agent is not enabled or the guest is
    /// not running.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guest_ips: Vec<String>,
}

/// Response from `GET /api/v1/vms/{id}/guest-agent` — guest agent status and reported
/// guest info.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GuestAgentInfo {
    /// Whether the guest agent channel is enabled on this VM.
    pub enabled: bool,
    /// Whether the guest agent is currently connected (guest responded). Only meaningful
    /// when `enabled` is true.
    pub connected: bool,
    /// Guest OS info reported by the agent, when connected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guest_os: Option<String>,
    /// Guest IP addresses reported by the agent, when connected.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub guest_ips: Vec<String>,
}
