//! Shared constraint types for fleet-wide consistency.
//!
//! These types are the lingua franca between all fleet repos.
//! Same types used by guard2mask, flux-vm, fleet-constraint-kernel, etc.

use serde::{Deserialize, Serialize};

/// A single constraint definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    pub id: String,
    pub kind: ConstraintKind,
    pub parameters: serde_json::Value,
}

/// The kind of constraint.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConstraintKind {
    EisensteinDisk,
    JointLimit,
    CollisionAvoid,
    TorqueLimit,
    SpeedLimit,
    Temporal,
    Custom(String),
}

/// Result of evaluating a single constraint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintResult {
    pub constraint_id: String,
    pub satisfied: bool,
    pub margin: f64,
    pub eval_time_ns: u64,
}

/// A batch of constraint results from a device.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintBatch {
    pub device_id: String,
    pub results: Vec<ConstraintResult>,
    pub timestamp_ns: u64,
    pub thermal_state_celsius: f64,
}
