//! Network storage shares (NFS / CIFS).
//!
//! A share is a remote filesystem mounted on the node and used as a content
//! source. Today the supported content is **install media**: ISOs found on a
//! mounted share are surfaced in the VM install-media picker (see
//! [`crate::vm::IsoImage`]) and can be attached to a VM's CD-ROM. Disk and
//! backup content types are intentionally out of scope for now.

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::common::{ResourceId, Timestamp};

/// Protocol of a network share.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ShareType {
    /// NFS export, addressed as `server:/export`.
    Nfs,
    /// SMB/CIFS share, addressed as `//server/share`.
    Cifs,
}

/// Whether the share is currently mounted and reachable on the node.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ShareState {
    /// Mounted and available.
    Connected,
    /// Configured but not currently mounted.
    Disconnected,
    /// The last mount attempt failed; see `last_error`.
    Error,
}

/// A configured network share.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NetworkShare {
    pub id: ResourceId,
    pub name: String,
    pub share_type: ShareType,
    /// Server host or IP address.
    pub server: String,
    /// NFS export path (`/export/isos`) or CIFS share name (`isos`). Named
    /// `export_path` rather than `export` because `export` is a reserved word
    /// in JavaScript/TypeScript modules and awkward for API consumers.
    pub export_path: String,
    /// Absolute mount point on the node.
    pub mount_point: String,
    pub state: ShareState,
    /// Whether the share is mounted read-only (always true for now — shares are
    /// content sources, not writable storage).
    pub read_only: bool,
    /// CIFS username, when applicable. The password is write-only and never
    /// returned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Extra mount options applied (any credential options are elided).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,
    /// Detail of the most recent mount failure, when `state` is `error`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
    pub created_at: Timestamp,
}

/// Body for `POST /api/v1/storage/shares` — add and mount a network share.
///
/// `Debug` is implemented by hand (not derived) so the CIFS `password` is never
/// printed if a request body is ever logged.
#[typeshare]
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateShareRequest {
    pub name: String,
    pub share_type: ShareType,
    /// Server host or IP address.
    pub server: String,
    /// NFS export path (`/export/isos`) or CIFS share name (`isos`). Named
    /// `export_path` rather than `export` (a reserved word in JS/TS modules).
    pub export_path: String,
    /// Extra comma-separated mount options, e.g. `vers=4.1` (NFS) or `vers=3.0`
    /// (CIFS). Credential options are supplied via the fields below, not here.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,
    /// CIFS username.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// CIFS password. Write-only: stored in a root-only credentials file on the
    /// node and never returned by the API.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// CIFS domain / workgroup.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}

impl std::fmt::Debug for CreateShareRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreateShareRequest")
            .field("name", &self.name)
            .field("share_type", &self.share_type)
            .field("server", &self.server)
            .field("export_path", &self.export_path)
            .field("options", &self.options)
            .field("username", &self.username)
            // Never print the password, even via {:?}.
            .field("password", &self.password.as_ref().map(|_| "<redacted>"))
            .field("domain", &self.domain)
            .finish()
    }
}
