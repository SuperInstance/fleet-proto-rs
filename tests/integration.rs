use fleet_proto::*;

#[test]
fn room_round_trip() {
    let json = r#"{"id":"forgemaster-logs","tile_count":42,"latest_tile":"tile-99"}"#;
    let room: Room = serde_json::from_str(json).unwrap();
    assert_eq!(room.id, "forgemaster-logs");
    assert_eq!(room.tile_count, 42);
    assert_eq!(room.latest_tile.as_deref(), Some("tile-99"));

    let back = serde_json::to_string(&room).unwrap();
    assert!(back.contains("forgemaster-logs"));
}

#[test]
fn tile_round_trip() {
    let json = r#"{
        "id": "t1",
        "room_id": "r1",
        "content": {"msg": "hello"},
        "timestamp": 1715212800,
        "hash": "abc123"
    }"#;
    let tile: Tile = serde_json::from_str(json).unwrap();
    assert_eq!(tile.id, "t1");
    assert_eq!(tile.content["msg"], "hello");

    let back = serde_json::to_string(&tile).unwrap();
    let tile2: Tile = serde_json::from_str(&back).unwrap();
    assert_eq!(tile2.hash, "abc123");
}

#[test]
fn i2i_round_trip() {
    let msg = I2iMessage {
        msg_type: I2iType::Deliverable,
        from: "forgemaster".into(),
        scope: "fleet-proto-rs".into(),
        summary: "crate shipped".into(),
        details: "tests pass, pushed to github".into(),
        status: I2iStatus::Complete,
    };

    let rendered = msg.render();
    assert!(rendered.contains("[I2I:DELIVERABLE]"));
    assert!(rendered.contains("fleet-proto-rs"));
    assert!(rendered.contains("crate shipped"));

    let parsed = I2iMessage::parse(&rendered).unwrap();
    assert_eq!(parsed.msg_type, I2iType::Deliverable);
    assert_eq!(parsed.scope, "fleet-proto-rs");
    assert_eq!(parsed.summary, "crate shipped");
}

#[test]
fn i2i_parse_header() {
    let text = "[I2I:BLOCKER] guard2mask — build fails on rustc 1.75\nstatus: blocked\nneed uuid fix\n";
    let msg = I2iMessage::parse(text).unwrap();
    assert_eq!(msg.msg_type, I2iType::Blocker);
    assert_eq!(msg.scope, "guard2mask");
    assert!(msg.summary.contains("build fails"));
    assert_eq!(msg.status, I2iStatus::Blocked);
}

#[test]
fn constraint_round_trip() {
    let c = Constraint {
        id: "c1".into(),
        kind: ConstraintKind::EisensteinDisk,
        parameters: serde_json::json!({"radius": 1.5}),
    };
    let json = serde_json::to_string(&c).unwrap();
    let c2: Constraint = serde_json::from_str(&json).unwrap();
    assert_eq!(c2.id, "c1");
    assert!(matches!(c2.kind, ConstraintKind::EisensteinDisk));
}

#[test]
fn constraint_custom_kind() {
    let c = Constraint {
        id: "c2".into(),
        kind: ConstraintKind::Custom("my_constraint".into()),
        parameters: serde_json::json!(null),
    };
    let json = serde_json::to_string(&c).unwrap();
    let c2: Constraint = serde_json::from_str(&json).unwrap();
    assert!(matches!(c2.kind, ConstraintKind::Custom(ref s) if s == "my_constraint"));
}

#[test]
fn constraint_batch_temporal_fields() {
    let batch = ConstraintBatch {
        device_id: "arm-01".into(),
        results: vec![ConstraintResult {
            constraint_id: "c1".into(),
            satisfied: true,
            margin: 0.25,
            eval_time_ns: 12345,
        }],
        timestamp_ns: 1715212800000000000,
        thermal_state_celsius: 42.5,
    };
    let json = serde_json::to_string(&batch).unwrap();
    assert!(json.contains("eval_time_ns"));
    assert!(json.contains("thermal_state_celsius"));

    let b2: ConstraintBatch = serde_json::from_str(&json).unwrap();
    assert_eq!(b2.results[0].eval_time_ns, 12345);
    assert!((b2.thermal_state_celsius - 42.5).abs() < 0.001);
}

#[test]
fn device_with_constraints() {
    let d = Device {
        id: "arm-01".into(),
        kind: DeviceKind::RobotArm,
        capability: CapabilityLevel::Enforcing,
        constraints: vec![Constraint {
            id: "c1".into(),
            kind: ConstraintKind::JointLimit,
            parameters: serde_json::json!({"max_angle": 180.0}),
        }],
    };
    let json = serde_json::to_string(&d).unwrap();
    let d2: Device = serde_json::from_str(&json).unwrap();
    assert_eq!(d2.id, "arm-01");
    assert!(matches!(d2.kind, DeviceKind::RobotArm));
    assert_eq!(d2.capability, CapabilityLevel::Enforcing);
    assert_eq!(d2.constraints.len(), 1);
}

#[test]
fn capability_level_equality() {
    assert_eq!(CapabilityLevel::Enforcing, CapabilityLevel::Enforcing);
    assert_ne!(CapabilityLevel::Raw, CapabilityLevel::Commander);
}
