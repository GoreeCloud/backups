# GoreeCloud Backups — Project Record

**Document Type:** Repository-Native Project Record  
**Status:** Active  
**Project:** GoreeCloud Backups  
**Authority:** Repository-local project record  
**Last Updated:** 2026-09-24

## 2026-09-24 — Native GoreeCloud Backups Definition Established

### Verified repository state

At the beginning of the initial documentation operation, the GoreeCloud/backups repository existed on GitHub but contained no committed project files.

### Recorded product direction

GoreeCloud Backups is established as a native GoreeCloud backup, restoration, verification, and recovery platform.

The planned architecture is explicitly independent of Kopia, Restic, Rclone, and other third-party backup products. These projects may be used for interoperability, migration, transport, libraries, protocols, or engineering reference where appropriate, but they do not define the native product identity, backup engine, or repository format.

### Recovery-first governing principle

The project is governed by the principle that successful backup creation alone is not proof of recoverability.

Recovery is considered demonstrated only when the required recovery point exists, remains intact and accessible, the required credentials are available, and the protected information can be restored and validated to the level required by policy.

### Documentation baseline established

The repository maintains PLANNED-FEATURES.md as the complete accepted planned-capability record, PROJECT-SPECIFICATIONS.md as the project-level specification and authority map, PROJECT-RECORD.md as the project history and verified-state record, IMPLEMENTED-FEATURES.md as the verified-implementation record, and CHANGELOGS.md as the repository change-history record.

### Implementation-state boundary

No application functionality is recorded as implemented merely because the planned specification exists.

Future implementation claims must be added to IMPLEMENTED-FEATURES.md only after the corresponding functionality and evidence have been verified.

### Initial repository documentation commits

The first authoritative planned-capability record was committed to the repository on 2026-09-24. Subsequent bootstrap documentation established the mandatory repository-native project records.

## 2026-09-24 — Repository Documentation Baseline Extended

### Structural reconciliation

A follow-on documentation pass reconciled the repository against GoreeCloud repository-structure governance without changing the product implementation state.

The pass replaced the skeletal README with a complete repository entry point, added SPECIFICATIONS.md as the repository-coupled implementation specification, added FEATURES.md as the current functionality and lifecycle summary, cross-linked the project specification, planned features, implemented features, project record, and changelog, and preserved the authoritative statement that no GoreeCloud Backups application functionality is yet verified as implemented.

### Scope boundary

This documentation reconciliation does not establish source implementation, runtime validation, release readiness, production acceptance, or Stable status.

## 2026-09-24 — Repository Governance Controls Extended

### Added controls

A second documentation pass added repository controls that could be established without inventing product implementation or unresolved policy decisions:

- BENEFITS.md records planned benefits with an explicit verification boundary.
- COMPETITIVE-OBJECTIVES.md records differentiation and comparison objectives without claiming current superiority.
- SECURITY.md records development-stage security guidance and sensitive-information restrictions without claiming a verified production security posture.
- NOTES.md records current repository state and unresolved baseline dependencies.
- .gitignore and .editorconfig provide conservative repository-wide defaults without selecting an implementation language or build system.

### Remaining baseline dependencies

Licensing, product-specific branding verification, privacy-policy language, user-manual maturity, and the machine-readable GoreeCloud Platform Contract remain dependent on separate authoritative inputs and must not be guessed.

The portfolio-level repository-baseline task remains the tracking authority for those unresolved obligations.


## 2026-09-24 — Repository Governance Baseline Completed

The remaining baseline records were added without changing product implementation state:

- BRANDING.md.
- USER-MANUAL.md.
- PRIVACY POLICY.md.
- .github/PULL_REQUEST_TEMPLATE.md.
- goreecloud.platform.yaml using Platform Contract schema 0.4.

The Platform Contract evaluates exactly nine Integral Platform Systems and marks each application-specific integration applicable-blocked because no implementation or acceptance evidence exists.

The canonical branding-assets repository was checked and no Backups-specific approved asset was verified. BRANDING.md therefore records the gap and prohibits treating substitute artwork as approved branding.

No repository license or approved rights notice was verified. Licensing remains unresolved.

This baseline does not establish a backup engine, restore engine, repository implementation, supported runtime, release, deployment, production acceptance, or Stable status.


## 2026-09-24 — License and Branding Baseline Resolved

The repository baseline was reconciled against current GoreeCloud licensing and branding authority.

### Licensing

The active GoreeCloud Software Licensing Policy establishes `AGPL-3.0-or-later` as the default fallback when no project-specific superseding license exists.

No Backups-specific superseding license decision was verified, so the repository now records the current governing fallback through:

- `LICENSE`.
- `LICENSE-DECISION.md`.
- README license disclosure.

This resolves the prior repository-rights gap without claiming that a final Backups-specific license-selection analysis has been completed.

### Branding

The canonical branding repository was re-audited and an approved historical Backup product icon was found at `products/backup/app-icon.svg`. Its visual identity remains suitable for the current native Backups product, while its metadata and provenance were stale.

The canonical branding authority was migrated to:

- Product ID: `backups`.
- Product name: GoreeCloud Backups.
- Canonical path: `products/backups/app-icon.svg`.
- Consumer repository: `GoreeCloud/backups`.
- Verified asset blob: `7a73f739c43ca35ad043b68cae632e84a8a68218`.

The stale singular asset path was removed from the current branding tree. The approved geometry was preserved; only product identity/provenance metadata changed.

Visual review confirmed the icon remains recognizable at full size and at 64 px, 32 px, and 16 px raster equivalents. Current consumer-eligible Glaze authority remains V1.6 / 1.6.0 Stable; V1.7 remains Development and does not replace the Stable target.

### Lifecycle

The repository Platform Contract and feature summary now align to the authoritative portfolio lifecycle of Development.

Development status does not establish any backup-engine, repository, restore, runtime, release, deployment, production, or Stable functionality. No application functionality is yet verified as implemented.


## 2026-09-24 — Milestone 0 Native Source Foundation Integrated

The first native GoreeCloud Backups source foundation was accepted and integrated through Pull Request #5.

### Technology decision

Rust 1.98.1 is the initial primary language for the security-sensitive and performance-critical core backup engine, repository-format implementation, restore engine, verification logic, and command-line foundation.

The initial workspace introduces no third-party Rust runtime dependencies. Future cryptography, compression, chunking, storage, networking, serialization, and protocol dependencies remain subject to separate security, licensing, provenance, portability, and maintenance review.

### Integrated source foundation

The integrated workspace contains:

- `goreecloud-backups-core`.
- `goreecloud-backups-repository`.
- `goreecloud-backups-cli`.
- Recovery-first evidence semantics.
- A fail-closed repository persistence gate.
- A Development CLI that reports backup/restore as not implemented and Stable eligibility as false.
- Pinned Rust and GitHub Actions validation.

Repository persistence is deliberately unavailable until the first versioned native repository format and compatibility rules are specified and accepted.

### Verification

Exact PR head `23d26fa2cfdd60a3227c60621e9defc439736751` passed Rust Foundation run `36096359838`, including exact-head checkout assertion, Rust 1.98.1 installation, formatting, locked metadata, clippy with warnings denied, tests, build, and CLI truth checks.

The PR was expected-head guarded and squash-merged as `b997478c447d11b902cca1ff82633ad370089ede`.

The exact merged main revision passed Rust Foundation push run `36096435671`.

### Acceptance boundary

Milestone 0 establishes a verified Development engineering foundation only.

It does not implement file backup, snapshotting, deduplication, compression, encryption, repository persistence, remote storage, restoration, agents, repository servers, scheduling, application-consistent protection, Glaze UI, runtime Integral Platform System integrations, deployment, production acceptance, or Stable qualification.

The next implementation gate is the first versioned native repository-format specification. Persistent repository-writing code must remain blocked until that format, compatibility policy, corruption behavior, atomicity model, and clean-environment recovery boundary are accepted.
