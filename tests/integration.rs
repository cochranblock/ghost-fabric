//! Integration tests — end-to-end subsystem verification.
//! Tests config round-trip via filesystem, radio→mesh→packet pipeline,
//! and multi-peer mesh scenarios.

use ghost_fabric_core::config;
use ghost_fabric_core::mesh::{self, T2};
use ghost_fabric_core::packet::{self, T16};
use ghost_fabric_core::radio::{self, T1};
use ghost_fabric_core::sensor::{self, T4};
use ghost_fabric_core::inference::{self, T6};

/// Init config to temp dir, reload it, verify identity persists.
#[test]
fn config_init_reload() {
    let dir = tempfile::tempdir().unwrap();
    let result = ghost_fabric_core::f11(Some(dir.path().to_path_buf()));
    assert!(result.starts_with("Initialized node: gf-"));

    // Second call should find existing config
    let result2 = ghost_fabric_core::f11(Some(dir.path().to_path_buf()));
    assert!(result2.starts_with("Node already initialized: gf-"));

    // Both should report the same node ID
    let id1 = result.strip_prefix("Initialized node: ").unwrap();
    let id2 = result2.strip_prefix("Node already initialized: ").unwrap();
    assert_eq!(id1, id2);
}

/// Full pipeline: radio init → send beacon → decode → update peers.
#[test]
fn radio_to_peer_table_pipeline() {
    let mut radio = radio::T8::new();
    radio.init(915, 7, 125).unwrap();

    let mut peers = mesh::T9::new();
    assert_eq!(peers.peer_count(), 0);

    // Simulate node "gf-remote" sending a beacon
    let beacon = packet::T12::f20("gf-remote", 85, 1, 1);
    let bytes = packet::f18(&beacon).unwrap();

    // Inject into radio RX buffer
    radio.inject_rx(bytes, -72);

    // Receive and process
    let data = radio.recv(1000).unwrap().unwrap();
    let frame = packet::f19(&data).unwrap();

    assert_eq!(frame.src, "gf-remote");
    assert_eq!(frame.kind, packet::T13::Beacon);

    // Update peer table from beacon
    if let packet::T14::Beacon { battery_pct, hop_count } = frame.payload {
        let mut peer = mesh::T3::new(&frame.src, radio.last_rssi().unwrap_or(-100), battery_pct);
        peer.hop_count = hop_count;
        peers.add_peer(peer);
    }

    assert_eq!(peers.peer_count(), 1);
    let peer = peers.get_peer("gf-remote").unwrap();
    assert_eq!(peer.rssi, -72);
    assert_eq!(peer.battery_pct, 85);
    assert_eq!(peer.hop_count, 1);
}

/// Multi-peer mesh: add 3 peers, verify routing picks strongest signal.
#[test]
fn multi_peer_route_selection() {
    let mut peers = mesh::T9::new();

    // Three peers at different signal strengths
    peers.add_peer(mesh::T3::new("gf-close", -50, 90));
    peers.add_peer(mesh::T3::new("gf-mid", -80, 70));
    peers.add_peer(mesh::T3::new("gf-far", -110, 30));

    assert_eq!(peers.peer_count(), 3);
    assert_eq!(peers.status(), "online (3 peers)");

    // Route to unknown destination should pick best relay (gf-close)
    let best = peers.best_route("gf-unknown").unwrap();
    assert_eq!(best.node_id, "gf-close");

    // Route to known peer should prefer direct
    let direct = peers.best_route("gf-far").unwrap();
    assert_eq!(direct.node_id, "gf-far");
}

/// Ping/pong discovery: send ping, receive pong, update peers.
#[test]
fn ping_pong_discovery() {
    let mut radio = radio::T8::new();
    radio.init(915, 7, 125).unwrap();

    let mut peers = mesh::T9::new();

    // Send ping
    let ping = packet::T12::ping("gf-local", 1);
    let ping_bytes = packet::f18(&ping).unwrap();
    radio.send(&ping_bytes).unwrap();

    // Simulate remote sending pong
    let pong = packet::T12::pong("gf-remote", "gf-local", 95, 2, 1);
    let pong_bytes = packet::f18(&pong).unwrap();
    radio.inject_rx(pong_bytes, -65);

    // Process pong
    let data = radio.recv(1000).unwrap().unwrap();
    let frame = packet::f19(&data).unwrap();
    assert_eq!(frame.kind, packet::T13::Pong);

    if let packet::T14::Pong { battery_pct, .. } = frame.payload {
        peers.add_peer(mesh::T3::new(&frame.src, -65, battery_pct));
    }

    assert_eq!(peers.peer_count(), 1);
    let peer = peers.get_peer("gf-remote").unwrap();
    assert_eq!(peer.battery_pct, 95);
}

/// Data frame: send sensor reading over mesh.
#[test]
fn sensor_to_data_frame() {
    let mut sensor = sensor::T10::new("temp", "C", vec![22.5, 23.0]);
    let reading = sensor.read().unwrap();

    let frame = packet::T12::f21("gf-local", "gf-remote", &reading.name, reading.value, &reading.unit, 1);
    let bytes = packet::f18(&frame).unwrap();
    let decoded = packet::f19(&bytes).unwrap();

    assert_eq!(decoded.kind, packet::T13::Data);
    if let packet::T14::Data { name, value, unit } = decoded.payload {
        assert_eq!(name, "temp");
        assert!((value - 22.5).abs() < f64::EPSILON);
        assert_eq!(unit, "C");
    } else {
        panic!("wrong payload type");
    }
}

/// Inference mock: load model, predict, send result as data frame.
#[test]
fn inference_to_data_frame() {
    let mut engine = inference::T11::new();
    engine.load_model("anomaly-3b", b"fake weights").unwrap();
    engine.set_predictions(vec![("alert".into(), 0.95), ("normal".into(), 0.05)]);

    let preds = engine.predict(&[1.0, 2.0, 3.0]).unwrap();
    let top = &preds[0];

    let frame = packet::T12::f21(
        "gf-local", "*",
        &top.label, top.confidence as f64, "confidence",
        1,
    );
    let bytes = packet::f18(&frame).unwrap();
    assert!(bytes.len() < packet::MAX_FRAME_BYTES);

    let decoded = packet::f19(&bytes).unwrap();
    assert!(decoded.is_broadcast());
}

/// TTL relay: verify frames don't relay forever.
#[test]
fn ttl_relay_chain() {
    let mut radio = radio::T8::new();
    radio.init(915, 7, 125).unwrap();

    // Beacon with TTL=3
    let beacon = packet::T12::f20("gf-origin", 80, 1, 1);
    assert_eq!(beacon.ttl, 3);

    let bytes = packet::f18(&beacon).unwrap();
    radio.inject_rx(bytes, -80);
    let data = radio.recv(0).unwrap().unwrap();
    let mut frame = packet::f19(&data).unwrap();

    // Relay hop 1: TTL 3→2
    assert!(frame.relay_hop());
    assert_eq!(frame.ttl, 2);

    // Relay hop 2: TTL 2→1
    assert!(frame.relay_hop());
    assert_eq!(frame.ttl, 1);

    // Relay hop 3: TTL 1→0, should return false
    assert!(!frame.relay_hop());
    assert_eq!(frame.ttl, 0);
}

/// Sync propagation: node A knows peers, broadcasts sync, node B learns them.
#[test]
fn sync_state_propagation() {
    // Node A has two known peers
    let mut node_a = mesh::T9::new();
    node_a.add_peer(mesh::T3::new("gf-x1", -55, 88));
    node_a.add_peer(mesh::T3::new("gf-x2", -75, 65));

    // A exports its peer table and creates a sync frame
    let entries = node_a.f24();
    assert_eq!(entries.len(), 2);

    let sync_frame = packet::T12::f23("gf-node-a", entries, 1);
    let bytes = packet::f18(&sync_frame).unwrap();
    assert!(bytes.len() < packet::MAX_FRAME_BYTES);

    // Frame transmitted over radio
    let mut radio = radio::T8::new();
    radio.init(915, 7, 125).unwrap();
    radio.send(&bytes).unwrap();

    // Node B receives it
    let tx = radio.drain_tx();
    let received = packet::f19(&tx[0]).unwrap();
    assert_eq!(received.kind, packet::T13::Sync);

    // Node B imports the peer table
    let mut node_b = mesh::T9::new();
    if let packet::T14::Sync { peers } = received.payload {
        let added = node_b.f25(&peers, &received.src);
        assert_eq!(added, 2);
    } else {
        panic!("expected Sync payload");
    }

    assert_eq!(node_b.peer_count(), 2);
    // Hop count incremented for relayed peers
    let x1 = node_b.get_peer("gf-x1").unwrap();
    assert_eq!(x1.hop_count, 2); // 1 hop from node_a + 1

    let x2 = node_b.get_peer("gf-x2").unwrap();
    assert_eq!(x2.hop_count, 2);
}

/// Sync dedup: node B already knows gf-x1, only adds gf-x2.
#[test]
fn sync_dedup_existing_peers() {
    let mut node_b = mesh::T9::new();
    node_b.add_peer(mesh::T3::new("gf-x1", -60, 90));

    let entries = vec![
        T16 { id: "gf-x1".into(), rssi: -55, battery: 88, hops: 1 },
        T16 { id: "gf-x2".into(), rssi: -75, battery: 65, hops: 1 },
    ];
    let added = node_b.f25(&entries, "gf-node-a");
    assert_eq!(added, 1);
    assert_eq!(node_b.peer_count(), 2);
}

/// Config validation rejects out-of-spec parameters.
#[test]
fn config_validation_rejects_bad_params() {
    let mut cfg = config::T0::default();

    // Valid config passes
    config::f17(&cfg).unwrap();

    // Bad SF
    cfg.spreading_factor = 15;
    assert!(config::f17(&cfg).is_err());
    cfg.spreading_factor = 7;

    // Bad bandwidth
    cfg.bandwidth_khz = 999;
    assert!(config::f17(&cfg).is_err());
    cfg.bandwidth_khz = 125;

    // Bad frequency
    cfg.frequency_mhz = 0;
    assert!(config::f17(&cfg).is_err());
}
