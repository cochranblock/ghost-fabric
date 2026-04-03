<!-- Unlicense — cochranblock.org -->

# Backlog

*Prioritized work stack. Most important at top. Max 20 items.*
*Self-reorganizes on review — stale items sink, urgent items rise.*

*Last review: 2026-04-03*

---

1. ~~[feature] Mesh packet types~~ — DONE. T12 Frame, T13 FrameKind, T14 Payload. CBOR via ciborium. Beacon/Data/Ack/Ping/Pong. 9 tests.
2. [feature] Main loop integration — wire PeerTable (T9) + RadioDriver (T1) into `start` command. Poll radio, update peers, score routes.
3. [test] Integration tests — end-to-end: init → start → inject mock packets → verify peer table updates → shutdown. Currently 0 integration tests.
4. [feature] Unix domain socket radio driver — implements T1 over UDS for local multi-process mesh testing without hardware.
5. [research] Routing algorithm variants — use kova MoE to generate 3 competing mesh routing strategies, score against packet loss / latency / battery drain.
6. [feature] Mesh state sync — compressed neighbor table broadcast on reconnection. Peers share who they know.
7. [feature] Packet authentication — HMAC-SHA256 on mesh frames. Nodes reject unsigned packets. Needs `ring` or `RustCrypto` dep.
8. [build] Cross-compile for ARM Linux — `aarch64-unknown-linux-gnu` target. Deploy to IRONHIVE nodes via kova C2. **Dep:** kova c2 sync.
9. [feature] Sensor trait impl — BME280 temperature/humidity/pressure over I2C. First real T4 driver. Needs `linux-embedded-hal` dep.
10. [feature] Inference trait impl — load quantized ONNX or safetensors model via Candle. First real T6 driver. **Dep:** kova model orchestration.
11. [build] Deploy to IRONHIVE — sync ghost-fabric to n0/n1/n2/n3 via `kova c2 sync`, build on workers, run 4-node mesh over UDS. **Dep:** kova C2.
12. [test] Multi-node mesh test — 4 ghost-fabric processes on IRONHIVE nodes, UDS radio, verify peer discovery + route scoring + packet relay.
13. [feature] Config CLI — `ghost-fabric config set frequency 868` / `config get` / `config list`. Edit node.json without hand-editing.
14. [feature] Logging — structured JSON logs to stderr. `--verbose` flag. Needed before any real debugging.
15. [docs] Hardware requirements doc — which LoRa modules (SX1262, RFM95W), which SBCs (Pi, BeagleBone), wiring diagrams, antenna specs.
16. [feature] Gateway node — optional HTTP endpoint exposing mesh status as JSON. Register with approuter for web dashboard. **Dep:** approuter registry.
17. [fix] Android app shows stubs — wire T9/T11 status into f10 so Android UI shows live peer count and model status instead of static strings.
18. [build] Signed releases — `cargo-dist` or manual, publish to GitHub Releases with SHA256 checksums. Needed for SSDF PO.2/PO.3 compliance.
19. [docs] Update cochranblock.org products page — remove "Coming Soon — waiting on kova" once mesh packet types ship. **Dep:** cochranblock repo.
20. [research] sx127x/sx126x driver survey — evaluate `lora-rs`, `sx127x`, `embedded-lora` crates for T1 hardware impl. Check unsafe counts, maintenance status.

---

### Cross-Project Dependencies

| Item | Depends On | Why |
|------|-----------|-----|
| #8, #11, #12 | **kova** C2 | Deploy binaries to IRONHIVE worker nodes via SSH/rsync |
| #1, #5 | **kova** Factory/MoE | Generate and evaluate code variants on IRONHIVE cluster |
| #10 | **kova** model orchestration | MoE routing decides which node runs which model |
| #16 | **approuter** | Register gateway node hostname for HTTP dashboard |
| #19 | **cochranblock** | Update product card on cochranblock.org |

### Tags

- `[build]` — compilation, deployment, release infrastructure
- `[test]` — test coverage, CI, quality gates
- `[docs]` — documentation, hardware guides, compliance
- `[feature]` — new functionality
- `[fix]` — bug fixes, corrections
- `[research]` — investigation, evaluation, architecture decisions
