<!-- Unlicense — cochranblock.org -->

# Supply Chain Security Audit

*Federal-grade supply chain verification per EO 14028. Performed 2026-03-30.*

## Tool Results

### 1. cargo audit — Known Vulnerabilities

```
Scanning Cargo.lock for vulnerabilities (314 crate dependencies)
```

**Result: PASS — 0 advisories found.**

No known CVEs in any direct or transitive dependency.

### 2. cargo tree --duplicates — Duplicate Dependencies

**Result: PASS — 0 duplicates.**

No duplicate crate versions in the ghost-fabric CLI dependency tree.

### 3. cargo outdated — Version Currency

**Result: PASS — all dependencies up to date.**

Every direct dependency is at the latest compatible version as of audit date.

### 4. Cargo.lock — Committed

**Result: PASS.** `Cargo.lock` is committed to version control, ensuring reproducible builds.

### 5. Yanked Crates

**Result: PASS.** No yanked crate versions in `Cargo.lock`.

### 6. Typosquatting Check

| Dep Name | Similar Popular Crate | Typosquat Risk |
|----------|-----------------------|----------------|
| clap | — | None (is the popular crate) |
| dirs | — | None (is the popular crate) |
| rand | — | None (is the popular crate) |
| serde | — | None (is the popular crate) |
| serde_json | — | None (is the popular crate) |
| zmij | ryu (prior float formatter) | **Low** — different name, same author (dtolnay), serde_json's chosen replacement for ryu |
| libc | — | None (is the popular crate) |

**Result: PASS.** All dependencies are well-known, high-download crates. No suspicious names.

## Deep Code Review

### Methodology

Reviewed source code at `~/.cargo/registry/src/` for each direct dependency. Checked for: unsafe blocks, process spawning, network calls, unexpected env var reads, unchecked unwraps on user input, and secret exfiltration vectors.

### Per-Dependency Analysis

#### serde-1.0.228

| Check | Finding |
|-------|---------|
| Unsafe blocks | 2 — `str::from_utf8_unchecked` on library-constructed buffers (known-valid UTF-8) |
| Shells out | build.rs only (`rustc --version` for feature detection) |
| Network calls | None |
| Env var reads | build.rs only (standard cargo vars) |
| Author | dtolnay — foundational Rust ecosystem maintainer |
| **Verdict** | **Clean** |

#### serde_json-1.0.149

| Check | Finding |
|-------|---------|
| Unsafe blocks | 12 — all on internally-validated data (transmute on repr(transparent) newtypes, from_utf8_unchecked after validation) |
| Stack overflow protection | Yes — recursion limit of 128 enforced by default (`remaining_depth: u8`) |
| Shells out | No |
| Network calls | No |
| Env var reads | No |
| Unwrap on user input | No — all `.unwrap()` in doc comments/examples only |
| Author | dtolnay |
| **Verdict** | **Clean** |

#### clap-4.6.0 / clap_builder-4.6.0

| Check | Finding |
|-------|---------|
| Unsafe blocks | 0 — `#![forbid(unsafe_code)]` declared |
| Shells out | No (`Command::new` refs are clap's own `clap::Command` type, not `std::process::Command`) |
| Network calls | No |
| Env var reads | `COLUMNS`/`LINES` for terminal width (standard), opt-in per-arg env vars |
| **Verdict** | **Clean** |

#### rand-0.8.5 / rand_chacha-0.3.1 / rand_core-0.6.4 / getrandom-0.2.17

| Check | Finding |
|-------|---------|
| Unsafe blocks | ~20 total across all rand crates |
| CSPRNG seeding | Properly seeded from OS entropy: macOS uses `CCRandomGenerateBytes`, Linux uses `getrandom(2)` syscall |
| RNG chain | OS entropy → ChaCha12 → auto-reseed every 64KB |
| Fork safety | `pthread_atfork` registered for reseed on fork |
| Shells out | No |
| Network calls | No |
| Env var reads | No |
| **Verdict** | **Clean** |

#### dirs-6.0.0

| Check | Finding |
|-------|---------|
| Unsafe blocks | 0 |
| Shells out | No |
| Network calls | No |
| Env var reads | XDG vars on Linux only (standard spec). On macOS: zero env var reads |
| **Verdict** | **Clean** |

#### zmij-1.0.21

| Check | Finding |
|-------|---------|
| Unsafe blocks | 75 — high count, but expected for a `#![no_std]` numeric formatting algorithm |
| Purpose | f64-to-decimal-string conversion (Schubfach algorithm). Replacement for ryu |
| Safety discipline | `#![deny(unsafe_op_in_unsafe_fn)]` — stricter than default |
| Dependencies | Zero runtime deps |
| Author | dtolnay |
| Shells out | No |
| Network calls | No |
| Env var reads | No |
| **Verdict** | **Acceptable risk** — pure computation, trusted author |

#### libc-0.2.183

| Check | Finding |
|-------|---------|
| Purpose | Raw FFI bindings to platform libc. Used for signal handling (SIGTERM/SIGKILL) in lifecycle module |
| Unsafe blocks | Entire crate is FFI definitions — inherently unsafe but standard |
| Author | Rust project |
| **Verdict** | **Clean** — standard platform binding |

### Unsafe Usage Summary (cargo-geiger scope)

| Crate | Unsafe Blocks | Concern Level |
|-------|--------------|---------------|
| zmij | 75 | Low (numeric algorithm, dtolnay) |
| rand (all) | ~20 | Low (CSPRNG, documented invariants) |
| serde_json | 12 | Low (validated data only) |
| serde | 2 | Low |
| ghost-fabric core | 0 | None |
| clap | 0 (forbid) | None |
| dirs | 0 | None |

No dependency exceeds 75 unsafe blocks. The highest (zmij) is a `no_std` numeric algorithm by dtolnay.

### Process Spawning Check

No dependency executes `std::process::Command` at runtime. Build scripts (`build.rs`) call `rustc --version` for feature detection only.

### Network Call Check

No dependency makes network calls. No telemetry, analytics, or phone-home behavior detected.

### Secret Exfiltration Check

No dependency reads unexpected files or environment variables. All env var reads are standard (`COLUMNS`, `LINES`, XDG spec on Linux).

## Recommended Actions

1. Integrate `cargo audit` into CI/pre-push hook
2. Run `cargo deny` with a config file for automated license checking
3. Consider `cargo-vet` for ongoing third-party review tracking
4. Pin minimum dep versions in `Cargo.toml` to prevent regression

## Overall Assessment

**PASS.** Zero vulnerabilities, zero yanked crates, zero duplicates, all deps current. Supply chain consists entirely of high-trust, well-maintained crates authored by known Rust ecosystem maintainers. No runtime process spawning, network calls, or unexpected env var reads. No secret exfiltration vectors.

---

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the Unlicense.*
