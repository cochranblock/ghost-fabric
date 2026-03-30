<!-- Unlicense — cochranblock.org -->

# Security Posture

## Current State

Ghost-fabric is an early-stage scaffold. The security surface is minimal because functionality is minimal.

## Cryptography

| Primitive | Status | Usage |
|-----------|--------|-------|
| AES-256-GCM | Planned | Future: mesh packet encryption |
| Argon2id | Planned | Future: node key derivation |
| HKDF | Planned | Future: session key derivation |
| CSPRNG (rand) | In use | Node ID generation via `rand::thread_rng()` (ChaCha20-based) |

**Current crypto in code:** Only CSPRNG for node ID generation. No encryption, signing, or key management implemented yet.

## No Plaintext Secrets

Verified by inspection:
- No API keys, tokens, or passwords in source
- No `.env` files
- `.gitignore` excludes `target/` (build artifacts)
- Node config (`node.json`) contains only radio parameters and a random node ID — no secrets

## Input Validation

| Input | Validation | Status |
|-------|-----------|--------|
| CLI arguments | clap (typed, validated) | In place |
| Config file (JSON) | serde (typed deserialization) | In place |
| Radio packets | Not implemented | N/A |
| Sensor data | Not implemented | N/A |

## Error Handling

- CLI errors: clap provides structured error messages with usage hints
- Config errors: `Option<T>` return with user-facing error messages and exit code 1
- No `unwrap()` on user-facing paths (config load returns `Option`)
- `panic = 'abort'` in release — no stack unwinding, immediate termination on panic

## Attack Surface

| Surface | Current Exposure | Notes |
|---------|-----------------|-------|
| Network listeners | None | No ports open, no HTTP, no sockets |
| File system | Config read/write only | Single JSON file in platform config dir |
| Radio (LoRa) | Not implemented | Future: will need packet authentication |
| External processes | None | No shell calls, no subprocess spawning |
| User input | CLI args only | Parsed by clap, type-safe |

## Memory Safety

Rust provides compile-time guarantees against:
- Buffer overflows
- Use-after-free
- Double-free
- Data races
- Null pointer dereference (no null — uses `Option<T>`)

No `unsafe` blocks in ghost-fabric core source code. The Android wrapper (`android/src/lib.rs`) contains one `unsafe` block for `std::env::set_var` (required by Rust edition 2024 — sets HOME path for config resolution on Android).

## Known Vulnerabilities

None. `cargo audit` can verify against the RustSec Advisory Database.
