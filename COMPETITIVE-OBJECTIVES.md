# GoreeCloud Backups — Competitive Objectives

**Status:** Active competitive-planning record  
**Last Reviewed:** 2026-09-24

## Product Purpose

GoreeCloud Backups is intended to be a native GoreeCloud backup, restoration, verification, and recovery platform centered on evidence-backed recoverability, self-hosting, privacy, portability, and independent recovery.

See [SPECIFICATIONS.md](./SPECIFICATIONS.md) and [PLANNED-FEATURES.md](./PLANNED-FEATURES.md) for authoritative product scope.

## Initial Benchmark Set

The initial benchmark set includes products and projects whose engineering, workflows, or market role may provide useful comparison points:

- Restic.
- Kopia.
- BorgBackup.
- Duplicati.
- Proxmox Backup Server.
- Veeam.
- Rclone as an adjacent storage-transport and interoperability reference rather than a native Backups engine.

This is an initial benchmark set, not a permanent or exhaustive market analysis. Detailed comparative claims must be revalidated before they are treated as current facts.

## Competitive Objectives

GoreeCloud Backups should:

- Make restore verification a first-class product workflow.
- Keep recovery possible without dependence on the original control plane.
- Provide native cross-platform repository recovery through a documented repository format.
- Combine deduplication, compression, encryption, and remote-storage efficiency without sacrificing recoverability.
- Provide evidence-backed protection states that do not overstate backup safety.
- Make destructive-event resistance an architectural property.
- Support application-consistent Protection Profiles maintained by GoreeCloud applications where appropriate.
- Keep basic backup and restore usable without a mandatory centralized service.
- Preserve self-hosting, portability, privacy, and storage-provider independence.
- Support migration from external backup ecosystems without inheriting their product identity.
- Provide strong CLI and API automation alongside a native Glaze UI.
- Minimize protected-content exposure in administration, monitoring, notification, diagnostics, and observability.
- Treat recovery-credential availability as part of recovery readiness.

## Differentiation

The intended differentiation is the combination of recovery-first semantics, evidence-backed restore verification, self-hosting, user-controlled storage, open repository evolution, provider independence, privacy-conscious administration, clean-environment recovery, and explicit GoreeCloud platform authority boundaries.

## Review Requirement

Competitive objectives must evolve as the backup ecosystem changes. Future detailed comparisons should record their review date and distinguish verified current facts from design lessons or product objectives.
