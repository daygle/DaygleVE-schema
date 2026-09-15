//! Path-scoped access control (RBAC).
//!
//! Access is decided per resource **path** rather than by a single node-wide
//! role. An [`AclEntry`] grants a [`Role`](crate::auth::Role) on a path to a
//! user; when `propagate` is set (the usual case) the grant also covers every
//! descendant of that path. The effective permissions for a request are the
//! union of the roles from every entry that covers the requested path.
//!
//! Paths mirror the API's resource tree:
//! - `/` — the whole node (a grant here is node-wide, the classic "global
//!   role").
//! - `/vms`, `/vms/{id}` — all VMs, or one VM.
//! - `/containers`, `/containers/{id}` — all containers, or one.
//! - `/pools`, `/pools/{id}` — resource pools.
//! - node-scoped areas such as `/network`, `/storage`, `/users`, `/audit` — a
//!   grant at `/` (or at that path) is required to act on them.
//!
//! A user's [`roles`](crate::auth::User::roles) are sugar for a grant at `/`
//! and may now be empty: such a user has no node-wide access and can act only
//! where an explicit `AclEntry` grants it (a scoped-only operator). This is a
//! full replacement for global roles — there is no implicit fallback beyond the
//! root grant that `roles` represents.

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::auth::Role;
use crate::common::{ResourceId, Timestamp};

/// A single path-scoped access-control entry.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AclEntry {
    pub id: ResourceId,
    /// Resource path the grant applies to, e.g. `/`, `/vms`, `/vms/abc`,
    /// `/pools/prod`. Always begins with `/` and has no trailing slash (except
    /// the root `/` itself).
    pub path: String,
    /// The user account the grant is for.
    pub subject: ResourceId,
    /// Role granted at the path.
    pub role: Role,
    /// When true, the grant covers the path and all of its descendants; when
    /// false, only the exact path.
    pub propagate: bool,
    /// Display name of the subject at creation time (username), for listing
    /// without a second lookup. Advisory only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject_username: Option<String>,
    pub created_at: Timestamp,
}

/// Body for `POST /api/v1/acl` — grant a role on a path to a user.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateAclEntryRequest {
    /// Resource path to grant on. Normalized server-side (a missing leading `/`
    /// is added, a trailing `/` is trimmed).
    pub path: String,
    /// The user account to grant to.
    pub subject: ResourceId,
    /// Role to grant.
    pub role: Role,
    /// Whether the grant covers descendants of the path. Defaults to true when
    /// omitted.
    #[serde(default)]
    pub propagate: Option<bool>,
}

/// One effective grant on a path for the current caller, returned by
/// `GET /api/v1/auth/me` so the UI can reason about what the user may do and
/// where without replaying the ACL itself.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PathGrant {
    pub path: String,
    pub role: Role,
    pub propagate: bool,
}
