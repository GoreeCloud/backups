# GoreeCloud Backups — Milestone 0

**Lifecycle:** Development  
**Milestone:** Native Source Foundation  
**Milestone Status:** Integrated / source-level acceptance complete  
**Implementation Boundary:** Engineering foundation only; no backup or restore capability is accepted

## Objective

Establish the smallest trustworthy native source foundation from which the GoreeCloud Backups engine can be developed without manufacturing backup functionality or committing prematurely to an unreviewed repository format.

## Included

Milestone 0 establishes:

- A pinned Rust workspace and toolchain.
- Core recovery-evidence semantics that encode the recovery-first product principle.
- A separate repository-format crate boundary.
- A fail-closed gate that prevents repository persistence before the initial format is defined and accepted.
- A minimal command-line executable that reports Development foundation state.
- Automated formatting, linting, tests, build validation, and CLI smoke validation.
- Documentation of the technology decision and implementation boundary.

## Explicitly Not Implemented

Milestone 0 does not implement or claim:

- File or directory backup.
- Snapshot creation.
- Content-defined chunking.
- Deduplication.
- Compression.
- Encryption or key management.
- Repository creation or persistent repository writes.
- Pack files or indexes.
- Retention or garbage collection.
- Restore operations.
- Repository verification.
- Restore testing.
- Storage-provider access.
- Agents or repository servers.
- Scheduling.
- User-facing Glaze UI.
- Platform-system runtime integration.
- Production deployment or release readiness.

## Recovery-First Invariant

A restore-verified state requires evidence that:

- A recovery point exists.
- The repository is accessible.
- Repository integrity has been verified.
- Required recovery credentials are available.
- A restore succeeded.
- The restored information was validated.

Backup-job success alone is never sufficient.

## Acceptance

Milestone 0 is accepted as a Development source foundation only when:

- The pinned toolchain installs successfully in CI.
- `cargo fmt --all -- --check` passes.
- `cargo clippy --workspace --all-targets -- -D warnings` passes.
- `cargo test --workspace --locked` passes.
- `cargo build --workspace --locked` passes.
- The CLI Development-status smoke check passes.
- Repository persistence remains fail-closed.
- Documentation continues to state that no actual backup or restore functionality exists.

Passing these checks does not establish release, deployment, production, recovery, Glaze UI, or Stable acceptance.

## Verification Evidence

- Pull Request: #5.
- Exact validated PR head: `23d26fa2cfdd60a3227c60621e9defc439736751`.
- Exact-head Rust Foundation run: `36096359838` — passed.
- Guarded squash merge: `b997478c447d11b902cca1ff82633ad370089ede`.
- Exact merged-main Rust Foundation push run: `36096435671` — passed.
- Repository persistence remained fail-closed throughout validation.

This evidence establishes Milestone 0 as an integrated Development engineering foundation only. It does not establish usable backup or restore capability, runtime recovery acceptance, deployment, production acceptance, or Stable eligibility.

## Next Milestone

The next engineering milestone should define and review the first versioned native repository-format specification before any code is allowed to persist recovery data. That work should define format identity, version negotiation, metadata/authentication boundaries, compatibility rules, corruption behavior, atomicity requirements, and clean-environment recovery expectations before storage code is enabled.
