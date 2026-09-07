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
