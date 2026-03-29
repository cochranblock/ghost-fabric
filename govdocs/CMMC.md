<!-- Unlicense — cochranblock.org -->

# CMMC Compliance Mapping

*Cybersecurity Maturity Model Certification — Level 1 and Level 2 practices supported by ghost-fabric.*

## Level 1 — Basic Cyber Hygiene

### AC — Access Control

| Practice | Status | Evidence |
|----------|--------|---------|
| AC.L1-3.1.1 — Limit system access to authorized users | Partial | Binary runs as local user process; no multi-user access. Future: node identity keys for mesh authentication |
| AC.L1-3.1.2 — Limit system access to authorized transactions | In place | Binary performs only local operations (config read/write). No network listeners |
| AC.L1-3.1.20 — Control external connections | In place | Zero network connections in current build. Future LoRa operates on dedicated 915MHz band, not IP |

### IA — Identification and Authentication

| Practice | Status | Evidence |
|----------|--------|---------|
| IA.L1-3.5.1 — Identify system users | Partial | Node ID generated per-node. No user authentication (single-user edge device) |
| IA.L1-3.5.2 — Authenticate users | N/A | Single-user edge device. No login required |

### SC — System and Communications Protection

| Practice | Status | Evidence |
|----------|--------|---------|
| SC.L1-3.13.1 — Monitor/control communications at boundaries | In place | No communications in current build. Future: LoRa packets authenticated per-node |
| SC.L1-3.13.5 — Implement subnetworks for CUI | In place by design | Each node is physically isolated. Mesh uses air-gapped LoRa, not IP networking |

## Level 2 — Advanced Cyber Hygiene

### AU — Audit and Accountability

| Practice | Status | Evidence |
|----------|--------|---------|
| AU.L2-3.3.1 — Create audit logs | Planned | No logging yet. Future: local audit log of node decisions and mesh events |
| AU.L2-3.3.2 — Trace actions to individual users | N/A | Single-user edge device |

### CM — Configuration Management

| Practice | Status | Evidence |
|----------|--------|---------|
| CM.L2-3.4.1 — Establish configuration baselines | In place | Cargo.lock pins all dependency versions. Release profile is deterministic |
| CM.L2-3.4.2 — Enforce security configuration settings | In place | `panic='abort'`, LTO, stripped symbols, zero network listeners |
| CM.L2-3.4.6 — Employ least functionality | In place | Binary contains only what's compiled in. No plugins, no dynamic loading, no interpreter |

### SA — Security Assessment

| Practice | Status | Evidence |
|----------|--------|---------|
| SA.L2-3.12.1 — Assess security controls | In place | clippy -D warnings, cargo build with zero warnings, QA rounds documented |
| SA.L2-3.12.4 — Develop action plans | In place | USER_STORY_ANALYSIS.md documents gaps and fixes |

### SI — System and Information Integrity

| Practice | Status | Evidence |
|----------|--------|---------|
| SI.L2-3.14.1 — Identify and correct flaws | In place | Rust compiler catches memory bugs at compile time. clippy catches logic issues |
| SI.L2-3.14.2 — Provide protection from malicious code | In place | No dynamic code execution. Single static binary. No interpreter, no eval |
| SI.L2-3.14.6 — Monitor system for attacks | Planned | No runtime monitoring yet |

## Summary

Ghost-fabric's design (single binary, no network, no interpreter, no dynamic loading, Rust memory safety) naturally aligns with CMMC principles of least functionality and minimal attack surface. Primary gaps are in audit logging and runtime monitoring — both planned for future mesh implementation.
