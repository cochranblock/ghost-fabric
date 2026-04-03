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
| PW.7 — Review and verify third-party components | In place | Cargo.lock pins versions; deep code review performed (see SUPPLY_CHAIN_AUDIT.md), `cargo audit` 0 CVEs |
| PW.9 — Test executable code | Partial | Build succeeds, clippy -D warnings clean; 35 unit tests (radio 7, mesh 11, inference 6, sensor 4, config 7). Integration tests not yet implemented. |

## RV — Respond to Vulnerabilities

| Practice | Status | Evidence |
|----------|--------|----------|
| RV.1 — Identify and confirm vulnerabilities | In place | `cargo audit` run (0 CVEs), deep code review of all 7 deps (see SUPPLY_CHAIN_AUDIT.md) |
| RV.2 — Assess and prioritize vulnerabilities | Planned | No vulnerability tracking process yet |
| RV.3 — Remediate vulnerabilities | In place | Cargo.lock pinning + `cargo update` workflow available |

## PO — Protect Operations

| Practice | Status | Evidence |
|----------|--------|----------|
| PO.1 — Protect all forms of code from unauthorized access | In place | Source on GitHub, Unlicense (public), no secrets in source |
| PO.2 — Provide a mechanism for verifying software integrity | Partial | Git commit hashes; no signed releases yet |
| PO.3 — Archive and protect each software release | Planned | No release artifacts yet |

## Summary

Ghost-fabric is early-stage. The Rust language provides strong baseline security (memory safety, type safety, no undefined behavior). The build pipeline enforces zero warnings. Supply chain is minimal (6 direct deps, all permissive), audited with `cargo audit` (0 CVEs) and deep code review. 35 unit tests cover all subsystem traits. Primary gaps: no signed releases, no integration tests.
