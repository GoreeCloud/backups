# GoreeCloud Backups — Features

**Document Type:** Current Functionality and Lifecycle Summary  
**Status:** Active  
**Project:** GoreeCloud Backups  
**Release Lifecycle:** Development  
**Last Updated:** 2026-09-24

## Current Verified Functionality

No usable GoreeCloud Backups backup or restore product functionality is currently verified as implemented.

The repository provides the project-definition and governance baseline, and Pull Request #5 integrated the first native Milestone 0 Development source foundation on `main`.

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

### Integrated Development source foundation

Pull Request #5 added a Rust 1.98.1 workspace, recovery-first evidence invariants, a fail-closed repository persistence boundary, a Development status CLI, and pinned exact-head source validation.

The integrated foundation intentionally keeps repository persistence disabled and does not implement backup or restore operations. Exact-head validation run 36096359838 passed at PR head `23d26fa2cfdd60a3227c60621e9defc439736751`; guarded squash merge `b997478c447d11b902cca1ff82633ad370089ede` was then verified by successful main push run 36096435671.

### Repository Format V1 design candidate

The current design branch proposes native repository profile `goreecloud-backups/repository-v1`, version 1.0.

The proposed format defines:

- Minimal immutable plaintext bootstrap metadata.
- Password-protected recovery key slots and a random repository master key.
- Standards-based KDF, key separation, keyed content identifiers, and authenticated encryption profiles.
- Immutable encrypted pack objects with independently authenticated records.
- Encrypted pack tables of contents from which indexes can be rebuilt.
- Encrypted immutable snapshot manifests published last as the snapshot commit record.
- Rebuildable encrypted indexes.
- Privacy-minimized recovery evidence objects.
- Safe reachability-based retention and garbage collection.
- Clean-environment recovery without the original control plane, agent database, scheduler, or GUI.

The Rust repository crate exposes the proposed V1 identity but continues to fail closed with persistence blocked until implementation and recovery-test acceptance. This is format design/source-boundary work, not usable backup functionality.

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
