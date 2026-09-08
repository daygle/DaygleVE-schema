//! Resource pools: named groupings of guests (VMs and containers) for
//! organization and, later, permission delegation.
//!
//! A pool is lightweight metadata — a name and an optional comment. Membership
//! lives on the guest (each `Vm`/`Lxc` carries an optional `pool` naming the
//! pool it belongs to), so a guest is in at most one pool. The pool `name` is
//! the stable identifier guests reference and is immutable once created;
//! updates change only the comment. A pool that still has members cannot be
//! deleted.

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::common::{ResourceId, Timestamp};

/// A resource pool record.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResourcePool {
    pub id: ResourceId,
    /// Unique, human-facing pool name. This is the value guests reference and
    /// is immutable once the pool exists.
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    pub created_at: Timestamp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<Timestamp>,
}

/// A pool with its current member count, for list views. The count is resolved
/// by scanning guests, so it reflects live membership rather than stored state.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResourcePoolSummary {
    pub id: ResourceId,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Number of guests (VMs + containers) currently assigned to this pool.
    pub member_count: u32,
    pub created_at: Timestamp,
}

/// The kind of guest a [`PoolMember`] refers to.
#[typeshare]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PoolMemberKind {
    Vm,
    Lxc,
}

/// One guest belonging to a pool, as returned by the pool detail endpoint.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PoolMember {
    pub kind: PoolMemberKind,
    pub id: ResourceId,
    pub name: String,
}

/// A pool plus its resolved members, from `GET /api/v1/pools/{id}`.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResourcePoolDetail {
    pub pool: ResourcePool,
    pub members: Vec<PoolMember>,
}

/// Body for `POST /api/v1/pools` — create a pool.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateResourcePoolRequest {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// Body for `PATCH /api/v1/pools/{id}` — update a pool. The name is immutable,
/// so only the comment can change. `comment: Some("")` clears the comment;
/// omitting the field leaves it unchanged.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdateResourcePoolRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}
