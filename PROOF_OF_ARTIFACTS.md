<!-- Unlicense — cochranblock.org -->

# Proof of Artifacts

*Visual and structural evidence that this project works, ships, and is real.*

> Edge intelligence scaffold with CLI, node identity, subsystem traits, 35 unit tests, and federal compliance documentation.

## Architecture

```mermaid
flowchart LR
    CLI[CLI / Android] --> Config[Node Config]
    Config --> Status[Status Report]
    Status --> Radio[Radio T1]
    Status --> Mesh[Mesh T2]
    Status --> Inference[Inference T6]
    Status --> Sensor[Sensor T4]
```

*Subsystem traits defined with mock implementations and tests. Hardware drivers not yet implemented. See WHITEPAPER.md for target architecture.*

### Target Architecture (not yet built)

```mermaid
flowchart LR
    Sensor[Sensor Data] --> Node[Ghost Fabric Node]
    Node --> Candle[Candle Inference]
    Candle --> Decision[Local Decision]
    Decision --> LoRa[LoRa 915MHz TX]
    LoRa --> Mesh[Cognitive Mesh]
    Mesh --> Node2[Adjacent Node]
    Node2 --> Action[Autonomous Action]
```

## Build Output

| Metric | Value |
|--------|-------|
| Binary size (release, stripped) | 470,080 bytes (459KB) |
| Binary size (pre-deps scaffold) | 285,936 bytes (279KB) |
| Target binary size (with weights) | 19MB (statically linked, embedded weights) |
| Runtime | Bare metal Rust — no interpreter, no GC |
| Radio band | 915MHz ISM/LoRa |
| Throughput | ~5.5 kbps (SF7/125kHz) |
| Cold-start target | <50ms |
| RAM target | 8–32MB |
| Python dependencies | Zero in production |
| Cloud dependencies | Zero |
| Android AAB (arm64-v8a) | 1,643,180 bytes (1.6MB) |
| Android .so (arm64-v8a) | 3,062,432 bytes (3MB) |

## Codebase Stats

| Metric | Value |
|--------|-------|
| Rust LOC (src/) | 1,101 |
| Source files | 8 (main.rs, lib.rs, config.rs, lifecycle.rs, radio.rs, mesh.rs, inference.rs, sensor.rs) |
| Public functions (P13 tokenized) | 17 (f0–f17) |
| Types (P13 tokenized) | 12 (T0–T11 including traits + mocks) |
| Fields (P13 tokenized) | 5 (s0–s4 including peers) |
| CLI commands | 3 (init, start, status) |
| Unit tests | 35 (config 7, radio 7, mesh 11, inference 6, sensor 4) |
| Direct dependencies | 6 (clap, dirs, libc, rand, serde, serde_json) |
| Transitive dependencies | ~49 |
| `unsafe` blocks (core) | 6 (lifecycle.rs: 4 libc::kill, main.rs: 2 SIGINT handler) |
| `unsafe` blocks (android) | 1 (set_var for HOME path) |

## Subsystem Implementation

| Subsystem | Trait | Mock | Tests | Status |
|-----------|-------|------|-------|--------|
| Radio | T1 (RadioDriver) | T8 (MockRadio) | 7 | Trait defined, no hardware driver |
| Mesh | T2 (MeshNetwork) | T9 (PeerTable) | 11 | In-memory peer table with route scoring |
| Sensor | T4 (SensorDriver) | T10 (MockSensor) | 4 | Trait defined, no GPIO/I2C/SPI driver |
| Inference | T6 (InferenceEngine) | T11 (MockEngine) | 6 | Trait defined, no Candle integration |
| Config | — | — | 7 | Validation + LoRa spec checking (f17) |
| **Total** | — | — | **35** | All tests passing |

## QA Results

### QA Round 1 (2026-03-27)

| Check | Result |
|-------|--------|
| `cargo build --release` | PASS — zero errors |
| `cargo clippy --release -- -D warnings` | PASS — zero warnings |
| `cargo test` | PASS — 0 tests at time of QA |
| P12 slop scan | PASS — "utilizing" fixed to "using" |
| Git status | PASS — clean |

### QA Round 2 (2026-03-27)

| Check | Result |
|-------|--------|
| `cargo clean && cargo build --release` | PASS — fresh compile, zero errors |
| `cargo clippy --release -- -D warnings` | PASS — zero warnings |
| Cargo.lock committed | PASS — tracked for reproducibility |
| Binary runs | PASS — prints version |

### Post-Feature QA (2026-03-29)

| Check | Result |
|-------|--------|
| `cargo build --release` | PASS — zero errors |
| `cargo clippy --release -- -D warnings` | PASS — zero warnings |
| `ghost-fabric --help` | PASS — shows subcommands |
| `ghost-fabric init` | PASS — generates node ID |
| `ghost-fabric status` | PASS — displays config |
| `ghost-fabric start` | PASS — reports subsystem status |

### Phase 1 QA (2026-04-02, P23 Triple Lens)

**Method:** P23 — guest analysis (pessimist), IRONHIVE swarm recon (optimist), security/unsafe audit (paranoia), then synthesis into prioritized action plan.

| Check | Result |
|-------|--------|
| `cargo build --release` | PASS — zero errors, 459KB binary |
| `cargo clippy -- -D warnings` | PASS — zero warnings |
| `cargo test` | PASS — 35 tests, 0 failures |
| `ghost-fabric start` + Ctrl+C | PASS — SIGINT handler, clean shutdown |
| Config validation | PASS — rejects invalid SF, BW, freq |

## P13 Tokenization Stats

| Category | Count | Range |
|----------|-------|-------|
| Functions | 17 | f0–f17 |
| Types | 12 | T0–T11 |
| Fields | 5 | s0–s4 |
| CLI commands | 3 | c0–c2 |
| Error variants | 0 | — |

Compression map: [docs/compression_map.md](docs/compression_map.md)

## Federal Compliance

12 documents in [`govdocs/`](govdocs/):

| Document | Framework |
|----------|-----------|
| SBOM.md | EO 14028 |
| SSDF.md | NIST SP 800-218 |
| SUPPLY_CHAIN.md | Supply chain integrity |
| SECURITY.md | Security posture |
| ACCESSIBILITY.md | Section 508 |
| PRIVACY.md | Privacy impact |
| FIPS.md | FIPS 140-2/3 |
| FedRAMP_NOTES.md | FedRAMP |
| CMMC.md | CMMC L1-L2 |
| ITAR_EAR.md | Export control |
| FEDERAL_USE_CASES.md | Agency use cases |
| SUPPLY_CHAIN_AUDIT.md | Deep code review of all deps |

## How to Verify

```bash
cargo build --release
./target/release/ghost-fabric init
./target/release/ghost-fabric status
./target/release/ghost-fabric start
./target/release/ghost-fabric --help
```

## Whitepaper

See [WHITEPAPER.md](WHITEPAPER.md) for the full technical argument.

---

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the Unlicense.*
