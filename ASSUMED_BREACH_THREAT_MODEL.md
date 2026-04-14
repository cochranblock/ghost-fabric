# Assumed Breach Threat Model

> **Operating assumption: every component below is already compromised. Design for damage containment and loud detection, not for prevention.**

This document is the canonical threat model for every project in the `cochranblock/*` portfolio. Each project adapts the Threat Surface section for its own context but shares the same first principles, mitigations, and verification protocol.

---

## First Principles

1. **Every record that matters has an external witness.** Hashes published to public git (or equivalent neutral timestamp authority) so tampering requires simultaneously corrupting your system AND the public chain.
2. **No single point of compromise.** Signing keys in hardware (YubiKey / TPM / Secure Enclave). Never in software. Never in env vars. Never in config files.
3. **Default air-gap.** No network dependency for correctness. Network is for backup + publishing hashes, both signed, both verifiable post-hoc.
4. **Append-only everything.** No delete path in any storage layer. Corrections are reversing entries referencing the original. Standard accounting discipline, enforced in code.
5. **Cryptographic audit chain.** Every day's state derives from the previous day's hash. Tampering with any day invalidates every subsequent day.
6. **Disclosure of methodology is a security feature.** If an auditor can independently verify the algorithm, they can independently verify the outputs. No "trust us" layers.
7. **Separation of duties enforced in software.** Entry, approval, and audit live in different trust zones. Compromise of one does not compromise the others.
8. **Redundancy across trust zones.** Local + different-cloud + different-format + offline. Attacker must compromise all to hide damage.
9. **Test breach scenarios regularly.** Triple Sims applied to tamper detection. If the chain does not detect a simulated tamper, the chain is broken.

---

## Threat Surface (ghost-fabric)

Ghost Fabric is a LoRa 915MHz mesh node framework — Rust CLI + Android app. Subsystem traits (`T1` radio, `T2` mesh, `T4` sensor, `T6` inference) ship with mock implementations (`T8`/`T9`/`T10`/`T11`); hardware drivers are planned. Federal compliance posture lives in `govdocs/` (CMMC, SSDF, FIPS, ITAR/EAR, SUPPLY_CHAIN_AUDIT).

### Records of consequence this project emits

- **T12 mesh frames** — beacon / poll / sync / relay, CBOR-encoded, HMAC-SHA256-authenticated since commit `e5e7de8` (16-byte truncated tag over the CBOR region, per-mesh key derived via HKDF-SHA256 from `network_secret` with `info = b"ghost-fabric mesh v1"`).
- **Peer table state** — RSSI, battery, hop count, `last_seen` per peer; input to composite route scoring (`T9`).
- **Node identity** — `gf-{hex}` ID persisted to `~/.config/ghost-fabric/node.json`.
- **Shared `network_secret`** — the symmetric root of trust for the entire mesh, stored plaintext in `node.json`.
- **Future:** on-node inference outputs and sensor readings once `T1` / `T4` / `T6` gain hardware drivers.

### In-scope threats

| Assume | Specific to ghost-fabric |
|--------|---------------------------|
| **RF injection on 915MHz** | The ISM band is open. Any attacker with a LoRa radio in range can transmit. `f19` MAC verify is the trust boundary — any code path that decodes a T12 frame before verify is a regression. |
| **Shared-secret compromise** | `network_secret` is symmetric and global across the mesh. One extracted key = full mesh impersonation. There is no revocation path today. |
| **Physical node seizure** | Edge nodes are solar-powered and unattended, deployed in hostile terrain (border, disaster response, defense). Attacker recovers `node.json` verbatim from disk. No rate limiting on key-extraction attempts. |
| **Peer-table poisoning by an authenticated peer** | HMAC proves origin, not truth. A compromised-but-legitimate node can publish fraudulent RSSI / battery / hop metrics to steer traffic or blackhole routes. `T9` composite scoring is not Byzantine-tolerant. |
| **Replay / freshness** | T12 frames carry no sequence number or authenticated timestamp. A replayed beacon authenticates. `last_seen` is an unauthenticated local-clock approximation. |
| **Config tampering** | `node.json` has no signature. Local malware or a physical attacker can swap `network_secret` to point a node at a hostile mesh, or rewrite the node ID to collide with a target peer. |
| **Supply chain (deps)** | 10 direct runtime deps (`clap`, `serde`, `serde_json`, `dirs`, `libc`, `rand`, `ciborium`, `hmac`, `sha2`, `hkdf`). `cargo audit` clean; deep review in `govdocs/SUPPLY_CHAIN_AUDIT.md`. Every new dep must clear the same bar. |
| **Mock-vs-hardware drift** | All four subsystems ship with mocks today. Passing tests on `T8` / `T9` / `T10` / `T11` is not evidence of correctness on real radios or sensors. A backdoored hardware driver is invisible to the mock suite. |
| **Clock manipulation** | LoRa nodes have no authoritative time source. Freshness checks rely on the node's own clock. An attacker with physical access can rewind the clock to replay stale peer state. |
| **Binary / hot-reload channel** | The `f13`-`f16` hot-reload path trusts whoever can write a replacement binary to disk. No signature check on the new binary today — a write primitive upgrades to a full node takeover. |
| **Android app install channel** | The `arm64-v8a` AAB is signed with the upload keystore. Sideload-based tampering on the device is the practical threat; Play Integrity or equivalent device-attestation is not wired. |

### Out of scope (N/A for this project)

- **Financial / accounting / ledger records** — none emitted. Ghost Fabric is a mesh framework, not an accounting system.
- **PII** — none collected (`govdocs/PRIVACY.md`). No human-subject data crosses the mesh.
- **Cloud-tethered state** — none by design. Air-gap is the default operating mode, not an exceptional case; First Principle #3 is satisfied structurally.
- **Public-chain hash publishing** — deferred. The project emits no artifact today whose tamper-evidence requires an external witness (mesh frames are ephemeral, config is per-node, peer state is transient). Re-open this section once mission logs, routing decisions, or cross-node audit trails become persistent records.
- **Daily cryptographic audit chain** — deferred for the same reason. Revisit once `T6` inference or `T4` sensor drivers emit persistent decision logs.
- **Hardware signing keys** — not integrated. The symmetric `network_secret` is the only key material today. Per-node asymmetric identity (YubiKey / TPM / Android Keystore / Secure Enclave) is on the backlog for the transition away from a single shared key.
- **DCAA / timecard / billable-hours surface** — N/A.

---

## Mitigations

| Assume | Mitigation | Verification |
|--------|-----------|--------------|
| Binary compromised | Hardware-key signatures for every output of consequence | Anyone can verify the public key matches expected fingerprint |
| Storage compromised | Append-only sled trees. Delete is not a function, not a policy. | Hash chain breaks on any rewrite. External witness detects. |
| Network MITM | Air-gap capable. Network used only for signed backups + hash publishing. | NTP + GitHub timestamp + hardware counter cross-checked. |
| Signing key stolen | Daily hash committed to public git. Stolen key cannot retroactively change committed days. | Any day older than the public commit is immutable in evidence. |
| Audit log tampered | Separate sled tree, write-only from main app. Auditor tool reads both + cross-checks. | Compromise of main app leaves audit log intact. |
| Backup tampered | 3 different targets with 3 different credentials (local USB + off-site cloud + paper). | Attacker needs all three to hide damage. |
| Insider / self-tampering | No admin role. No delete. Reversing entries only. | Legal record immune to author second-thoughts. |
| Clock manipulation | Multiple time sources: local clock, NTP, git commit timestamp, hardware-key counter. | Divergence flags exception requiring supervisor approval. |
| Supply chain (deps) | `cargo audit` in CI. Pinned SBOM. Reproducible builds where possible. | Anyone can reproduce the binary from source + lockfile. |
| Physical device seizure | Full-disk encryption. Hardware key physically separate from device. | Stolen laptop without key is useless for forgery. |

---

## Public-Chain Deployment

This project publishes tamper-evident hashes to a public companion repo: `cochranblock/<project>-chain` (where `<project>` is the project name).

- **Daily cycle:** at 23:59 local, compute BLAKE3 of all records-of-consequence from the day. Sign with hardware key. Commit to chain repo. Push.
- **GitHub timestamp** on the commit = neutral third-party witness. Anyone can cold-verify records were not rewritten after commit time.
- **Verification:** `<project> verify` reads the chain and re-derives hashes. Any divergence = tampering detected.

This pattern is a private Certificate Transparency log for project state. Same primitive Google uses for TLS certs, applied to whatever the project tracks.

---

## Triple Sims for Tamper Detection

Standard Triple Sims gate (run 3x identically) extended with a tamper-scenario sim:

1. Normal run → produce canonical output
2. Simulated tampering (flip one bit in storage) → `verify` must flag it
3. Simulated clock rewind → `verify` must flag it

If any sim fails to detect, the chain is broken. Fix before merge.

---

## Scope of this Document

- Covers: any artifact this project emits that has legal, financial, or audit consequence.
- Does NOT cover: source code itself (public under Unlicense, not sensitive), build outputs (reproducible), marketing content (public by design).
- If your project emits no records of consequence, the relevant sections are zero-length and the public-chain deployment is skipped. Document that explicitly.

---

## Relation to Other Docs

- **TIMELINE_OF_INVENTION.md** — establishes priority dates for contributions. Feeds into the chain's initial state.
- **PROOF_OF_ARTIFACTS.md** — cryptographic signatures on release artifacts. Adjacent pattern, same first principles.
- **DCAA_COMPLIANCE.md** (where applicable) — how this threat model satisfies FAR/DFARS audit requirements.

---

## Status

- [ ] Threat Surface section adapted for this project
- [ ] Hardware-key signing integrated or N/A documented
- [ ] Public-chain repo created and connected or N/A documented
- [ ] Triple Sims tamper-detection test present or N/A documented
- [ ] External verification procedure documented

---

*Unlicensed. Public domain. Fork, strip attribution, adapt, ship.*

*Canonical source: cochranblock.org/threat-model — last revision 2026-04-14*
