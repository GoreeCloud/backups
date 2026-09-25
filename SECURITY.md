# GoreeCloud Backups — Security

**Status:** Pre-implementation security guidance  
**Last Updated:** 2026-09-24

## Security State

No production GoreeCloud Backups implementation is currently verified in this repository.

The security controls described in planned documentation are requirements and design objectives, not evidence of implemented protection.

## Security Principles

GoreeCloud Backups is expected to apply:

- Least privilege.
- Client-side authenticated encryption where appropriate.
- Separation of backup-writing, maintenance, recovery, and repository-administration authority.
- Restricted destructive permissions.
- Recovery credentials independent from the original machine.
- Strong confirmation for destructive operations.
- Immutable-storage or object-lock support where available.
- Safe failure for interrupted backup, restore, retention, garbage collection, and migration.
- Minimal secret exposure in logs, metrics, notifications, and diagnostics.
- Explicit authorization for restore and repository-administration operations.
- Privacy-preserving audit records.

## Cryptography

GoreeCloud Backups must use established and maintained cryptographic algorithms and implementations.

The project must not invent custom cryptographic primitives.

## Secret Handling

Do not commit:

- Passwords.
- API tokens.
- Private keys.
- Recovery keys.
- Repository passwords.
- Encryption master keys.
- Production credentials.
- Secret-bearing environment files.
- Sensitive backup contents.
- Private user data.

Use sanitized examples and documented secret references instead.

## Vulnerability Reporting

Do not publish sensitive vulnerability details, exploit material, credentials, or private recovery information in public issues.

Use an approved private GoreeCloud security-reporting channel when reporting a sensitive vulnerability. This repository must not claim a specific reporting mechanism until that mechanism has been verified as available.

## Release Security Gate

Security claims require implementation and evidence. Stable or production recovery claims must not be made until applicable security, privacy, restore, and recovery validation has been completed.
