# GoreeCloud Backups — Implemented Features

**Document Type:** Repository-Native Implemented Feature Record  
**Status:** Active  
**Project:** GoreeCloud Backups  
**Authority:** Verified implementation lifecycle record  
**Last Updated:** 2026-09-24

## Product Functionality

No GoreeCloud Backups backup or restore product functionality is currently verified as implemented.

The Milestone 0 source foundation does not provide usable backup, repository persistence, restore, scheduling, storage-provider, agent, server, or graphical functionality.

## Development Engineering Foundation

Draft Pull Request #5 contains the first native Development source foundation:

- Rust workspace pinned to Rust 1.98.1.
- `goreecloud-backups-core` crate.
- Recovery-first evidence model that requires recovery-point existence, repository accessibility, repository-integrity verification, recovery-credential availability, successful restore, and restored-data validation before reporting restore verification.
- `goreecloud-backups-repository` crate.
- Fail-closed repository persistence gate that rejects persistence until the initial native repository format is defined and accepted.
- `goreecloud-backups-cli` crate.
- Development status CLI that reports backup and restore engines as not implemented and Stable eligibility as false.
- Pinned repository CI candidate for formatting, metadata, linting, tests, build, and CLI truth checks.

This foundation remains a Development candidate until exact-head validation and governed integration are complete.

## Promotion Rule

A product capability may be added as implemented only after its implementation is verified against authoritative repository state and any required tests, recovery evidence, security controls, target-runtime evidence, and acceptance gates.

Source scaffolding or passing source-level CI does not establish recoverability, release readiness, deployment, production acceptance, or Stable status.

For planned scope, see [PLANNED-FEATURES.md](./PLANNED-FEATURES.md).
