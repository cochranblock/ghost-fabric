<!-- Unlicense — cochranblock.org -->

# Export Control Classification — ITAR/EAR

## Classification Summary

Ghost-fabric is **publicly available open-source software** released under the Unlicense (public domain equivalent). Under EAR Section 734.7 and ITAR Section 120.11, published open-source software is generally excluded from export controls.

## EAR Analysis

### Category 5, Part 2 — Information Security (Cryptography)

| Question | Answer |
|----------|--------|
| Does the software perform encryption? | No (current build) |
| Does it contain cryptographic source code? | No — only CSPRNG for random number generation |
| Is encryption the primary function? | No |
| Is the crypto publicly available? | Yes — all source on GitHub, Unlicense |

**Current classification: EAR99** — no export license required.

### Future State (when mesh encryption is added)

When AES-256-GCM packet encryption is implemented:
- The software would contain encryption functionality
- However, it remains publicly available open-source
- EAR Section 742.15(b) provides a License Exception TSU (Technology and Software Unrestricted) for publicly available encryption source code
- **Requirement:** File a notification with BIS (Bureau of Industry and Security) and NSA per EAR 740.13(e) before or at the time of publication
- **Action item:** When crypto is added, email `crypt@bis.doc.gov` and `enc@nsa.gov` with the repository URL

## ITAR Analysis

| Question | Answer |
|----------|--------|
| Is this on the USML (United States Munitions List)? | No |
| Does it contain defense articles? | No |
| Is it specifically designed for military use? | No — general-purpose edge computing |
| Does it contain classified information? | No |

Ghost-fabric is not ITAR-controlled. It is general-purpose edge intelligence software. While it lists "border and perimeter security" as an application, the software itself is a generic mesh networking + inference platform with no military-specific design elements.

## Sanctions Screening

As open-source software on GitHub, ghost-fabric is available globally. No sanctions screening is required for public repository access. If commercial licensing or support contracts are offered to specific entities, OFAC screening would apply at that point.

## Recommendations

1. Maintain public availability of all source code
2. When encryption is added: file BIS/NSA notification
3. Do not add classified or CUI data to the repository
4. If military-specific features are requested: conduct a fresh ITAR review before implementation
