# GoreeCloud Backups — Features

**Document Type:** Current Functionality and Lifecycle Summary  
**Status:** Active  
**Project:** GoreeCloud Backups  
**Release Lifecycle:** Development  
**Last Updated:** 2026-09-24

## Current Verified Functionality

No GoreeCloud Backups application functionality is currently verified as implemented.

The repository provides the project-definition and governance baseline, and Draft Pull Request #5 contains the first native Milestone 0 Development source foundation.

### Verified repository capabilities

- Repository-native project specification exists.
- Repository-native project record exists.
- Planned feature lifecycle record exists.
- Implemented feature lifecycle record exists.
- Repository changelog exists.
- Repository-coupled specification exists.
- Current feature/lifecycle summary exists.
- Governed AGPL-3.0-or-later fallback licensing is established.
- Approved product branding is established in `GoreeCloud/branding-assets` at `products/backups/app-icon.svg`.
- Platform Contract 0.4 declaration exists with all nine Integral Platform Systems still blocked pending implementation and acceptance.

These documentation capabilities are repository governance state; they are not backup-engine or product-runtime functionality.

### Development source candidate

Draft Pull Request #5 adds a Rust 1.98.1 workspace, recovery-first evidence invariants, a fail-closed repository persistence boundary, a Development status CLI, and pinned source validation.

The candidate intentionally keeps repository persistence disabled and does not implement backup or restore operations. Exact-head CI and governed integration remain required before the source foundation is treated as integrated main-branch state.

## Planned Product Capabilities

The accepted planned capability set includes native backup and restore, snapshots, deduplication, compression, authenticated encryption, repository storage, retention, verification, restore testing, recovery evidence, monitoring, notifications, CLI/API surfaces, Glaze UI, multi-system administration, and GoreeCloud integrations.

The complete planned record is [PLANNED-FEATURES.md](./PLANNED-FEATURES.md).

## Implemented Feature Authority

Only [IMPLEMENTED-FEATURES.md](./IMPLEMENTED-FEATURES.md) may record verified product implementation state.

A feature must not move from planned to implemented merely because it is described in documentation, accepted as a requirement, designed, scaffolded, or partially coded.

## Lifecycle Boundary

The following states are distinct and must not be collapsed:

- Planned.
- Source implemented.
- Tested.
- Runtime verified.
- Recovery verified.
- Released.
- Production accepted.
- Stable.

Future entries should identify the strongest verified state supported by authoritative evidence.
