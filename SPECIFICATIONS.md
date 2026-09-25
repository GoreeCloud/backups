# GoreeCloud Backups — Specifications

**Status:** Active specification / planned implementation  
**Lifecycle:** Concept / planning baseline  
**Implementation State:** No application functionality currently verified  
**Last Updated:** 2026-09-24

## Purpose

GoreeCloud Backups is intended to provide native GoreeCloud backup, restoration, verification, and recovery for approved files, applications, services, servers, workstations, infrastructure, databases, container environments, and other supported data sources.

## Core Requirements

The product is planned to provide:

- A GoreeCloud-developed backup and restore engine.
- Incremental, versioned recovery snapshots.
- Content-defined chunking and content-addressed storage.
- Cross-snapshot and cross-file deduplication.
- Compression before encrypted repository storage.
- Authenticated client-side encryption with recoverable key-management architecture.
- An open, versioned GoreeCloud Backups repository format.
- Independent repository storage providers and multiple repositories.
- Application-consistent backup mechanisms.
- Native scheduling, retention, maintenance, and garbage collection.
- Repository, integrity, and recovery verification.
- Restore planning, restore execution, restore testing, and validation.
- Recovery evidence that distinguishes completed backup operations from demonstrated recoverability.
- Explicit protection states backed by evidence.
- Native CLI, API, diagnostics, monitoring, notifications, activity history, and Glaze UI.
- Clean-environment recovery that does not require the original control plane to survive.
- Optional interoperability and migration tooling for other backup ecosystems without making them the native engine.

## Recovery-First Requirement

Backup completion alone must not be represented as proof of recoverability.

Protection and recovery status must account for:

- Recovery-point existence.
- Repository accessibility.
- Repository and data integrity.
- Required credential availability.
- Restore capability.
- Restore-test evidence.
- Validation of restored data or applications where required.

## Product Boundaries

GoreeCloud Backups is not intended to become:

- A synchronization replacement for GoreeCloud Sync.
- The broader continuity, succession, preservation, and digital-legacy layer provided by Everkeep.
- A fork or rebranding of Kopia, Restic, Rclone, or another backup product.

## Security and Privacy

The design must support privacy by default, least privilege, client-side encryption where appropriate, credential separation, destructive-event resistance, explicit authorization boundaries, protected recovery credentials, and minimal exposure of protected content in logs, metrics, notifications, diagnostics, and administration.

## Failure Safety

Interrupted backup, restore, migration, retention, and maintenance operations must fail safely and must not silently invalidate previously recoverable repository state.

## Stable Qualification

Stable recovery claims require representative restore evidence. Successful backup creation, successful repository writes, or passing narrow unit tests are not sufficient by themselves.

## Authoritative Detail

The full accepted planned-capability scope is maintained in [PLANNED-FEATURES.md](./PLANNED-FEATURES.md).

Project-level governance and authority boundaries are maintained in [PROJECT-SPECIFICATIONS.md](./PROJECT-SPECIFICATIONS.md).
