# GoreeCloud Backups

GoreeCloud Backups is GoreeCloud's planned native backup, restoration, verification, and recovery platform for applications, services, servers, workstations, infrastructure, and other approved data sources.

> **Current state:** specification and repository documentation only. No GoreeCloud Backups application functionality is currently verified as implemented in this repository.

## Product Direction

GoreeCloud Backups is intended to be an original GoreeCloud application and backup engine. It is not planned as a fork of Kopia, Restic, Rclone, or another third-party backup product.

Its governing product principle is recovery-first:

**A backup is not proven merely because it was created. Recovery is proven when the required recovery point exists, remains intact and accessible, the required credentials are available, and the protected information can be successfully restored and validated.**

The planned platform includes a native backup and restore engine, versioned snapshots, deduplication, compression, authenticated encryption, an open versioned repository format, multiple repository backends, application-consistent protection, retention, integrity verification, restore testing, recovery evidence, monitoring, notifications, CLI/API access, Glaze UI, and GoreeCloud integrations.

## Authoritative Repository Records

| Record | Purpose |
| --- | --- |
| [PROJECT-SPECIFICATIONS.md](./PROJECT-SPECIFICATIONS.md) | Project identity, scope, architecture, recovery model, security/privacy boundaries, and acceptance requirements |
| [SPECIFICATIONS.md](./SPECIFICATIONS.md) | Repository-coupled implementation specification and authority map |
| [PLANNED-FEATURES.md](./PLANNED-FEATURES.md) | Accepted planned features and capabilities |
| [FEATURES.md](./FEATURES.md) | Current functionality and lifecycle summary |
| [IMPLEMENTED-FEATURES.md](./IMPLEMENTED-FEATURES.md) | Verified implemented functionality only |
| [BENEFITS.md](./BENEFITS.md) | Planned product benefits and the evidence boundary for claiming them |
| [COMPETITIVE-OBJECTIVES.md](./COMPETITIVE-OBJECTIVES.md) | Differentiation and evaluation objectives without unverified superiority claims |
| [SECURITY.md](./SECURITY.md) | Repository-safe security guidance and reporting expectations |
| [NOTES.md](./NOTES.md) | Current repository notes and unresolved baseline dependencies |
| [BRANDING.md](./BRANDING.md) | Branding authority and current Backups asset-establishment status |
| [USER-MANUAL.md](./USER-MANUAL.md) | Pre-implementation user manual and future operational guidance |
| [PRIVACY POLICY.md](./PRIVACY%20POLICY.md) | Privacy requirements and pre-implementation disclosure |
| [goreecloud.platform.yaml](./goreecloud.platform.yaml) | GoreeCloud Platform Contract 0.4 declaration |
| [PROJECT-RECORD.md](./PROJECT-RECORD.md) | Significant project history and verified project-state records |
| [CHANGELOGS.md](./CHANGELOGS.md) | Repository-local change history |

## Implementation-State Rule

Documentation, design intent, accepted plans, architecture, and repository records are not implementation evidence.

A capability must not be described as implemented, released, production-ready, or Stable until the applicable source, tests, recovery evidence, security controls, target-environment behavior, and acceptance gates have been verified.

## Third-Party Boundary

Kopia, Restic, Rclone, and similar technologies may be supported for interoperability, migration, transport, storage access, libraries, protocols, or engineering reference. They do not define the native GoreeCloud Backups product identity, engine, or repository format.

## Repository

This repository is the authoritative GitHub location for GoreeCloud Backups project documentation and future implementation.


## License

GoreeCloud Backups is currently governed by `AGPL-3.0-or-later` through the GoreeCloud default fallback license policy because no Backups-specific superseding license decision is recorded.

See [LICENSE](./LICENSE) and [LICENSE-DECISION.md](./LICENSE-DECISION.md).
