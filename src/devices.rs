//! Device types for the fleet.

use serde::{Deserialize, Serialize};

use crate::constraints::Constraint;

/// A device in the fleet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    pub kind: DeviceKind,
    pub capability: CapabilityLevel,
    pub constraints: Vec<Constraint>,
}

/// The kind of device.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceKind {
    RobotArm,
    SensorArray,
    SonarArray,
    Esp32Node,
    JetsonNode,
    FpgaNode,
    Unknown(String),
}

/// Capability level of a device.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityLevel {
    Raw,
    Aware,
    Enforcing,
    Commander,
}
