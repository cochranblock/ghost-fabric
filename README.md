# Ghost Fabric

Sovereign edge intelligence over sub-GHz cognitive mesh networks. Rust CLI + Android app for LoRa 915MHz mesh node management. Core subsystem traits (radio, mesh, inference, sensor) are defined with mock implementations for testing. Hardware drivers are planned.

**Stack:** Rust, clap CLI, egui (Android), serde/JSON config
**License:** Unlicense

## Quick Start

```bash
cargo build --release
ghost-fabric init      # generate node identity
ghost-fabric status    # show node config
ghost-fabric start     # start mesh node (Ctrl+C to stop)
```

## Platforms

| Target | Artifact | Size |
|--------|----------|------|
| macOS/Linux CLI | `ghost-fabric` binary | 459 KB |
| Android (arm64-v8a) | `app-release.aab` | 1.6 MB |

## Features

- **Node identity**: generates `gf-{hex}` node ID, persists JSON config
- **CLI**: `init`, `start`, `status` subcommands with `--help`
- **Hot reload**: PID lockfile + SIGTERM/SIGKILL lifecycle — deploy by running the new binary
- **Graceful shutdown**: SIGINT handler for clean Ctrl+C exit
- **Config validation**: LoRa spec enforcement (SF 6-12, valid bandwidths, frequency range)
- **Android app**: NativeActivity + egui, auto-inits node on launch
- **P13 tokenization**: compressed symbol names per Kova conventions

## Subsystem Traits

Each subsystem defines a trait for hardware abstraction and a mock for testing:

| Module | Trait | Mock | Tests | Status |
|--------|-------|------|-------|--------|
| `radio.rs` | `T1` (RadioDriver) | `T8` (MockRadio) | 7 | Trait defined, no hardware driver |
| `mesh.rs` | `T2` (MeshNetwork) | `T9` (PeerTable) | 11 | In-memory peer table with route scoring |
| `inference.rs` | `T6` (InferenceEngine) | `T11` (MockEngine) | 6 | Trait defined, no Candle integration |
| `sensor.rs` | `T4` (SensorDriver) | `T10` (MockSensor) | 4 | Trait defined, no GPIO/I2C/SPI driver |
| `config.rs` | — | — | 7 | Config persistence + LoRa validation |

**35 unit tests**, all passing.

## Mesh Routing

The peer table (`T9`) scores routes by:
- RSSI signal strength (-120dBm to -40dBm)
- Battery level (0-100%)
- Contact freshness (decays over 10 minutes)
- Hop count penalty

Direct peer matches are preferred; otherwise the highest-scoring peer is used as relay.

## Federal Compliance

See [`govdocs/`](govdocs/) — SBOM, SSDF, FIPS, CMMC, FedRAMP, ITAR/EAR, supply chain audit (deep code review), security posture, privacy assessment, and federal use cases.

---

Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. 14 Unlicense repos. [See all products →](https://cochranblock.org/products)
