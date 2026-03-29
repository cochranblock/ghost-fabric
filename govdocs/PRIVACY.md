<!-- Unlicense — cochranblock.org -->

# Privacy Impact Assessment

## Data Collection

| Data Type | Collected? | Details |
|-----------|-----------|---------|
| Personal Identifiable Information (PII) | No | No user accounts, no names, no emails |
| Location data | No | No GPS, no geolocation |
| Telemetry / analytics | No | No phone-home, no usage tracking |
| Sensor readings | Future | Will process locally, never transmit raw data off-node |
| Network traffic | No | No network connections in current build |

## Data Storage

| Item | Location | Content | PII? |
|------|----------|---------|------|
| Node config | `~/.config/ghost-fabric/node.json` (Linux/macOS) | Random node ID, radio parameters | No |

The node ID (`gf-{random hex}`) is a random 64-bit value. It is not derived from hardware identifiers, IP addresses, or user data. It cannot be used to identify a person.

## Data Transmission

Ghost-fabric currently transmits **no data** over any network. The binary runs entirely locally.

**Future state:** When LoRa mesh networking is implemented, the design transmits only compressed decisions — never raw sensor data. Data stays on the node where it was generated.

## Third-Party Data Sharing

None. No analytics services, no cloud APIs, no external databases.

## GDPR Applicability

Ghost-fabric does not process personal data as defined by GDPR Article 4(1). If future versions process data that could identify natural persons (e.g., perimeter security classifications), a Data Protection Impact Assessment (DPIA) will be required.

## CCPA Applicability

Ghost-fabric does not collect, sell, or share consumer personal information. CCPA does not apply to the current build.

## Data Retention

Node config persists until the user deletes it. No automatic data retention, no logs, no databases.

## Verification

```bash
# Confirm no network activity
# (binary makes zero network calls)
cargo run --release -- start

# Inspect stored data
cat ~/.config/ghost-fabric/node.json
```
