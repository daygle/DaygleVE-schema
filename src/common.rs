//! Cross-cutting types: API versioning, the error envelope, pagination and
//! shared scalar aliases used by every other module.

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// The current major API version string, embedded in the base path
/// (`/api/v1`). Bumped only on a breaking change — see `docs/VERSIONING.md`.
pub const API_VERSION: &str = "v1";

/// RFC 3339 / ISO-8601 timestamp, e.g. `2026-09-01T12:00:00Z`.
///
/// Kept as a string alias so the schema crate stays dependency-light; the
/// backend parses/serialises via `chrono`, the frontend via `Date`.
#[typeshare]
pub type Timestamp = String;

/// Opaque identifier for a platform resource (VM, container, dataset, …).
///
/// UUID v4 in canonical hyphenated form. Treated as an opaque string by
/// clients — never parse structure out of it.
#[typeshare]
pub type ResourceId = String;

/// Standard error envelope returned by every non-2xx API response.
///
/// The HTTP status code conveys the coarse category; [`code`](ApiError::code)
/// gives a stable, machine-readable reason that clients may switch on.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApiError {
    /// Stable, machine-readable error kind.
    pub code: ErrorCode,
    /// Human-readable description, safe to surface in a UI.
    pub message: String,
    /// Optional per-field validation details (`field name` → `reason`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<FieldError>,
    /// Correlation id echoing the `x-request-id` header, for log tracing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

/// A single field-level validation problem inside an [`ApiError`].
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FieldError {
    /// Dotted path to the offending field, e.g. `spec.memory_mib`.
    pub field: String,
    /// Why the field was rejected.
    pub message: String,
}

/// Stable, machine-readable error categories. New variants may be appended
/// within a major version; clients must tolerate unknown values.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    /// Request body or parameters failed validation.
    Validation,
    /// Authentication is missing or invalid.
    Unauthorized,
    /// Authenticated, but the subject lacks the required permission.
    Forbidden,
    /// The referenced resource does not exist.
    NotFound,
    /// The request conflicts with current resource state.
    Conflict,
    /// A dependency (hypervisor, ZFS, network) reported a failure.
    HypervisorError,
    /// Anything unclassified / internal.
    Internal,
}

/// Query parameters accepted by every list endpoint.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PageQuery {
    /// 1-based page index. Defaults to `1` when omitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Page size. Server clamps to a sane maximum. Defaults to `50`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
}

/// Envelope wrapping a page of results from a list endpoint.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Page<T> {
    /// The items on this page.
    pub items: Vec<T>,
    /// 1-based index of this page.
    pub page: u32,
    /// Requested page size.
    pub per_page: u32,
    /// Total number of items across all pages.
    pub total: u64,
}

/// Health/readiness payload returned by `GET /api/v1/health`.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HealthStatus {
    /// `true` when the node is ready to serve API traffic.
    pub healthy: bool,
    /// Running backend version (crate version).
    pub version: String,
    /// API version served, e.g. `v1`.
    pub api_version: String,
    /// Seconds the backend process has been running.
    pub uptime_seconds: u64,
}
