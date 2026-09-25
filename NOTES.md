# GoreeCloud Backups — Repository Notes

**Document Type:** Repository-Native Notes  
**Status:** Active  
**Project:** GoreeCloud Backups  
**Last Updated:** 2026-09-24

## Current Repository State

- The repository is in specification and documentation bootstrap state.
- No GoreeCloud Backups application functionality is currently verified as implemented.
- PLANNED-FEATURES.md contains the accepted planned capability set.
- IMPLEMENTED-FEATURES.md remains the authority for verified implementation state.
- PROJECT-SPECIFICATIONS.md and PROJECT-RECORD.md are the authoritative project specification and history records.
- SPECIFICATIONS.md provides the repository-coupled implementation specification.
- FEATURES.md summarizes current functionality and lifecycle state.

## Current Product Boundary

GoreeCloud Backups is planned as an original GoreeCloud backup and recovery product, not a fork of Kopia, Restic, Rclone, or another backup application.

External backup products may be supported through migration, interoperability, transport, storage access, libraries, protocols, or engineering reference.

## Documentation Integrity

Planned capabilities must remain clearly separated from implemented capabilities.

Documentation-only changes must not be used as evidence for runtime behavior, release readiness, production acceptance, or Stable status.

## Repository Baseline Status

The repository documentation/governance baseline is established.

- Governed `AGPL-3.0-or-later` fallback licensing is recorded in `LICENSE` and `LICENSE-DECISION.md`.
- The approved GoreeCloud Backups product icon is established in `GoreeCloud/branding-assets` at `products/backups/app-icon.svg`, verified blob `7a73f739c43ca35ad043b68cae632e84a8a68218`.
- Privacy, user-manual, security, branding, and Platform Contract baseline records exist on main.
- Lifecycle is Development, but no application functionality is yet verified as implemented.

Remaining work is product implementation and evidence-backed acceptance rather than missing repository-baseline documentation.
