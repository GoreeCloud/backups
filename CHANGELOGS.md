# GoreeCloud Backups — Changelogs

All notable repository changes should be recorded here without representing planned functionality as implemented.

## Unreleased

### 2026-09-24

- Established the authoritative PLANNED-FEATURES.md record for the native GoreeCloud Backups platform.
- Defined the recovery-first product principle and evidence-backed protection model.
- Recorded the requirement that GoreeCloud Backups remain an original GoreeCloud application rather than a fork of Kopia, Restic, Rclone, or another backup product.
- Added repository-native project specification and project record documents.
- Added the verified implemented-feature record with no application functionality yet claimed as implemented.
- Expanded README.md into a truthful repository entry point with lifecycle status, product direction, authority links, and implementation-state boundaries.
- Added SPECIFICATIONS.md as the repository-coupled implementation specification and authority bridge.
- Added FEATURES.md as the current functionality and lifecycle summary, explicitly preserving the no-verified-implementation state.
- Added BENEFITS.md and COMPETITIVE-OBJECTIVES.md as planned, evidence-bounded product objectives.
- Added SECURITY.md and NOTES.md with repository-safe development guidance and remaining baseline dependencies.
- Added conservative repository-wide .gitignore and .editorconfig controls without assuming an implementation language or build system.

- Added BRANDING.md with the verified current absence of a Backups-specific canonical branding asset and the requirement not to invent substitute branding.
- Added USER-MANUAL.md as a pre-implementation user manual that does not claim unavailable workflows.
- Added PRIVACY POLICY.md as a pre-implementation privacy requirements and disclosure record.
- Added .github/PULL_REQUEST_TEMPLATE.md for state, verification, security/privacy, and recovery-aware change review.
- Added the GoreeCloud Platform Contract 0.4 declaration with all nine Integral Platform Systems marked applicable-blocked pending implementation and acceptance.

- Applied the GoreeCloud default `AGPL-3.0-or-later` fallback license through `LICENSE` and `LICENSE-DECISION.md`; no Backups-specific superseding license decision is recorded.
- Re-audited `GoreeCloud/branding-assets`, migrated the approved Backup icon to the current `products/backups/app-icon.svg` identity, updated the canonical catalog to `GoreeCloud Backups` / `GoreeCloud/backups`, and removed the stale singular asset path.
- Verified the canonical Backups icon remains recognizable at full size and 64 px / 32 px / 16 px raster equivalents without relying on color alone for its recovery meaning.
- Reconciled repository lifecycle metadata to Development while preserving the no-verified-implementation boundary.
- Removed resolved licensing and branding blockers from current repository status records; product implementation, runtime acceptance, platform integrations, Glaze application conformance, release, deployment, production acceptance, and Stable qualification remain open.

- Integrated Pull Request #5 as the first native Milestone 0 source foundation.
- Selected Rust 1.98.1 as the initial core engine/repository/CLI language and recorded the decision in TECHNOLOGY-DECISION.md.
- Added a dependency-free Rust workspace with core, repository, and CLI crates.
- Added recovery-first evidence logic that requires repository accessibility/integrity, recovery credentials, successful restoration, and restored-data validation rather than treating backup existence as restore proof.
- Added a fail-closed repository persistence gate so no persistent native repository format can be used before its specification is accepted.
- Added a Development CLI that truthfully reports backup/restore as not implemented and Stable eligibility as false.
- Added pinned exact-head Rust source validation for formatting, metadata, clippy, tests, build, and CLI truth checks.
- Verified exact PR head `23d26fa2cfdd60a3227c60621e9defc439736751` through Rust Foundation run `36096359838`, guarded squash merge `b997478c447d11b902cca1ff82633ad370089ede`, and exact merged-main push run `36096435671`.
- Preserved the implementation boundary: Milestone 0 is an integrated Development engineering foundation, not usable backup or restore functionality.
