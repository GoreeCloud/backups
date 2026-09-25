# GoreeCloud Backups

GoreeCloud Backups is the planned native GoreeCloud backup, restoration, verification, and recovery platform for applications, services, servers, workstations, infrastructure, and other approved data sources.

> **Current repository state:** Documentation and product-definition baseline only. No backup engine, restore engine, repository implementation, user interface, agent, server, or production capability is currently verified as implemented in this repository.

## Product Principle

**A backup is not proven merely because it was created. Recovery is proven when the required recovery point exists, remains intact and accessible, the required credentials are available, and the protected information can be successfully restored and validated.**

## Native Product Boundary

GoreeCloud Backups is intended to be an original GoreeCloud application and backup engine. Kopia, Restic, Rclone, and other third-party projects may influence engineering decisions or support optional interoperability, migration, transport, protocol, library, or compatibility mechanisms, but they do not define the native product identity, repository format, or backup engine.

## Current Status

- Lifecycle: Concept / planning baseline.
- Application implementation: Not yet verified.
- Release status: Unreleased.
- Stable status: Not established.
- Production deployment: Not established.
- Platform Contract conformance: Nonconformant until implementation and acceptance evidence exists.

## Repository Documentation

- [SPECIFICATIONS.md](./SPECIFICATIONS.md) — current repository-coupled product specification.
- [PROJECT-SPECIFICATIONS.md](./PROJECT-SPECIFICATIONS.md) — project-level specification and governance record.
- [PROJECT-RECORD.md](./PROJECT-RECORD.md) — verified project history and decisions.
- [FEATURES.md](./FEATURES.md) — current verified functionality summary.
- [IMPLEMENTED-FEATURES.md](./IMPLEMENTED-FEATURES.md) — verified implemented features only.
- [PLANNED-FEATURES.md](./PLANNED-FEATURES.md) — accepted planned capability scope.
- [BENEFITS.md](./BENEFITS.md) — intended benefits with evidence boundaries.
- [COMPETITIVE-OBJECTIVES.md](./COMPETITIVE-OBJECTIVES.md) — benchmark set and differentiation objectives.
- [BRANDING.md](./BRANDING.md) — product branding authority and current branding status.
- [USER-MANUAL.md](./USER-MANUAL.md) — current user-facing availability and future manual structure.
- [PRIVACY POLICY.md](./PRIVACY%20POLICY.md) — repository-level privacy requirements and current-state disclosure.
- [SECURITY.md](./SECURITY.md) — security boundaries and reporting guidance.
- [NOTES.md](./NOTES.md) — repository-level operational and planning notes.
- [CHANGELOGS.md](./CHANGELOGS.md) — repository change history.
- [goreecloud.platform.yaml](./goreecloud.platform.yaml) — GoreeCloud Platform Contract declaration.

## Development Status Integrity

Planned documentation must not be interpreted as implementation evidence. A capability moves into implemented-state documentation only after the authoritative repository state and required validation evidence demonstrate that the capability actually exists.

## Licensing

No repository license or approved rights notice has yet been verified for this repository. No license grant should be inferred merely from public repository visibility. Licensing must be resolved before distribution or reuse claims are made.
