<!-- Unlicense — cochranblock.org -->

# Ghost Fabric — Compression Map (P13)

## Functions (f+num)

| Token | Name | Location |
|-------|------|----------|
| f0 | main (entry point) | src/main.rs |
| f1 | start (start mesh node) | src/main.rs |
| f2 | status (display node identity) | src/main.rs |
| f3 | init (generate node config) | src/config.rs |
| f4 | load (load node config from disk) | src/config.rs |
| f5 | radio_status | src/radio.rs |
| f6 | mesh_status | src/mesh.rs |
| f7 | inference_status | src/inference.rs |
| f8 | sensor_status | src/sensor.rs |
| f9 | config_path | src/config.rs |
| f10 | node_status_string | src/lib.rs |
| f11 | init_and_report | src/lib.rs |
| f12 | android_main | android/src/lib.rs |
| f13 | pid_path | src/lifecycle.rs |
| f14 | acquire (PID lock + SIGTERM old) | src/lifecycle.rs |
| f15 | signal_old (SIGTERM/SIGKILL) | src/lifecycle.rs |
| f16 | release (remove PID lockfile) | src/lifecycle.rs |
| f17 | validate (check config LoRa spec) | src/config.rs |
| f18 | encode (frame to CBOR bytes) | src/packet.rs |
| f19 | decode (CBOR bytes to frame) | src/packet.rs |
| f20 | beacon (create beacon frame) | src/packet.rs |
| f21 | data_frame (create data frame) | src/packet.rs |

## Types (t+num)

| Token | Name | Location |
|-------|------|----------|
| T0 | NodeConfig | src/config.rs |
| T1 | RadioDriver | src/radio.rs |
| T2 | MeshNetwork | src/mesh.rs |
| T3 | Peer | src/mesh.rs |
| T4 | SensorDriver | src/sensor.rs |
| T5 | SensorReading | src/sensor.rs |
| T6 | InferenceEngine | src/inference.rs |
| T7 | Prediction | src/inference.rs |
| T8 | MockRadio | src/radio.rs |
| T9 | PeerTable | src/mesh.rs |
| T10 | MockSensor | src/sensor.rs |
| T11 | MockEngine | src/inference.rs |
| T12 | Frame (wire format) | src/packet.rs |
| T13 | FrameKind (packet type tag) | src/packet.rs |
| T14 | Payload (typed variants) | src/packet.rs |

## Fields (s+num)

| Token | Name | Type | Parent |
|-------|------|------|--------|
| s0 | node_id | String | T0 |
| s1 | frequency_mhz | u32 | T0 |
| s2 | spreading_factor | u8 | T0 |
| s3 | bandwidth_khz | u32 | T0 |
| s4 | peers | Vec\<String\> | T0 |

## Error Variants (E+num)

*None yet.*

## CLI Commands (c+num)

| Token | Name | Description |
|-------|------|-------------|
| c0 | init | Generate node config |
| c1 | start | Start mesh node |
| c2 | status | Show node identity |
