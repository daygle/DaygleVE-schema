//! USB passthrough types: host USB inventory and per-guest assignment.

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// A host USB device eligible for passthrough.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UsbDevice {
    /// USB vendor id as four lowercase hex digits, e.g. `1d6b`.
    pub vendor_id: String,
    /// USB product id as four lowercase hex digits, e.g. `0003`.
    pub product_id: String,
    /// Human-readable description (`<manufacturer> <product>`), when the device
    /// reports it; otherwise the bare `vendor:product` id.
    pub description: String,
}

/// A host USB device passed through to a VM, matched by USB vendor:product.
/// Matching by id (rather than bus/port address) keeps the passthrough stable
/// across a replug or host reboot.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UsbAssignment {
    /// USB vendor id, four hex digits.
    pub vendor_id: String,
    /// USB product id, four hex digits.
    pub product_id: String,
}
