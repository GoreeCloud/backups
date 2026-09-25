# GoreeCloud Backups

GoreeCloud Backups is GoreeCloud's planned native backup, restoration, verification, and recovery platform for applications, services, servers, workstations, infrastructure, and other approved data sources.

> **Current state:** Development. The Milestone 0 native Rust engineering foundation is integrated on `main`, but no usable backup, repository-persistence, or restore functionality is implemented.

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
| [BRANDING.md](./BRANDING.md) | Approved Backups identity, canonical asset provenance, and branding boundaries |
| [USER-MANUAL.md](./USER-MANUAL.md) | Pre-implementation user manual and future operational guidance |
| [PRIVACY POLICY.md](./PRIVACY%20POLICY.md) | Privacy requirements and pre-implementation disclosure |
| [goreecloud.platform.yaml](./goreecloud.platform.yaml) | GoreeCloud Platform Contract 0.4 declaration |
| [TECHNOLOGY-DECISION.md](./TECHNOLOGY-DECISION.md) | Initial core implementation language and technology boundaries |
| [MILESTONE-0.md](./MILESTONE-0.md) | Native source-foundation scope and acceptance gates |
| [PROJECT-RECORD.md](./PROJECT-RECORD.md) | Significant project history and verified project-state records |
| [CHANGELOGS.md](./CHANGELOGS.md) | Repository-local change history |

## Milestone 0 Development Foundation

Pull Request #5 integrated the first native source foundation using Rust 1.98.1 as `b997478c447d11b902cca1ff82633ad370089ede`.

The integrated foundation includes:

- A dependency-free Rust workspace for the initial core.
- Recovery-first evidence semantics.
- A repository-format boundary that fails closed and refuses persistent repository use until a format is accepted.
- A minimal Development status CLI.
- Pinned source validation for formatting, linting, tests, build, and CLI truth.

This foundation does **not** provide usable backup or restore functionality. Exact-head validation run 36096359838 and post-merge main run 36096435671 both passed.

See [TECHNOLOGY-DECISION.md](./TECHNOLOGY-DECISION.md) and [MILESTONE-0.md](./MILESTONE-0.md).

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


## Branding

The approved GoreeCloud Backups application icon is maintained in `GoreeCloud/branding-assets` at `products/backups/app-icon.svg`. Branding establishes product identity only and does not imply application implementation, Glaze UI application conformance, release readiness, or production acceptance.
