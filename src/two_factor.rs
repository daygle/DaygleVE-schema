//! Two-factor authentication (TOTP) enrollment types.
//!
//! DaygleVE supports time-based one-time passwords (RFC 6238) as a second
//! factor on top of the password login. Enrollment is a two-step handshake so a
//! misconfigured authenticator can never lock an account out:
//!
//! 1. `POST /api/v1/auth/2fa/setup` returns a fresh shared secret and an
//!    `otpauth://` provisioning URI (for a QR code). The secret is stored in a
//!    *pending*, not-yet-active state — login is unaffected until it is
//!    confirmed.
//! 2. `POST /api/v1/auth/2fa/confirm` verifies a code generated from that
//!    secret, activates the second factor, and returns one-time recovery codes.
//!
//! Once enabled, `POST /api/v1/auth/login` requires a valid TOTP code (or an
//! unused recovery code) in addition to the password. Disabling
//! (`POST /api/v1/auth/2fa/disable`) requires a current code to prove the
//! caller still controls the authenticator.
//!
//! Secrets and recovery codes are returned to the client exactly once, at the
//! point they are generated; the server keeps only what it needs to verify
//! (the shared secret, and hashes of the recovery codes).

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// Response to `POST /api/v1/auth/2fa/setup`. Carries the freshly-generated
/// shared secret needed to configure an authenticator app. The second factor is
/// **not** active until confirmed — see [`ConfirmTwoFactorRequest`].
///
/// `Debug` is hand-written so the secret is never printed.
#[typeshare]
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TwoFactorSetupResponse {
    /// The base32-encoded shared secret to enter manually into an authenticator.
    pub secret: String,
    /// `otpauth://totp/...` provisioning URI encoding the same secret, issuer
    /// and account label — render it as a QR code for easy enrollment.
    pub otpauth_uri: String,
}

impl std::fmt::Debug for TwoFactorSetupResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TwoFactorSetupResponse")
            .field("secret", &"<redacted>")
            .field("otpauth_uri", &"<redacted>")
            .finish()
    }
}

/// Body for `POST /api/v1/auth/2fa/confirm` — finish enrollment by proving the
/// authenticator is configured correctly. On success the second factor becomes
/// active and recovery codes are issued.
///
/// `Debug` is hand-written so the code is never printed.
#[typeshare]
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConfirmTwoFactorRequest {
    /// The current 6-digit code shown by the authenticator app.
    pub code: String,
}

impl std::fmt::Debug for ConfirmTwoFactorRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConfirmTwoFactorRequest")
            .field("code", &"<redacted>")
            .finish()
    }
}

/// Response to `POST /api/v1/auth/2fa/confirm`. Carries one-time recovery codes
/// that let the account holder log in if they lose their authenticator. Each
/// code works once; store them somewhere safe — they are shown **only here**.
///
/// `Debug` is hand-written so the codes are never printed.
#[typeshare]
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TwoFactorEnabledResponse {
    /// Single-use recovery codes. The server keeps only their hashes and can
    /// never redisplay them.
    pub recovery_codes: Vec<String>,
}

impl std::fmt::Debug for TwoFactorEnabledResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TwoFactorEnabledResponse")
            .field("recovery_codes", &"<redacted>")
            .finish()
    }
}

/// Body for `POST /api/v1/auth/2fa/disable` — turn off the second factor. A
/// current TOTP code (or an unused recovery code) is required so a walk-up
/// attacker with only a live session cannot silently remove the protection.
///
/// `Debug` is hand-written so the code is never printed.
#[typeshare]
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DisableTwoFactorRequest {
    /// A current 6-digit authenticator code, or one of the unused recovery
    /// codes.
    pub code: String,
}

impl std::fmt::Debug for DisableTwoFactorRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DisableTwoFactorRequest")
            .field("code", &"<redacted>")
            .finish()
    }
}
