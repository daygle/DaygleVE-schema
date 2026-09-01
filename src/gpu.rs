//! GPU passthrough types: host GPU inventory and per-guest assignment.

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// A host GPU (or IOMMU function) eligible for passthrough.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GpuDevice {
    /// PCI address, e.g. `0000:01:00.0`.
    pub pci_address: String,
    /// Human-readable vendor, e.g. `NVIDIA`.
    pub vendor: String,
    /// Human-readable model, e.g. `GeForce RTX 4090`.
    pub model: String,
    /// PCI `vendor:device` id, e.g. `10de:2684`.
    pub pci_id: String,
    /// IOMMU group the device belongs to; all functions in a group must be
    /// passed through together.
    pub iommu_group: u32,
    /// Whether the device is currently bound to `vfio-pci` and free to assign.
    pub available: bool,
    /// Id of the guest this device is assigned to, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<String>,
}

/// A GPU assignment attached to a VM.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GpuAssignment {
    /// PCI address of the passed-through device.
    pub pci_address: String,
    /// Expose the GPU's ROM/vBIOS to the guest (needed by some drivers).
    #[serde(default)]
    pub primary: bool,
}

/// Body for `POST /api/v1/gpus/{pci_address}/bind` — bind a host GPU to
/// `vfio-pci` so it can be passed through.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BindGpuRequest {
    /// Rebind the device even if a host driver currently claims it.
    #[serde(default)]
    pub force: bool,
}
