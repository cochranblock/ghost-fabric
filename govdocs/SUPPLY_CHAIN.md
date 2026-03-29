<!-- Unlicense — cochranblock.org -->

# Supply Chain Integrity

## Dependency Sources

All Rust dependencies are sourced from **crates.io**, the official Rust package registry. crates.io is operated by the Rust Foundation and enforces:
- Immutable published versions (no post-publish modification)
- Cryptographic checksums for each crate version
- Mandatory crate ownership verification

## Version Pinning

`Cargo.lock` is committed to version control. This ensures:
- Every build uses the exact same dependency versions
- No silent upgrades between builds
- Reproducible builds across machines and CI environments

**Verify:** `git log --oneline Cargo.lock` shows lock file history.

## Build Reproducibility

| Property | Status |
|----------|--------|
| Cargo.lock committed | Yes |
| Single codegen unit | Yes (`codegen-units = 1`) |
| LTO enabled | Yes (`lto = true`) |
| Deterministic build profile | Yes (opt-level, panic, strip all specified) |
| Vendored binaries | None — all code compiled from source |
| Pre-built artifacts | None |
| Build scripts (build.rs) | None in ghost-fabric; some in transitive deps (proc-macros only) |

## No Vendored Binaries

Ghost-fabric contains zero pre-compiled binaries. The entire dependency tree compiles from Rust source code via `cargo build`. The final binary is a single statically linked executable.

## Source Availability

- **Repository:** github.com/cochranblock/ghost-fabric
- **License:** Unlicense (public domain)
- **All source code** is human-readable and auditable

## Verification Commands

```bash
# Verify dependency tree
cargo tree

# Check for known vulnerabilities
cargo audit

# Verify lock file matches manifests
cargo check

# Full clean rebuild
cargo clean && cargo build --release
```

## Supply Chain Risks

| Risk | Mitigation |
|------|-----------|
| Compromised crate on crates.io | Cargo.lock pins exact versions; manual audit possible |
| Typosquatting | Only well-known, high-download crates used (clap, serde, rand) |
| Build-time code execution | Proc-macros (serde_derive, clap_derive) run at compile time — auditable |
| Transitive dependency bloat | 5 direct deps, ~45 transitive — minimal surface |
