<!-- Unlicense — cochranblock.org -->

# NIST SP 800-218 — Secure Software Development Framework

*Mapping ghost-fabric practices to SSDF tasks.*

## PS — Prepare the Organization

| Practice | Status | Evidence |
|----------|--------|----------|
| PS.1 — Define security requirements | Partial | WHITEPAPER.md defines edge sovereignty, zero cloud deps, minimal attack surface |
| PS.2 — Implement roles and responsibilities | In place | Human-directed, AI-assisted development (see TIMELINE_OF_INVENTION.md) |
| PS.3 — Implement supporting toolchains | In place | Rust compiler (memory-safe), clippy (lint), cargo (build/dep management) |

## PW — Protect the Software

| Practice | Status | Evidence |
|----------|--------|----------|
| PW.1 — Design software to meet security requirements | In place | Single static binary, no interpreter, no dynamic linking, no network listeners in current build |
| PW.4 — Reuse existing well-secured components | In place | All deps are high-trust crates.io packages (clap, serde, rand) — see SBOM.md |
| PW.5 — Create source code following secure practices | In place | Rust's ownership model prevents use-after-free, buffer overflows, data races at compile time |
| PW.6 — Configure compilation/build to improve security | In place | Release profile: LTO, single codegen unit, panic=abort (no unwinding attack surface), stripped symbols |
| PW.7 — Review and verify third-party components | Partial | Cargo.lock pins versions; manual audit not yet performed |
| PW.9 — Test executable code | Partial | Build succeeds, clippy passes with -D warnings; no unit/integration tests yet |

## RV — Respond to Vulnerabilities

| Practice | Status | Evidence |
|----------|--------|----------|
| RV.1 — Identify and confirm vulnerabilities | Planned | `cargo audit` not yet integrated; zero CVEs in current dep set |
| RV.2 — Assess and prioritize vulnerabilities | Planned | No vulnerability tracking process yet |
| RV.3 — Remediate vulnerabilities | In place | Cargo.lock pinning + `cargo update` workflow available |

## PO — Protect Operations

| Practice | Status | Evidence |
|----------|--------|----------|
| PO.1 — Protect all forms of code from unauthorized access | In place | Source on GitHub, Unlicense (public), no secrets in source |
| PO.2 — Provide a mechanism for verifying software integrity | Partial | Git commit hashes; no signed releases yet |
| PO.3 — Archive and protect each software release | Planned | No release artifacts yet |

## Summary

Ghost-fabric is early-stage. The Rust language provides strong baseline security (memory safety, type safety, no undefined behavior). The build pipeline enforces zero warnings. Supply chain is minimal (5 direct deps, all permissive). Primary gaps: no `cargo audit`, no signed releases, no test suite.
