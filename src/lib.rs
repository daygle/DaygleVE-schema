//! # DaygleVE Schema
//!
//! Shared API type definitions for the DaygleVE single-node virtualization
//! platform. This crate is the single source of truth for every
//! request/response body, enum, and error shape crossing the API boundary
//! between [`DaygleVE-backend`] (Rust) and [`DaygleVE-frontend`] (TypeScript).
//!
//! ## Rules
//!
//! - **Types only.** No business logic, no I/O, no framework or UI code.
//! - Every public type derives [`serde::Serialize`] + [`serde::Deserialize`]
//!   and is annotated with `#[typeshare]` so TypeScript bindings can be
//!   generated (`scripts/generate.sh` → `generated/typescript`).
//! - Backend and frontend both import from here; they never redefine these
//!   shapes locally.
//!
//! ## Modules
//!
//! - [`common`] — API version, error envelope, pagination, id/time helpers.
//! - [`auth`] — login, tokens, users, roles and RBAC permissions.
//! - [`vm`]    — KVM/QEMU virtual machine lifecycle types.
//! - [`lxc`]   — LXC container lifecycle types.
//! - [`storage`] — ZFS pools, datasets, snapshots and clones.
//! - [`network`] — Linux bridges and VLANs.
//! - [`gpu`]   — GPU passthrough inventory and assignment.
//! - [`metrics`] — CPU, RAM, disk, network and guest-state metrics.
//! - [`operations`] — durable host-operation lifecycle and recovery records.
//!
//! [`DaygleVE-backend`]: https://github.com/daygle/DaygleVE-backend
//! [`DaygleVE-frontend`]: https://github.com/daygle/DaygleVE-frontend

pub mod auth;
pub mod backup;
pub mod broker;
pub mod common;
pub mod gpu;
pub mod lxc;
pub mod lxc_snapshot;
pub mod metrics;
pub mod network;
pub mod operations;
pub mod share;
pub mod storage;
pub mod vm;

/// The API version this schema describes. Kept in sync with
/// [`common::API_VERSION`]; see `docs/VERSIONING.md`.
pub use common::API_VERSION;
