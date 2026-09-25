# GoreeCloud Backups — Technology Decision

**Status:** Accepted for Milestone 0 Development  
**Project:** GoreeCloud Backups  
**Decision Scope:** Initial native engine, repository, and CLI foundation  
**Rust Toolchain:** 1.98.1  
**License:** AGPL-3.0-or-later

## Decision

GoreeCloud Backups will use **Rust** as the primary implementation language for the native backup engine, repository-format implementation, restore engine, verification logic, command-line interface, and other security-sensitive or performance-critical core components.

The initial source foundation pins Rust 1.98.1.

This decision applies to the core product implementation. It does not require every future GoreeCloud Backups client or presentation surface to use Rust.

## Rationale

GoreeCloud Backups handles information whose integrity and recoverability are critical. Its core implementation will eventually process untrusted and malformed repository data, large file streams, cryptographic material, compression streams, content chunks, filesystem metadata, remote storage responses, interrupted operations, and destructive maintenance workflows.

Rust is selected because it provides:

- Memory safety without a garbage collector.
- Strong type and ownership systems suited to repository and recovery invariants.
- Predictable native performance for content-defined chunking, hashing, compression, encryption integration, pack processing, verification, and restore operations.
- Cross-platform native binaries suitable for CLI, agent, server, and recovery tooling.
- Explicit error handling and strong testability.
- A suitable boundary for security-sensitive parsers and repository-format code.
- A practical path to reusable libraries shared by command-line, agent, server, and future native integrations.

This matches GoreeCloud's portfolio guidance that Rust is preferred for memory-safe systems, security-sensitive components, parsers, networking, and performance-critical components.

## Initial Architecture

Milestone 0 uses one Rust workspace with deliberately small boundaries:

- `goreecloud-backups-core` — product-domain invariants and recovery-evidence semantics.
- `goreecloud-backups-repository` — repository-format boundary and fail-closed persistence gate.
- `goreecloud-backups-cli` — command-line process foundation.

The repository-format crate intentionally does **not** write persistent repository data in Milestone 0. Persistence remains blocked until the initial repository-format specification and compatibility rules are accepted.

## Dependency Rule

Milestone 0 begins with no third-party Rust runtime dependencies.

Future dependencies for cryptography, hashing, compression, content-defined chunking, storage protocols, serialization, databases, networking, or platform integration must be selected individually for:

- Security and maintenance status.
- License compatibility.
- Supply-chain provenance.
- Portability.
- Performance.
- API stability.
- Long-term project health.

GoreeCloud Backups will not implement custom cryptographic primitives or novel compression algorithms.

## User Interface and Client Languages

Core-engine language does not dictate presentation technology.

Expected future boundaries are:

- Web/administrative Glaze UI: TypeScript where a browser surface is appropriate.
- Android: Kotlin for native Android integration where a first-class Android client is justified.
- Apple platforms: Swift where first-class Apple clients are justified.
- Linux desktop: implementation language and toolkit remain a separate client decision; the core Rust libraries and CLI may be reused where technically appropriate.

Every graphical surface must target the current consumer-eligible Stable Glaze UI release at the time of implementation and complete application-specific acceptance.

## Change Control

This technology decision may be revised if implementation evidence demonstrates that another language or split architecture materially improves security, portability, maintainability, interoperability, or recovery reliability.

A future change must preserve repository-format compatibility, recovery tooling, user-owned data portability, documented migration, and the ability to recover repositories without dependence on one graphical client or control plane.
