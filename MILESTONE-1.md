# GoreeCloud Backups — Milestone 1

**Lifecycle:** Development  
**Milestone:** Native Repository Format V1  
**Status:** Design candidate  
**Persistence Boundary:** Persistent repository writes remain blocked

## Objective

Define and accept the first native GoreeCloud Backups repository format before implementing durable repository writes.

## Design Scope

Milestone 1 defines:

- Repository identity and versioning.
- Storage-provider-neutral logical layout.
- Minimal plaintext bootstrap metadata.
- Key-slot and master-key model.
- Cryptographic profile.
- Key separation.
- Keyed repository-local content identifiers.
- Immutable packs and independently authenticated records.
- Deterministic structured metadata encoding.
- File/tree/snapshot representation.
- Snapshot-last atomic commit semantics.
- Rebuildable indexes.
- Recovery evidence objects.
- Retention and safe garbage-collection rules.
- Clean-environment recovery.
- Format evolution and compatibility.
- Parser, path, corruption, and privacy requirements.

The proposed format is defined in [REPOSITORY-FORMAT.md](./REPOSITORY-FORMAT.md).

## Implementation Boundary

This milestone does not itself authorize repository persistence.

The current Rust persistence gate remains closed until the format implementation has passed fixtures, cryptographic interoperability, parser/fuzz, pack, corruption, snapshot-interruption, index-rebuild, path-safety, and clean-recovery tests.

## Initial Implementation Sequence

After format design acceptance, implementation should proceed in bounded steps:

1. Define versioned descriptor/key-slot schemas and binary/CBOR fixtures.
2. Implement strict decoding before writing.
3. Implement key derivation/wrapping with known-answer tests.
4. Implement pack record encoding/decoding in memory.
5. Implement encrypted pack table-of-contents recovery.
6. Implement disposable/rebuildable indexes.
7. Implement snapshot-manifest decoding/encoding.
8. Implement a temporary local test repository adapter.
9. Prove snapshot-last interruption safety.
10. Prove clean-environment open, index rebuild, and restore of controlled fixtures.
11. Only then consider enabling persistent repository creation for Development testing.

## Acceptance Boundary

Milestone 1 design acceptance means the repository format is sufficiently specified to implement against.

It does not mean:

- Persistence implementation is complete.
- Cryptography implementation is accepted.
- Backups can protect user data.
- Restore is usable.
- Any storage provider is supported.
- Production repositories may be created.
- Stable compatibility is guaranteed.

Those require later implementation and recovery evidence.
