# GoreeCloud Backups — Competitive Objectives

**Status:** Active competitive-planning record  
**Last Reviewed:** 2026-09-24

## Product Purpose

GoreeCloud Backups is intended to be a native GoreeCloud backup, restoration, verification, and recovery platform centered on evidence-backed recoverability, self-hosting, privacy, portability, and independent recovery.

See [SPECIFICATIONS.md](./SPECIFICATIONS.md) and [PLANNED-FEATURES.md](./PLANNED-FEATURES.md) for authoritative product scope.

## Initial Benchmark Set

The initial benchmark set includes projects and products whose engineering, workflows, or market role may provide useful comparison points:

- Restic — open-source backup repository and restore workflows.
- Kopia — snapshot, repository, deduplication, encryption, and maintenance concepts.
- BorgBackup — deduplicating archival and backup workflows.
- Duplicati — user-facing scheduled encrypted backup workflows and broad storage support.
- Proxmox Backup Server — managed backup, verification, retention, and infrastructure recovery workflows.
- Veeam — enterprise backup, recovery, verification, management, and operational workflows.
- Rclone — adjacent storage-transport and provider interoperability rather than a native Backups engine.

This list is an initial benchmark set, not a permanent or exhaustive market analysis. Detailed competitor claims must be revalidated before they are treated as current facts.

## Competitive Objectives

GoreeCloud Backups should pursue the following outcomes:

- Make restore verification a first-class product workflow rather than an optional afterthought.
- Keep recovery possible without dependence on the original control plane.
- Provide native cross-platform repository recovery through a documented repository format.
- Combine strong deduplication, compression, encryption, and remote-storage efficiency without sacrificing recoverability.
- Provide clear evidence-backed protection states that do not overstate backup safety.
- Make destructive-event and ransomware resistance an architectural property rather than only an operational recommendation.
- Support application-consistent protection profiles that can be maintained by GoreeCloud applications themselves.
- Keep core backup and restore operation usable without a mandatory centralized service.
- Preserve self-hosting, portability, privacy, and storage-provider independence.
- Support optional migration from external backup ecosystems without inheriting their product identity.
- Provide strong CLI and API automation alongside a native Glaze UI.
- Minimize protected-content exposure in administration, monitoring, notification, diagnostics, and observability.
- Make recovery credentials part of health and readiness instead of treating repository bytes alone as sufficient.

## Differentiation

The intended GoreeCloud differentiation is not feature copying. It is the combination of:

- Recovery-first product semantics.
- Evidence-backed restore verification.
- Self-hosting and user-controlled storage.
- Open repository evolution and recovery documentation.
- Independence from one cloud or storage provider.
- Privacy-conscious administration.
- Clean-environment recovery.
- Strong integration with GoreeCloud continuity, security, privacy, policy, observability, and management systems while keeping authority boundaries explicit.

## Review Requirement

Competitive objectives must evolve as the backup ecosystem changes. Any future detailed product comparison should record its review date and distinguish verified current facts from design lessons or product objectives.
