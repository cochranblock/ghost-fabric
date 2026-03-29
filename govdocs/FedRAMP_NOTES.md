<!-- Unlicense — cochranblock.org -->

# FedRAMP Applicability Notes

## Deployment Model

Ghost-fabric is **not a cloud service**. It is a single binary deployed to edge hardware. FedRAMP authorization is not required for the software itself.

| Question | Answer |
|----------|--------|
| Is this SaaS? | No |
| Is this PaaS? | No |
| Is this IaaS? | No |
| Is this on-premises? | Yes — runs on bare metal edge nodes |
| Does it connect to cloud services? | No |
| Does it store data in the cloud? | No |

## Authorization Boundary

The authorization boundary is the **physical edge node** running the ghost-fabric binary. There is no cloud component, no shared infrastructure, and no multi-tenant access.

```
┌─────────────────────────────┐
│  Physical Edge Node         │
│  ┌───────────────────────┐  │
│  │  ghost-fabric binary  │  │
│  │  - config (local fs)  │  │
│  │  - inference (local)  │  │
│  │  - radio (LoRa TX/RX) │  │
│  └───────────────────────┘  │
│  Authorization boundary ──────── here
└─────────────────────────────┘
         │ LoRa 915MHz (air gap)
         ▼
┌─────────────────────────────┐
│  Adjacent Edge Node         │
│  (same boundary model)      │
└─────────────────────────────┘
```

## When FedRAMP Would Apply

FedRAMP would apply if:
- A hosted management console is built to monitor mesh nodes remotely
- Node data is aggregated in a cloud dashboard
- A SaaS layer is added for fleet management

None of these exist today. The architecture is explicitly designed to avoid cloud dependencies.

## Relevant Framework Instead

For on-premises/edge deployments in federal environments, the applicable frameworks are:
- **NIST SP 800-53** (Security and Privacy Controls) — applies to the information system the node is part of
- **NIST SP 800-171** (CUI protection) — if processing Controlled Unclassified Information
- **CMMC** — if part of a DoD contractor's environment (see CMMC.md)
- **STIG** — if deployed on DoD networks, a STIG would need to be developed for the binary
