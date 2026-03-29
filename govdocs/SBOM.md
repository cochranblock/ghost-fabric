<!-- Unlicense — cochranblock.org -->

# Software Bill of Materials (SBOM)

*Per Executive Order 14028 — Improving the Nation's Cybersecurity*

## Direct Dependencies

| Crate | Version | License | Purpose |
|-------|---------|---------|---------|
| clap | 4.6.0 | MIT OR Apache-2.0 | CLI argument parsing |
| dirs | 6.0.0 | MIT OR Apache-2.0 | Platform config directory resolution |
| rand | 0.8.5 | MIT OR Apache-2.0 | Node ID generation |
| serde | 1.0.228 | MIT OR Apache-2.0 | Config serialization |
| serde_json | 1.0.149 | MIT OR Apache-2.0 | JSON config file format |

**Direct dependencies: 5**
**Total transitive dependencies: ~45**

## Transitive Dependencies (depth 2)

| Crate | Version | License |
|-------|---------|---------|
| clap_builder | 4.6.0 | MIT OR Apache-2.0 |
| clap_derive | 4.6.0 | MIT OR Apache-2.0 |
| dirs-sys | 0.5.0 | MIT OR Apache-2.0 |
| libc | 0.2.183 | MIT OR Apache-2.0 |
| rand_chacha | 0.3.1 | MIT OR Apache-2.0 |
| rand_core | 0.6.4 | MIT OR Apache-2.0 |
| serde_core | 1.0.228 | MIT OR Apache-2.0 |
| serde_derive | 1.0.228 | MIT OR Apache-2.0 |
| itoa | 1.0.18 | MIT OR Apache-2.0 |
| memchr | 2.8.0 | Unlicense OR MIT |
| zmij | 1.0.21 | MIT |

## License Summary

All dependencies are dual-licensed MIT OR Apache-2.0, with two exceptions:
- `memchr`: Unlicense OR MIT (compatible)
- `zmij`: MIT only (compatible)

No GPL, AGPL, or copyleft dependencies. All licenses are permissive and compatible with government use.

## How to Verify

```bash
cargo tree --depth 2 --format "{p} {l}"
```

## Source Registry

All dependencies sourced from **crates.io** (Rust's official package registry). Pinned via `Cargo.lock` in version control.
