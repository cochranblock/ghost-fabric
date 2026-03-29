<!-- Unlicense — cochranblock.org -->

# FIPS 140-2/3 Compliance Status

## Current State: Not Applicable

Ghost-fabric does not currently implement cryptographic operations beyond CSPRNG-based node ID generation. FIPS 140-2/3 validation is not required for the current build.

## Cryptographic Primitives in Use

| Primitive | Library | FIPS Validated? | Usage |
|-----------|---------|----------------|-------|
| ChaCha20 (CSPRNG) | rand v0.8 (rand_chacha) | No | Node ID generation only |

## Planned Cryptographic Primitives

When mesh networking and packet encryption are implemented:

| Primitive | Planned Library | FIPS Status | Usage |
|-----------|----------------|-------------|-------|
| AES-256-GCM | ring or RustCrypto | ring uses BoringSSL (FIPS-capable); RustCrypto is not FIPS-validated | Mesh packet encryption |
| Argon2id | RustCrypto argon2 | Not FIPS-validated; NIST SP 800-63B recommends it | Node key derivation |
| HKDF-SHA256 | RustCrypto hkdf | SHA-256 is FIPS-approved (FIPS 180-4); implementation not validated | Session key expansion |
| Ed25519 | ring or ed25519-dalek | ring uses BoringSSL (FIPS-capable) | Node identity signing |

## Path to FIPS Compliance

1. **Use `ring` for crypto:** ring wraps BoringSSL, which has a FIPS 140-2 validated configuration (BoringCrypto module, CMVP cert #4407)
2. **Replace Argon2id:** For FIPS environments, use PBKDF2-HMAC-SHA256 (NIST SP 800-132) instead of Argon2id
3. **Build with FIPS flag:** BoringSSL supports `FIPS=1` build mode that restricts to validated algorithms
4. **Module boundary:** The FIPS cryptographic boundary would be the BoringSSL/ring layer, not the ghost-fabric binary itself

## For FIPS-Required Deployments

If deployed in a federal environment requiring FIPS 140-2/3:
- Crypto operations must use a CMVP-validated module
- `ring` with BoringCrypto is the most viable Rust path
- Self-certification is insufficient — must reference an existing CMVP certificate
- Document the validated module version and certificate number in deployment artifacts
