# GoreeCloud Backups — Repository Specifications

**Document Type:** Repository-Coupled Application Specification  
**Status:** Active specification / Development engineering foundation integrated; no usable backup or restore functionality  
**Project:** GoreeCloud Backups  
**Repository:** `GoreeCloud/backups`  
**Last Updated:** 2026-09-24

## 1. Authority and Purpose

This file defines the repository-coupled implementation specification for GoreeCloud Backups.

The broader project specification is maintained in [PROJECT-SPECIFICATIONS.md](./PROJECT-SPECIFICATIONS.md). The accepted planned capability inventory is maintained in [PLANNED-FEATURES.md](./PLANNED-FEATURES.md).

If this file and those records differ, the more specific repository implementation requirement must remain consistent with the higher-level project specification and GoreeCloud governance.

## 2. Current Implementation State

The Milestone 0 native Rust Development engineering foundation is integrated on authoritative `main`.

Verified source-level foundation state includes the core/repository/CLI workspace, recovery-first evidence semantics, a fail-closed repository persistence boundary, a Development status CLI, and exact-head/post-merge Rust validation.

No usable file-backup, snapshot, persistent-repository, restore, storage-provider, agent, server, scheduler, or graphical product functionality is verified. Source-foundation integration must not be treated as runtime recovery evidence, release evidence, production acceptance, or Stable status.

## 3. Initial Technology Foundation

The initial native core implementation uses Rust 1.98.1.

Rust is selected for the core backup engine, repository-format implementation, restore engine, verification logic, CLI, and other security-sensitive or performance-critical components because these responsibilities require memory safety, predictable native performance, strong error handling, portability, and robust parsing/data-integrity boundaries.

Milestone 0 begins with no third-party Rust runtime dependencies. Cryptographic, compression, chunking, serialization, storage, networking, and protocol dependencies must be added only after project-specific security, maintenance, license, provenance, portability, and API review.

The repository-format V1 design candidate now defines the proposed profile `goreecloud-backups/repository-v1` and version 1.0. Persistent repository use remains deliberately blocked until the format implementation and its required cryptographic, parser, corruption, interruption, path-safety, index-rebuild, and clean-recovery tests are accepted.

See [TECHNOLOGY-DECISION.md](./TECHNOLOGY-DECISION.md), [MILESTONE-0.md](./MILESTONE-0.md), [REPOSITORY-FORMAT.md](./REPOSITORY-FORMAT.md), and [MILESTONE-1.md](./MILESTONE-1.md).

## 4. Native Product Boundary

GoreeCloud Backups must be implemented as an original GoreeCloud backup, restoration, verification, and recovery system.

The native product is expected to own its:

- Backup and restore engine.
- Snapshot and recovery-point model.
- Repository format.
- Repository indexing and maintenance behavior.
- Verification and recovery-evidence model.
- Protection-policy model.
- CLI and API.
- User experience.

Kopia, Restic, Rclone, and similar projects may be used only through explicitly bounded supporting roles such as interoperability, migration, transport, storage adapters, libraries, protocols, or engineering reference.

## 5. Recovery-First Requirement

The implementation must distinguish successful data capture from proven recoverability.

At minimum, the system must be able to represent separately:

- Backup-operation success.
- Snapshot existence.
- Repository health.
- Required-object availability.
- Recovery-credential availability.
- Restore-test success.
- Restoration validation.
- Evidence-backed protection state.

A completed backup operation alone must not produce a `Restore Verified` or equivalent state.

## 6. Clean-Environment Recovery

The repository format and recovery tooling must support recovery without dependence on the original application installation, control-plane database, workstation, server, agent, or graphical interface.

A supported clean-environment path is expected to require only a trusted release, the repository location, required recovery credentials, repository verification, recovery-point selection, restoration, and validation.

## 7. Repository and Storage Requirements

The planned native repository architecture must support:

- Versioned repository metadata.
- Content-defined chunking.
- Content-addressed data.
- Deduplication.
- Compression before encryption where applicable.
- Authenticated encryption.
- Pack/index structures suitable for local and remote storage.
- Integrity verification.
- Retention and garbage collection.
- Safe interrupted-operation recovery.
- Multiple independent repositories.
- Storage-provider portability.
- Controlled format evolution and migration.

The native repository format must not require one cloud provider or third-party backup engine.

The current V1 design candidate is defined in [REPOSITORY-FORMAT.md](./REPOSITORY-FORMAT.md). Format design acceptance is not permission to persist user backup data; the implementation gate in [MILESTONE-1.md](./MILESTONE-1.md) remains controlling.

## 8. Security and Privacy Requirements

Implementation must preserve least privilege, credential separation, recovery independence, safe destructive-operation controls, privacy-conscious monitoring, and minimal exposure of protected content.

The system must not invent custom cryptographic algorithms. Established and maintained cryptographic implementations must be used.

Sensitive values such as repository passwords, encryption keys, reusable credentials, and protected file contents must not be exposed in ordinary logs, notifications, observability signals, or diagnostic output.

## 9. Failure-Safety Requirements

Backup, restore, maintenance, retention, garbage collection, transfer, and migration workflows must fail safely.

Process crashes, host reboots, network interruption, full disks, repository unavailability, partial transfers, corruption, stale locks, credential expiry, concurrent operations, and interrupted destructive workflows must not silently convert previously valid recovery data into an unrecoverable state.

## 10. Integration Boundaries

Planned GoreeCloud integrations include Notify, Manager, Everkeep, Wardveil Security, Privacy Shield, Policy, Observability, Mesh, Sync, and Identity.

GoreeCloud Backups remains authoritative for technical backup, repository, verification, restore, and recovery-evidence state. Presentation or orchestration systems must not manufacture stronger protection claims than the underlying Backups evidence supports.

## 11. Acceptance Boundary

Implementation claims belong in [IMPLEMENTED-FEATURES.md](./IMPLEMENTED-FEATURES.md) only after verification.

Stable recovery claims require representative restoration evidence, not merely successful backup creation or source-level tests.

See [FEATURES.md](./FEATURES.md) for the current lifecycle summary and [CHANGELOGS.md](./CHANGELOGS.md) for repository changes.
