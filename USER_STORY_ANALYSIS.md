<!-- Unlicense — cochranblock.org -->

# User Story Analysis

*Full end-to-end user walkthrough. Performed 2026-03-27.*

---

## 1. Discovery

**How does a user find this?** GitHub search or cochranblock.org products page.

**First impression (README.md):** One-liner: "Sovereign edge intelligence over sub-GHz cognitive mesh networks." Clear what the domain is in 10 seconds — LoRa mesh + edge AI + Rust. Missing: no build instructions, no usage examples, no "getting started" section. A technical reader gets the vision; a practical reader has nothing to do next.

**WHITEPAPER.md:** Strong. Reads like a position paper, not marketing fluff. Makes a clear technical argument. The 915MHz bandwidth constraint as a forcing function is the strongest section.

**Score: 6/10** — vision is clear, actionable next steps are not.

## 2. Installation

```bash
cargo build --release   # works, 0.26s, zero deps
cargo run --release      # prints "ghost-fabric v0.1.0"
```

**Does --help work?** No. `--help`, `-h`, `--version` all print the same version string. No argument parsing exists. The binary ignores all input.

**Score: 3/10** — it compiles and runs, but does nothing.

## 3. First Use (Happy Path)

There is no happy path. The binary prints a version string and exits. There is no:
- CLI interface
- Configuration file
- Subcommands
- Interactive mode
- Network listener
- Sensor reader

A user who cloned this repo expecting to run a mesh node would immediately realize this is a scaffold only.

**Score: 1/10** — no functionality to exercise.

## 4. Second Use Case

A user's second attempt would be to read the whitepaper and try to extend the scaffold. The project structure (single main.rs, zero deps) makes this easy to fork but provides no foundation to build on — no traits, no module structure, no abstractions to plug into.

**Score: 2/10** — clean slate, but nothing to build on.

## 5. Edge Cases (5 tested)

| Input | Expected | Actual | Verdict |
|-------|----------|--------|---------|
| `--help` | Help text with usage | Prints version | FAIL — no arg parsing |
| `--version` | Version string | Prints version (same output) | ACCIDENTAL PASS |
| `status` subcommand | Error or status info | Prints version, ignores input | FAIL |
| `--config /nonexistent` | Error about missing file | Prints version, ignores input | FAIL |
| Empty stdin pipe | Should not hang | Does not hang | PASS |

**Score: 2/10** — doesn't crash, but ignores all input silently.

## 6. Feature Gap Analysis

What a user expects vs. what exists:

| Expected Feature | Status |
|-----------------|--------|
| LoRa radio initialization | Missing |
| Mesh node discovery | Missing |
| Sensor data ingestion | Missing |
| On-device inference (Candle) | Missing |
| Decision routing | Missing |
| CLI with subcommands (start, status, config) | Missing |
| Configuration file (node ID, radio params, model path) | Missing |
| Logging / diagnostics | Missing |
| Graceful shutdown | Missing |
| Health check endpoint | Missing |

## 7. Documentation Gaps

Questions a user would have that docs don't answer:

1. How do I configure a node? (No config system)
2. What hardware do I need? (Not specified)
3. How do I connect two nodes? (No networking code)
4. What model format is supported? (No inference code)
5. What LoRa modules are compatible? (Not specified)
6. How do I deploy this to an ARM device? (No cross-compile docs)
7. What's the roadmap? (No milestones or issues)

## 8. Competitor Check

| Product | What It Does | How Ghost Fabric Compares |
|---------|-------------|--------------------------|
| Meshtastic | LoRa mesh messaging, open source, runs on ESP32 | Production-ready, large community. Ghost Fabric is whitepaper-only. |
| Edge Impulse | On-device ML for embedded | Full toolchain for model training + deployment. Ghost Fabric has no ML pipeline. |
| TensorFlow Lite Micro | Inference on microcontrollers | Battle-tested inference runtime. Ghost Fabric names Candle but doesn't use it. |
| Renode | Embedded systems simulation | Different domain but solves the "test without hardware" problem Ghost Fabric ignores. |

**Honest assessment:** Ghost Fabric's thesis (Rust + LoRa + edge AI in one binary) is differentiated. No competitor does exactly this. But competitors have working code. Ghost Fabric has a whitepaper.

## 9. Verdict

| Category | Score (1-10) | Notes |
|----------|-------------|-------|
| Usability | 1 | No features to use |
| Completeness | 1 | Scaffold only |
| Error Handling | 2 | Doesn't crash, but ignores everything |
| Documentation | 5 | Whitepaper is solid; build/usage docs missing |
| Would You Pay For This? | 1 | Not today — nothing to buy |
| **Overall** | **2** | Strong thesis, zero execution |

## 10. Top 3 Fixes to Make This Shippable

### Fix 1: CLI with argument parsing
Add clap for subcommands: `ghost-fabric start`, `ghost-fabric status`, `ghost-fabric --help`. A user needs to feel like they're holding a real tool.

### Fix 2: Node identity and config
Generate a node ID on first run. Write a config file. Give the node a name. This is the foundation for everything — mesh, routing, identity.

### Fix 3: Scaffold the module structure
Create empty but typed modules: `radio.rs`, `mesh.rs`, `inference.rs`, `sensor.rs`, `config.rs`. Define traits. Give contributors something to implement against.

*These three fixes are implemented in the accompanying commit.*

---

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the Unlicense.*
