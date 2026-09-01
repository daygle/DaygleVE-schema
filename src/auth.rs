//! Authentication and RBAC types: login, bearer tokens, users, roles and the
//! fine-grained permission set the backend enforces.

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::common::{ResourceId, Timestamp};

/// Credentials submitted to `POST /api/v1/auth/login`.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Successful login response carrying a bearer token and the caller's identity.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoginResponse {
    /// Opaque bearer token to send as `Authorization: Bearer <token>`.
    pub token: String,
    /// Absolute expiry of the token.
    pub expires_at: Timestamp,
    /// The authenticated user.
    pub user: User,
}

/// A platform user account. Never carries secrets — password hashes stay
/// server-side.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    pub id: ResourceId,
    pub username: String,
    /// Roles assigned to this user; the effective permission set is their
    /// union.
    pub roles: Vec<Role>,
    pub created_at: Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_login_at: Option<Timestamp>,
}

/// Coarse-grained role. The effective [`Permission`] set is derived from the
/// role by the backend's RBAC layer.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    /// Full control over the node and all resources, including user admin.
    Admin,
    /// Create/modify/delete guests, storage and networks; no user admin.
    Operator,
    /// Read-only visibility into all resources and metrics.
    Viewer,
}

/// A single fine-grained permission checked at the API boundary. Roles map to
/// sets of these; endpoints declare the permission they require.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Permission {
    VmRead,
    VmWrite,
    VmPower,
    LxcRead,
    LxcWrite,
    LxcPower,
    StorageRead,
    StorageWrite,
    NetworkRead,
    NetworkWrite,
    GpuRead,
    GpuWrite,
    MetricsRead,
    UserAdmin,
}

/// The caller's own identity and effective permissions, from
/// `GET /api/v1/auth/me`.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CurrentUser {
    pub user: User,
    /// Flattened, de-duplicated permission set granted by the user's roles.
    pub permissions: Vec<Permission>,
}
