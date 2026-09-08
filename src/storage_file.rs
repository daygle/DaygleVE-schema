//! Local media-library types: uploaded install ISOs and LXC container-template
//! tarballs the node keeps on local storage.
//!
//! These back the library-management endpoints (`GET/POST/DELETE
//! /api/v1/storage/isos` and `.../ct-templates`). Install media chosen for a VM
//! is still enumerated (with network shares folded in) by
//! [`crate::vm::IsoImage`] via `GET /api/v1/vms/iso-images`; the types here are
//! the *management* view of what lives in the node's own upload directories.

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// Which local library a file belongs to.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StorageFileKind {
    /// A bootable installer/live ISO offered as VM install media.
    Iso,
    /// An LXC container-template rootfs tarball (`.tar`, `.tar.gz`, `.tar.xz`,
    /// `.tar.zst`, or `.tgz`) usable as a container's rootfs source.
    CtTemplate,
    /// A VM disk image (`.qcow2`, `.vmdk`, `.raw`, `.img`, `.vdi`, `.vhd`,
    /// `.vhdx`) that can be imported into a zvol as a VM disk.
    DiskImage,
}

/// Body for `POST /api/v1/storage/disk-images/import` — convert an uploaded
/// disk image into a new ZFS zvol usable as a VM disk.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImportDiskImageRequest {
    /// File name of the uploaded disk image (from `GET
    /// /api/v1/storage/disk-images`).
    pub image_name: String,
    /// Target ZFS dataset for the new zvol, e.g. `tank/vm-imported-disk0`. Must
    /// not already exist.
    pub dataset: String,
    /// Size of the target zvol in GiB. When omitted, the image's virtual size is
    /// detected and rounded up. When given, it must be at least the virtual
    /// size.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_gib: Option<u64>,
}

/// Result of a successful `POST /api/v1/storage/disk-images/import`: the new
/// zvol now holds the image's contents and can back a VM disk.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImportDiskImageResponse {
    /// ZFS dataset of the created zvol, e.g. `tank/vm-imported-disk0`. Use this
    /// as a VM disk's `dataset`.
    pub dataset: String,
    /// Provisioned size of the zvol in GiB.
    pub size_gib: u64,
}

/// A file in one of the node's local upload libraries.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StorageFile {
    /// Bare file name, e.g. `debian-13.0-amd64-netinst.iso` or
    /// `alpine-3.20-rootfs.tar.xz`. This is the id used in the delete path and,
    /// for templates, as a container's `template_file`.
    pub name: String,
    /// Absolute host path of the file.
    pub path: String,
    pub size_bytes: u64,
    pub kind: StorageFileKind,
}
