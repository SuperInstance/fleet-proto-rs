# fleet-proto — Shared Fleet Types for Rust

PLATO client, I2I messaging, constraint types, and device types.
One crate, every Rust fleet repo depends on it.

## What's Inside

- **PlatoClient**: HTTP client for PLATO room server
- **I2I messages**: inter-agent protocol (parse/render)
- **Constraints**: Eisenstein disk, joint limits, collision, temporal
- **Devices**: robot arms, sensors, ESP32, Jetson, FPGA nodes
- **Temporal**: `eval_time_ns` and `thermal_state_celsius` on every constraint result

## Use Cases

- Any Rust service that talks to PLATO
- Fleet constraint checking services
- Device discovery and registration
- I2I bottle parsing/generation

## Composable With

- **fleet-constraint-kernel**: GPU constraint evaluation (uses these types)
- **snap-lut + snap-lut-eisenstein**: FPGA constraint snapping
- **temporal-flux**: temporal opcodes that produce `ConstraintResult`
- **physics-clock**: reality parity verification using `eval_time_ns`
- **cocapn-schemas**: JSON schema validation for tiles

## Install

```toml
[dependencies]
fleet-proto = "0.1"
```

## Example

```rust
use fleet_proto::{I2iMessage, I2iType, I2iStatus, Constraint, ConstraintKind, Device, DeviceKind, CapabilityLevel};

// I2I message round-trip
let msg = I2iMessage {
    msg_type: I2iType::Deliverable,
    from: "forgemaster".into(),
    scope: "fleet-proto-rs".into(),
    summary: "crate shipped".into(),
    details: "tests pass".into(),
    status: I2iStatus::Complete,
};
let rendered = msg.render();
let parsed = I2iMessage::parse(&rendered).unwrap();

// Constraint with temporal tracking
let result = fleet_proto::ConstraintResult {
    constraint_id: "c1".into(),
    satisfied: true,
    margin: 0.25,
    eval_time_ns: 12345,  // physics-clock temporal fingerprint
};
```

## License

Apache 2.0
