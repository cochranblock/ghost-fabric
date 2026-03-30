# Ghost Fabric

Sovereign edge intelligence over sub-GHz cognitive mesh networks. Rust CLI + Android app for LoRa 915MHz mesh node management. Currently a scaffold with node identity, config, and subsystem status reporting. Inference, radio, and mesh networking are planned — not yet implemented.

**Stack:** Rust, clap CLI, egui (Android), serde/JSON config
**License:** Unlicense

## Quick Start

```bash
cargo build --release
ghost-fabric init      # generate node identity
ghost-fabric status    # show node config
ghost-fabric start     # start mesh node
```

## Federal Compliance

See [`govdocs/`](govdocs/) — SBOM, SSDF, FIPS, CMMC, FedRAMP, ITAR/EAR, supply chain integrity, security posture, privacy assessment, and federal use cases.

---

Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. 14 Unlicense repos. [See all products →](https://cochranblock.org/products)
