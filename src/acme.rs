//! ACME (Let's Encrypt) automatic TLS certificate types.
//!
//! DaygleVE can obtain and renew a TLS certificate for the node's own HTTPS
//! listener from an ACME certificate authority (Let's Encrypt by default),
//! solving the HTTP-01 challenge on the unauthenticated
//! `/.well-known/acme-challenge/{token}` route. The obtained certificate is
//! installed on the live listener and renewed automatically before expiry.
//!
//! The ACME account key and issued private key are node secrets: they are
//! stored on the node and never returned by the API. Only the non-secret
//! configuration and certificate metadata are surfaced here.

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::common::Timestamp;

/// The ACME directory URL for Let's Encrypt's production environment. Issues
/// certificates trusted by browsers; subject to strict rate limits.
pub const LETS_ENCRYPT_PRODUCTION: &str = "https://acme-v02.api.letsencrypt.org/directory";

/// The ACME directory URL for Let's Encrypt's staging environment. Issues
/// certificates from an untrusted test root, with far higher rate limits — use
/// it to validate a setup before switching to production.
pub const LETS_ENCRYPT_STAGING: &str = "https://acme-staging-v02.api.letsencrypt.org/directory";

/// Lifecycle state of the node's ACME-managed certificate.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AcmeState {
    /// ACME is switched off; TLS (if any) uses a statically configured
    /// certificate or the node serves plain HTTP.
    Disabled,
    /// ACME is enabled but no certificate is installed yet, or an issuance /
    /// renewal is currently in progress.
    Pending,
    /// A valid ACME certificate is installed and serving.
    Active,
    /// The most recent issuance or renewal failed; see `last_error`. Any
    /// previously issued certificate keeps serving until it expires.
    Error,
}

/// Non-secret ACME configuration. The account key and issued private key are
/// never included here.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AcmeConfig {
    /// Whether ACME certificate management is enabled.
    pub enabled: bool,
    /// ACME directory URL. Defaults to Let's Encrypt production; set to the
    /// staging URL while testing (see [`LETS_ENCRYPT_STAGING`]).
    pub directory_url: String,
    /// Contact email registered with the ACME account (expiry notices, etc.).
    pub contact_email: String,
    /// Fully-qualified domain names to include on the certificate. The first is
    /// the primary (subject common name); the rest are subject-alternative
    /// names. At least one is required to issue.
    pub domains: Vec<String>,
    /// Whether the operator has agreed to the CA's terms of service. Required
    /// before an account can be registered.
    pub terms_agreed: bool,
    /// Renew the certificate when fewer than this many days remain before
    /// expiry. Defaults to 30.
    pub renewal_days: u32,
}

/// Body for `PUT /api/v1/security/acme/config` — replace the ACME
/// configuration. Changing the domains or directory triggers a fresh issuance
/// on the next renewal tick (or immediately via the issue endpoint).
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateAcmeConfigRequest {
    pub enabled: bool,
    pub directory_url: String,
    pub contact_email: String,
    pub domains: Vec<String>,
    pub terms_agreed: bool,
    /// Renew when fewer than this many days remain. Omit to keep the default
    /// (30). Must be between 1 and 89.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renewal_days: Option<u32>,
}

/// Runtime status of the node's ACME certificate, from
/// `GET /api/v1/security/acme`.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AcmeStatus {
    /// The current non-secret configuration.
    pub config: AcmeConfig,
    /// The lifecycle state of the managed certificate.
    pub state: AcmeState,
    /// Domains present on the currently installed certificate (may differ from
    /// `config.domains` until the next issuance completes).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub certificate_domains: Vec<String>,
    /// When the installed certificate was issued.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<Timestamp>,
    /// When the installed certificate expires.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<Timestamp>,
    /// When DaygleVE last successfully issued or renewed the certificate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_renewal_at: Option<Timestamp>,
    /// Human-readable message from the most recent issuance/renewal failure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
    /// Whether an ACME account has been registered with the CA.
    pub account_registered: bool,
}
