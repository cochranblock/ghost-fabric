<!-- Unlicense — cochranblock.org -->

# Timeline of Invention

*Dated, commit-level record of what was built, when, and why. Proves human-piloted AI development — not generated spaghetti.*

> Every entry below maps to real commits. Run `git log --oneline` to verify.

---

## Human Revelations — Invented Techniques

*Novel ideas that came from human insight, not AI suggestion. These are original contributions to the field.*

### RSSI/Battery/Hop Route Scoring for LoRa Mesh (April 2026)

**Invention:** A mesh routing algorithm that scores routes using a weighted combination of signal strength (RSSI), battery level, hop count, and freshness — choosing routes through nodes that are strong-signal, well-powered, close, and recently seen, rather than just shortest-path.

**The Problem:** Standard mesh routing (AODV, DSR) uses hop count as the primary metric. In LoRa networks operating at 915MHz, this fails: a 2-hop route through a node with -120dBm signal and 5% battery is worse than a 3-hop route through strong, charged nodes. Shortest path isn't best path when nodes are solar-powered sensors in a field.

**The Insight:** Military radio networks don't route through the closest relay — they route through the strongest, most reliable relay. A forward observer with a weak radio and a dying battery is a worse relay than a base station 2 hops away. Signal quality, power state, and recency all matter more than hop count for edge networks.

**The Technique:**
1. `PeerTable` (T9): maintains peer list with RSSI, battery, last_seen per peer
2. Route scoring: weighted formula combining RSSI (signal quality), battery (power remaining), hop count (distance), and freshness (time since last contact)
3. Stale peer eviction: peers not seen within threshold are removed from routing table
4. Each routing decision picks the peer with the best composite score, not the fewest hops
5. 11 unit tests validate scoring edge cases

**Result:** Routes prefer strong-signal, well-powered, recently-seen nodes. Dying or unreachable nodes are automatically evicted. The mesh self-heals as nodes come and go.

**Named:** Composite Route Scoring
**Commit:** `673b202` (P23 Phase 1 implementation)
**Origin:** Military radio relay operations — choosing which hilltop to relay through based on signal strength, power, and reliability, not just distance. Michael Cochran's Army signals experience applied to sub-GHz IoT mesh networking.

### 2026-04-08 — Human Revelations Documentation Pass

**What:** Documented novel human-invented techniques across the full CochranBlock portfolio. Added Human Revelations section with Composite Route Scoring.
**Commit:** See git log
**AI Role:** AI formatted and wrote the sections. Human identified which techniques were genuinely novel, provided the origin stories, and directed the documentation pass.

---

## Entries

### 2026-03-26 — Ghost Fabric Whitepaper + Rust Scaffold

**What:** Published whitepaper on sovereign edge intelligence over sub-GHz cognitive mesh networks. Created Rust binary scaffold. Covers: 915MHz physics constraints, Python ecosystem liability, 19MB Rust architecture, cognitive mesh overlay, and applications (agriculture, disaster response, perimeter security, industrial IoT, sovereign infrastructure).
**Why:** Defines the technical thesis for edge AI on LoRa — needed for SBIR proposals (DoD/DHS cyber and IoT topics) and as public proof of the approach.
**Commit:** `f502788`, `24e2817`
**AI Role:** AI drafted whitepaper sections and added technical specifics (LoRa throughput, L3 cache sizing, dependency counts). Human directed the thesis, validated all claims against hardware specs, and corrected cache-resident execution claims for accuracy.
**Proof:** [WHITEPAPER.md](WHITEPAPER.md)

### 2026-03-26 — License + Proof of Artifacts + Timeline

**What:** Added Unlicense, created PROOF_OF_ARTIFACTS.md and TIMELINE_OF_INVENTION.md.
**Commit:** `6c6bd47`, `bc93ff6`

### 2026-03-27 — QA Round 1: Doc Fix + Cargo.lock

**What:** P12 slop eradication ("utilizing" → "using" in whitepaper). Added README with cochranblock.org backlink. Committed Cargo.lock for binary crate reproducibility.
**Commit:** `940b69b`, `26dd389`, `5dd39b7`
**QA Result:** PASS — `cargo build --release` zero errors, `cargo clippy --release -- -D warnings` clean, zero warnings.

### 2026-03-27 — P13 Tokenization + Binary Size Optimization

**What:** Applied Kova P13 compression mapping. Created `docs/compression_map.md`. Renamed entry point to `f0`. Added release profile: `opt-level='z'`, LTO, single codegen unit, `panic='abort'`, strip. Binary: 285,936 bytes (279KB) pre-deps.
**Commit:** `ce0a27c`
**QA Result (Round 2):** PASS — `cargo clean && cargo build --release` zero errors, clippy clean, `cargo test` passes (0 tests).

### 2026-03-29 — User Story Analysis + Top 3 Fixes

**What:** Full end-to-end user story walkthrough. Brutal honest assessment: score 2/10. Implemented top 3 fixes:
1. CLI with clap: `init`, `start`, `status` subcommands, `--help`, `--version`
2. Node identity: generates `gf-{hex}` ID, persists config to `~/.config/ghost-fabric/node.json`
3. Module scaffold: `radio.rs`, `mesh.rs`, `inference.rs`, `sensor.rs`, `config.rs`

Updated P13 compression map: f0-f9, T0, s0-s3, c0-c2.
**Commit:** `0623d4d`
**Binary size:** 469,792 bytes (459KB) with 5 deps (clap, dirs, rand, serde, serde_json).
**Proof:** [USER_STORY_ANALYSIS.md](USER_STORY_ANALYSIS.md)

### 2026-03-29 — Federal Compliance Govdocs

**What:** Created 11 federal compliance documents in `govdocs/`:
- SBOM.md — Software Bill of Materials (EO 14028)
- SSDF.md — NIST SP 800-218 compliance mapping
- SUPPLY_CHAIN.md — dependency provenance and build reproducibility
- SECURITY.md — security posture, attack surface analysis
- ACCESSIBILITY.md — Section 508 / CLI accessibility
- PRIVACY.md — privacy impact assessment (no PII collected)
- FIPS.md — FIPS 140-2/3 status and path to compliance
- FedRAMP_NOTES.md — deployment model (on-prem, not cloud)
- CMMC.md — CMMC Level 1-2 practice mapping
- ITAR_EAR.md — export control classification (EAR99)
- FEDERAL_USE_CASES.md — agency-specific use cases (DoD, DHS, USDA, DOE, NASA, GSA, VA)

Updated README with quick start and govdocs reference.
**Commit:** `f9f5342`
**AI Role:** AI drafted all govdocs from source code inspection. Human directed which frameworks to address and validated federal program references.

### 2026-03-29 — TOI + POA Update

**What:** Updated TIMELINE_OF_INVENTION.md and PROOF_OF_ARTIFACTS.md with all session commits, QA results, binary sizes, P13 stats.
**Commit:** `6bb7edc`

### 2026-03-29 — Android AAB Build

**What:** Restructured project into Cargo workspace: `ghost_fabric_core` library + CLI binary + Android `cdylib`. Built Android AAB using cargo-ndk (arm64-v8a) + Gradle `bundleRelease`. NativeActivity + egui app, auto-initializes node on first launch, displays node status with refresh button. Signed with upload keystore. Target API 35, min SDK 28.
**Commit:** `be652be`
**Binary size:** AAB 1,643,180 bytes (1.6MB), .so 3,062,432 bytes (3MB).
**AI Role:** AI replicated pixel-forge's Android build pattern. Human directed the target and approved the approach.

### 2026-03-30 — Truth Audit

**What:** Adversarial fact-check of all project documentation. Found 13 discrepancies: present-tense claims for non-existent features (README, WHITEPAPER), stale metrics (POA), undisclosed unsafe block (SECURITY.md). Fixed all — changed whitepaper to design-intent language, updated all metrics, added current-state architecture diagram.
**Commit:** `3aaf362`
**AI Role:** AI performed adversarial audit, verified every claim against code and build output.

### 2026-03-30 — Supply Chain Security Audit + Hot Reload + File Cleanup

**What:** Federal-grade supply chain verification: `cargo audit` (0 CVEs), `cargo outdated` (all current), `cargo tree --duplicates` (0), deep code review of all 7 deps (unsafe counts, process spawning, network calls, env var reads). Added hot reload lifecycle module (f13-f16): PID lockfile, SIGTERM old instance, 5s grace period, SIGKILL fallback. File cleanup: .gitignore hardened. Written to govdocs/SUPPLY_CHAIN_AUDIT.md.
**Commit:** `b4d35e6`
**AI Role:** AI performed deep code review of dependency source code in `~/.cargo/registry/src/`. Human directed the audit scope.

### 2026-03-30 — Polish Pass

**What:** Synced all documentation with current state. README: added platforms table, features list. TOI: added 4 missing entries. POA: updated LOC (328), files (8), functions (16), deps (6), binary size (470,080). SBOM: added libc as direct dep.
**Commit:** `04e115e`

### 2026-04-02 — P23 Triple Lens Audit + Phase 1 Implementation

**What:** Full P23 audit (guest analysis as pessimist lens, IRONHIVE swarm recon as optimist lens, supply chain + unsafe review as paranoia lens), then synthesized into prioritized action plan. Executed Phase 1 immediately:

**P23 Findings:**
- Pessimist: score 3/10, all core features stubbed, 0 tests, misleading `start` command
- Optimist: IRONHIVE swarm online (4/4 nodes), Factory/MoE/Academy available for code gen
- Paranoia: 4 unsafe blocks undisclosed, config accepts invalid LoRa params, no graceful shutdown
- Synthesis: trait foundations + tests + SIGINT first, then use IRONHIVE for mesh protocol gen

**Phase 1 Implementation:**
1. **Radio:** T1 (RadioDriver) trait — init/send/recv/status. T8 (MockRadio) with TX/RX buffers. 7 tests.
2. **Mesh:** T2 (MeshNetwork) trait — add_peer/remove_peer/route/peers. T3 (Peer) struct. T9 (PeerTable) with RSSI+battery+freshness route scoring, stale peer eviction. 11 tests.
3. **Inference:** T6 (InferenceEngine) trait — load_model/predict. T11 (MockEngine). 6 tests.
4. **Sensor:** T4 (SensorDriver) trait — read/status/name. T10 (MockSensor). 4 tests.
5. **Config:** added peers field (backward compat), f17 LoRa spec validation (SF 6-12, valid BW, freq 150-960MHz). 7 tests.
6. **Main loop:** `start` command now runs a main loop with SIGINT handler for clean Ctrl+C shutdown.

Updated P13 compression map: f0-f17, T0-T11, s0-s4, c0-c2. LOC: 328 → 1,101.
**Commit:** `673b202`
**Method:** P23 — three lenses dispatched in parallel (guest analysis, IRONHIVE recon, security review), synthesized into prioritized plan, then executed.
**QA Result:** `cargo build` PASS, `cargo clippy -- -D warnings` zero warnings, `cargo test` 35/35 pass.
**AI Role:** AI performed P23 audit and implemented all traits, mocks, and tests. Human directed the audit scope and approved the plan.

### 2026-04-09 — Packet Authentication: HMAC-SHA256 on Every T12 Frame

**What:** Top backlog item shipped — every mesh frame now carries a 16-byte truncated HMAC-SHA256 tag derived from a shared `network_secret` via HKDF-SHA256. Frames with a wrong key, tampered CBOR, or tampered tag are rejected at `f19` before CBOR decode. Closes the spoofing, peer-table-poisoning, and ping-amplification attack surface against unauthenticated mesh broadcasts.

**Why:** Pre-auth, any RF attacker on the 915MHz band could inject a beacon claiming any node ID, poison every neighbor's PeerTable, or amplify pings into broadcast storms. T12 was a wide-open trust boundary. Federal deployments (CMMC, SSDF PO.4) require authenticated message origin on tactical networks.

**Implementation:**
1. **Deps:** `hmac` 0.12, `sha2` 0.10, `hkdf` 0.12 — pure-Rust, RustCrypto family, no `unsafe`
2. **`f26` (HKDF):** derives a 32-byte mesh key from `network_secret` bytes using `HKDF-SHA256` with `info = b"ghost-fabric mesh v1"`
3. **`f18` (encode):** CBOR-encodes the frame, computes HMAC-SHA256 over the CBOR bytes, appends the first 16 bytes (truncated tag) — wire format: `[CBOR][16-byte MAC]`
4. **`f19` (decode):** splits off the trailing 16 bytes, recomputes HMAC over the CBOR region, verifies via `verify_truncated_left` (constant-time), then CBOR-decodes only on MAC success
5. **`T0::network_secret`:** new config field, `#[serde(default)]` for backward compatibility with existing `node.json` files (empty string = open mesh, still MAC-protected from random radio corruption)
6. **Main loop:** derives the key once at startup, threads it through every `f18` send and `f19` recv (beacon, poll, sync, relay)
7. **13 new tests:** wrong-key rejection, CBOR tamper rejection, MAC tamper rejection, truncation rejection, garbage rejection, empty input, HKDF determinism, end-to-end attacker injection, config round-trip + back-compat
8. **Frame budget:** MAX_CBOR_BYTES = 251 - 16 = 235 — leaves 16 bytes of headroom inside the LoRa 255-byte limit for the MAC

**Bugs found and fixed during wire-up:**
- `T0::default()` was missing the new `network_secret` field — wouldn't compile
- `f19` initially used `mac.verify_slice()`, which requires the *full* 32-byte HMAC; we append only 16 bytes, so every round-trip silently failed. Fixed to `verify_truncated_left()`. Caught because the round-trip tests panicked with "MAC verification failed" while the negative tests (which expect failure) all passed — a textbook case of asymmetric test coverage exposing the bug.

**Commit:** `e5e7de8`
**QA Result:** `cargo check` PASS, `cargo test` 118/118 pass (103 unit + 15 integration).
**AI Role:** AI implemented the HMAC wire format, wired the key through main loop and UDS radio, wrote all 13 auth tests, diagnosed the `verify_slice` vs `verify_truncated_left` bug from the asymmetric failure pattern. Human directed the priority (top backlog item) and approved the design (16-byte truncated tag, HKDF-derived key).

---

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the Unlicense.*
