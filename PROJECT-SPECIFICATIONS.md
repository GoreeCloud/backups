# GoreeCloud Backups — Project Specifications

**Document Type:** Repository-Native Project Specification  
**Status:** Active specification / planned implementation  
**Project:** GoreeCloud Backups  
**Authority:** Repository-local project specification  
**Last Updated:** 2026-09-24

## 1. Product Definition

GoreeCloud Backups is GoreeCloud's native backup, restoration, verification, and recovery platform for applications, services, servers, workstations, infrastructure, and other approved data sources.

The product is defined by a recovery-first model: backup creation alone is not sufficient evidence of recoverability. Protection status must reflect the existence, integrity, accessibility, credential availability, restoration capability, and validation state of the applicable recovery point.

## 2. Native Development Boundary

GoreeCloud Backups is an original GoreeCloud application and backup engine.

Its core backup engine, repository format, snapshot model, restoration system, verification system, protection-policy model, control plane, API, CLI, and user experience are to be designed and maintained as GoreeCloud software.

Kopia, Restic, Rclone, and other third-party projects may influence design or be supported through optional interoperability, storage, migration, transport, protocol, or compatibility mechanisms. They are not the upstream product from which GoreeCloud Backups is derived and must not define the native GoreeCloud Backups repository format or product architecture.

Established third-party implementations may be used for cryptography, compression, protocols, storage SDKs, and similar supporting functions where appropriate. GoreeCloud Backups must not invent custom cryptographic algorithms.

## 3. Core System Scope

The planned system scope includes:

- A native backup and restore engine.
- Incremental and versioned snapshot processing.
- Content-defined chunking and content-addressed storage.
- Cross-snapshot and cross-file deduplication.
- Compression before encrypted storage.
- Authenticated client-side encryption and repository key management.
- An open, versioned native repository format.
- Encrypted pack storage, indexing, compaction, and maintenance.
- Multiple independent repositories.
- Local, removable, mounted, network, object-storage, SFTP, and other approved storage backends.
- Backup agents for protected systems.
- A future GoreeCloud Backups Repository Server.
- Application-consistent backup workflows.
- Protection Profiles and reusable Protection Policies.
- Native scheduling, retention, garbage collection, and repository maintenance.
- Repository and data-integrity verification.
- Native restore planning, execution, testing, and validation.
- Recovery-evidence records and explicit evidence-backed protection states.
- Native monitoring, notifications, activity history, audit records, diagnostics, CLI, API, and Glaze UI.
- Central multi-system administration without unnecessary centralization of plaintext backup data.
- Migration and interoperability tooling for historical or external backup systems.

The complete accepted planned capability set is maintained in [PLANNED-FEATURES.md](./PLANNED-FEATURES.md).

## 4. Recovery Architecture Requirements

The architecture must support independent recovery when the original application installation, agent, workstation, server, control-plane database, graphical interface, or other original execution environment has been lost.

A supported clean-environment recovery path is expected to require only:

1. A trusted GoreeCloud Backups release.
2. The approved repository location.
3. The required recovery credentials.
4. Repository access and verification.
5. Recovery-point selection.
6. Restoration.
7. Restoration validation.

Centralized services may add administration, visibility, orchestration, and policy capabilities but must not become an unnecessary single point of failure for basic repository recovery.

## 5. Security and Destructive-Event Resistance

The system is planned to separate ordinary backup-writing authority from maintenance, recovery, and repository-administration authority where practical.

The design is expected to support:

- Restricted deletion privileges.
- Immutable or logically immutable recovery data where appropriate.
- Object-lock or equivalent provider controls where available.
- Independent repository accounts and credentials.
- Offline and off-site repositories.
- Multiple repositories for recovery independence.
- Strong safeguards around destructive operations.
- Audit records for retention and deletion operations.
- Client-side encryption where the repository model supports it.
- Recovery credentials that remain usable without the original machine.

A healthy repository that cannot be decrypted when required must not be represented as fully recoverable.

## 6. Application Consistency

Application-consistent recovery is a first-class requirement.

The system may use pre-backup and post-backup hooks, database dumps, checkpoints, application-native exports, filesystem freeze operations, service coordination, container coordination, staging, cleanup, and validation.

A successful filesystem copy must not automatically be described as application-consistent.

## 7. Verification and Recovery Evidence

Repository verification, integrity verification, restore testing, and validation are core product capabilities rather than optional maintenance functions.

The product must distinguish among:

- A backup operation that completed.
- A snapshot that exists.
- A repository that is healthy.
- A recovery point whose required objects and credentials are available.
- A representative restoration that succeeded.
- A restoration whose resulting application or dataset was validated.

Protection states and user-interface presentation must be backed by the evidence actually available.

## 8. Integration Boundaries

Planned GoreeCloud integrations include GoreeCloud Notify, Manager, Everkeep, Wardveil Security, Privacy Shield, Policy, Observability, Mesh, Sync, and Identity.

These integrations must preserve authority boundaries:

- GoreeCloud Backups remains authoritative for technical backup, repository, verification, restore, and recovery-evidence state.
- Everkeep remains responsible for broader continuity, preservation, portability, resilience, succession, and digital-legacy concerns.
- GoreeCloud Sync remains a synchronization and availability system; synchronization is never automatically treated as backup.
- Presentation layers such as Manager or Wardveil must not replace underlying GoreeCloud Backups technical evidence.
- Network reachability through GoreeCloud Mesh must not itself establish backup authorization.

## 9. Privacy Requirements

GoreeCloud Backups must follow privacy-by-default principles.

Operational monitoring, notifications, observability, restore testing, diagnostics, and audit records should minimize protected content and must not unnecessarily expose backup data, filenames, secrets, encryption keys, repository passwords, or reusable credentials.

Backup administration must not become justification for routine inspection of protected personal data.

## 10. Failure-Safety Requirement

Backup, restore, retention, garbage collection, maintenance, repository transfer, and migration workflows must be designed to fail safely.

Process crashes, machine reboots, network interruption, repository unavailability, partial transfers, full disks, corruption, stale locks, concurrent operations, authentication expiration, and interrupted destructive workflows must not silently convert a previously valid repository into an unrecoverable one.

## 11. Testing and Stable Claims

Testing is expected to include unit, integration, repository-format, backup/restore round-trip, deduplication, encryption, corruption, interruption, retention, garbage-collection, migration, cross-machine recovery, storage-backend, permissions, application-restoration, and disaster-recovery coverage.

Backup creation without representative restoration is not sufficient evidence for a Stable recovery claim.

## 12. Authoritative Repository Records

- [PLANNED-FEATURES.md](./PLANNED-FEATURES.md) — accepted planned feature and capability scope.
- [IMPLEMENTED-FEATURES.md](./IMPLEMENTED-FEATURES.md) — verified implemented functionality only.
- [CHANGELOGS.md](./CHANGELOGS.md) — repository change history.
- [PROJECT-RECORD.md](./PROJECT-RECORD.md) — project history, decisions, milestones, and verified project-state records.

Planned functionality must not be copied into IMPLEMENTED-FEATURES.md until implementation evidence has been verified.
