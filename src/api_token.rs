//! Long-lived API tokens for programmatic access.
//!
//! Unlike the short-lived, in-memory session tokens minted at login, an API
//! token is persisted and survives restarts, so automation (CI, scripts, IaC)
//! can authenticate without a password. A token is owned by a user account and
//! carries a subset of that user's effective permissions — it can never grant
//! more than its creator holds.
//!
//! The raw token secret is returned exactly once, at creation
//! ([`CreateApiTokenResponse::token`]); only a hash is stored, so the server can
//! never show it again. Every other type here is non-secret metadata.

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::auth::Permission;
use crate::common::{ResourceId, Timestamp};

/// Non-secret metadata for one API token.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApiToken {
    pub id: ResourceId,
    /// Human-readable label chosen at creation, e.g. `ci-deploy`.
    pub name: String,
    /// Non-secret display prefix of the raw token (e.g. `dgv_a1b2c3d4`), enough
    /// to recognize a token without revealing it.
    pub prefix: String,
    /// Permissions this token grants (a subset of the owner's at creation time).
    pub permissions: Vec<Permission>,
    /// Username of the account that owns the token.
    pub owner: String,
    pub created_at: Timestamp,
    /// When the token expires, if ever. Absent means it does not expire.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Timestamp>,
    /// When the token was last used to authenticate, if ever. Updated at most
    /// periodically, so it is approximate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<Timestamp>,
}

/// Body for `POST /api/v1/tokens` — create an API token.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateApiTokenRequest {
    /// Label for the token (1–64 characters).
    pub name: String,
    /// Permissions to grant. Omit or leave empty to grant all of the caller's
    /// own permissions. Any listed permission the caller lacks is rejected — a
    /// token can never escalate beyond its creator.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permissions: Vec<Permission>,
    /// Days until the token expires. Omit for a token that does not expire.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_in_days: Option<u32>,
}

/// Response to `POST /api/v1/tokens`. The `token` is the raw bearer secret and
/// is shown **only here** — it cannot be retrieved again.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateApiTokenResponse {
    /// The raw bearer token. Send it as `Authorization: Bearer <token>`. Store
    /// it now; the server keeps only a hash and can never redisplay it.
    pub token: String,
    /// The stored metadata for the new token.
    pub api_token: ApiToken,
}
