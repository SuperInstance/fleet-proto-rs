//! # fleet-proto — Shared Fleet Types for Rust
//!
//! PLATO client, I2I messaging, constraint types, and device types.
//! One crate, every Rust fleet repo depends on it.

pub mod constraints;
pub mod devices;
pub mod i2i;
pub mod plato;

pub use constraints::{Constraint, ConstraintBatch, ConstraintKind, ConstraintResult};
pub use devices::{CapabilityLevel, Device, DeviceKind};
pub use i2i::{I2iMessage, I2iStatus, I2iType};
pub use plato::{PlatoClient, Room, Tile};
