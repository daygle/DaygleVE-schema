//! General PCI passthrough types: non-GPU host PCI inventory and per-guest
//! assignment. Display-class devices are covered by the [`crate::gpu`] flow and
//! are excluded from this inventory to avoid overlap.

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// A host PCI function eligible for passthrough (NICs, HBAs, controllers, …).
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PciDevice {
    /// PCI address, e.g. `0000:01:00.0`.
    pub pci_address: String,
    /// PCI `vendor:device` id, e.g. `8086:10d3`.
    pub pci_id: String,
    /// Human-readable vendor when known, else the hex vendor id.
    pub vendor: String,
    /// Coarse device category from the PCI base class, e.g. `Network`,
    /// `Storage`, `Serial bus`.
    pub class: String,
    /// IOMMU group; all functions in a group must be passed through together.
    pub iommu_group: u32,
    /// Whether the device is bound to `vfio-pci` and free to assign.
    pub available: bool,
}

/// A host PCI device passed through to a VM.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PciAssignment {
    /// PCI address of the passed-through device.
    pub pci_address: String,
}
