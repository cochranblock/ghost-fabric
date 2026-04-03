<!-- Unlicense — cochranblock.org -->

# Backlog

*Prioritized work stack. Most important at top. Max 20 items.*
*Self-reorganizes on review — stale items sink, urgent items rise.*

*Last review: 2026-04-03 (P23 Triple Lens)*

---

1. [security] Packet authentication — HMAC-SHA256 on every T12 frame. Shared key derived from configurable secret via HKDF. Nodes reject on MAC mismatch. Blocks src spoofing, peer table poisoning, and ping amplification. Add `hmac` + `sha2` + `hkdf` deps. Sign in f18, verify in f19.
2. [security] Duplicate suppression + peer table cap — Seen-set of (src, seq) HashSet in T9 drops relayed duplicates, kills broadcast cascade. Cap PeerTable at 64 entries with LRU eviction on overflow, prevents OOM from flooded Sync frames. One PR: same attack surface.
3. [test] Extract f22 to lib.rs + full handler tests — Move frame dispatch (Beacon/Ping/Pong/Data/Ack/Sync handlers, relay decision) from main.rs into a testable `pub fn f22(frame, my_id, peers, radio, seq)` in lib.rs. Add one test per T13 variant: peer added, pong sent, table merged, self-src dropped, TTL relay decremented. Currently zero coverage on the hottest path.
4. [fix] Android app shows stubs — wire T9/T11 status into f10 so Android UI shows live peer count and model status instead of static strings.
5. [feature] Logging — structured logs to stderr. `--verbose` flag. Needed before real debugging.
6. [feature] Config CLI — `ghost-fabric config set frequency 868` / `config get` / `config list`. Edit node.json without hand-editing.
7. [test] Multi-node mesh test — spawn 4 ghost-fabric processes with UDS radio, verify peer discovery + route scoring + packet relay end-to-end.
8. [research] Routing algorithm variants — use kova MoE to generate 3 competing mesh routing strategies. **Dep:** kova cluster tunnels.
9. [build] Cross-compile for ARM Linux — `aarch64-unknown-linux-gnu` target. Deploy to IRONHIVE nodes via kova C2. **Dep:** kova c2 sync.
10. [build] Deploy to IRONHIVE — sync ghost-fabric to n0/n1/n2/n3, build on workers, run 4-node mesh over UDS. **Dep:** kova C2.
11. [feature] Sensor trait impl — BME280 temperature/humidity/pressure over I2C. First real T4 driver. Needs `linux-embedded-hal` dep.
12. [feature] Inference trait impl — load quantized ONNX or safetensors model via Candle. First real T6 driver. **Dep:** kova model orchestration.
13. [feature] Gateway node — optional HTTP endpoint exposing mesh status as JSON. Register with approuter. **Dep:** approuter registry.
14. [docs] Hardware requirements doc — which LoRa modules (SX1262, RFM95W), which SBCs (Pi, BeagleBone), wiring diagrams, antenna specs.
15. [build] Signed releases — GitHub Releases with SHA256 checksums. Needed for SSDF PO.2/PO.3 compliance.
16. [docs] Update cochranblock.org products page — remove "Coming Soon — waiting on kova" once mesh packet types ship. **Dep:** cochranblock repo.
17. [research] sx127x/sx126x driver survey — evaluate `lora-rs`, `sx127x`, `embedded-lora` crates for T1 hardware impl. Check unsafe counts, maintenance status.

---

### Completed (this session)

- ~~Mesh packet types~~ — T12/T13/T14, CBOR via ciborium, 9 tests
- ~~Main loop integration~~ — MockRadio + PeerTable wired, beacon/poll/evict
- ~~Integration tests~~ — 8 tests, full pipeline coverage
- ~~UDS radio driver~~ — T15, /tmp/gf-*.sock, 6 tests
- ~~Mesh state sync~~ — T13::Sync/T14::Sync/T16, f23/f24/f25, periodic broadcast every 30s, 10 new tests

### Cross-Project Dependencies

| Item | Depends On | Why |
|------|-----------|-----|
| #8, #9, #10 | **kova** C2 | Deploy binaries to IRONHIVE worker nodes |
| #12 | **kova** model orchestration | MoE routing for edge inference |
| #13 | **approuter** | Register gateway hostname for HTTP dashboard |
| #16 | **cochranblock** | Update product card on cochranblock.org |

### Tags

- `[build]` — compilation, deployment, release infrastructure
- `[test]` — test coverage, CI, quality gates
- `[docs]` — documentation, hardware guides, compliance
- `[feature]` — new functionality
- `[fix]` — bug fixes, corrections
- `[research]` — investigation, evaluation, architecture decisions
