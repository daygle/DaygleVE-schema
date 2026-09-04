//! Authentication and RBAC types: login, bearer tokens, users, roles and the
//! fine-grained permission set the backend enforces.

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::common::{ResourceId, Timestamp};

/// Credentials submitted to `POST /api/v1/auth/login`.
#[typeshare]
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

impl std::fmt::Debug for LoginRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoginRequest")
            .field("username", &self.username)
            .field("password", &"<redacted>")
            .finish()
    }
}

/// Successful login response carrying a bearer token and the caller's identity.
#[typeshare]
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoginResponse {
    /// Opaque bearer token to send as `Authorization: Bearer <token>`.
    pub token: String,
    /// Absolute expiry of the token.
    pub expires_at: Timestamp,
    /// The authenticated user.
    pub user: User,
}

impl std::fmt::Debug for LoginResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoginResponse")
            .field("token", &"<redacted>")
            .field("expires_at", &self.expires_at)
            .field("user", &self.user)
            .finish()
    }
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
    /// View durable host-operation history and crash-recovery records.
    OperationsRead,
    /// Start operator-triggered host reconciliation jobs.
    OperationsWrite,
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
    /// True when the account is still on a seeded/temporary password and must
    /// set a new one (the UI should force a password change).
    pub must_change_password: bool,
}

/// Body for `POST /api/v1/users` — create a user account.
///
/// `Debug` is hand-written so the plaintext `password` is never printed.
#[typeshare]
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    /// Roles to grant; the effective permission set is their union.
    pub roles: Vec<Role>,
}

impl std::fmt::Debug for CreateUserRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreateUserRequest")
            .field("username", &self.username)
            .field("password", &"<redacted>")
            .field("roles", &self.roles)
            .finish()
    }
}

/// Body for `PATCH /api/v1/users/{id}` — update a user's roles and/or reset
/// their password (admin action). Only present fields are applied.
///
/// `Debug` is hand-written so a reset `password` is never printed.
#[typeshare]
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateUserRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<Role>>,
    /// Reset the account password to this value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl std::fmt::Debug for UpdateUserRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UpdateUserRequest")
            .field("roles", &self.roles)
            .field("password", &self.password.as_ref().map(|_| "<redacted>"))
            .finish()
    }
}

/// Body for `POST /api/v1/auth/change-password` — the caller changes their own
/// password.
///
/// `Debug` is hand-written so neither password is printed.
#[typeshare]
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

impl std::fmt::Debug for ChangePasswordRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChangePasswordRequest")
            .field("current_password", &"<redacted>")
            .field("new_password", &"<redacted>")
            .finish()
    }
}
