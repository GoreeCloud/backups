# GoreeCloud Backups — Native Repository Format V1

**Status:** Proposed for Development acceptance  
**Format:** GoreeCloud Backups Native Repository Format  
**Format Version:** 1.0  
**Repository Profile ID:** `goreecloud-backups/repository-v1`  
**Persistence State:** Blocked until this specification and its implementation are separately validated  
**Primary Goal:** Portable, independently recoverable, encrypted, corruption-detecting, storage-provider-neutral recovery repositories

## 1. Governing Principle

A GoreeCloud Backups repository is a recovery artifact, not merely an implementation detail.

The repository format must remain understandable and recoverable without the original graphical client, central control plane, agent database, or deployment environment. A healthy repository must be recoverable in a clean environment with compatible open-source recovery tooling and the required recovery credentials.

The format must prefer preservation and explicit failure over silent repair, implicit migration, destructive rewriting, or optimistic assumptions.

## 2. V1 Design Goals

Repository Format V1 is designed to provide:

- Client-side authenticated encryption.
- Repository-local deduplication without exposing raw plaintext content hashes in object names.
- Compression before encryption.
- Immutable data objects.
- Snapshot commit atomicity.
- Rebuildable derived indexes.
- Pack-level storage efficiency and range-retrieval support.
- Clean-environment recovery.
- Multiple independently stored recovery credential slots.
- Explicit format/version compatibility.
- Corruption detection and fail-closed parsing.
- Local-filesystem and object-storage compatibility.
- Safe interrupted-operation behavior.
- Future migration without making one storage provider authoritative.

## 3. Non-Goals

V1 does not make these concerns part of the durable repository format:

- A central control-plane database.
- User-interface state.
- Scheduler state.
- Agent enrollment state.
- GoreeCloud Manager state.
- Temporary locks.
- Local caches.
- Search indexes that cannot be rebuilt.
- Cloud-provider-specific metadata.
- Plaintext filenames, directory names, labels, comments, or application metadata.
- Raw plaintext content hashes as storage object names.

Operational systems may maintain these separately, but losing them must not make an otherwise healthy repository unrecoverable.

## 4. Repository Identity

Every repository has:

- A randomly generated 128-bit repository identifier.
- Format major version `1`.
- Format minor version `0`.
- Profile identifier `goreecloud-backups/repository-v1`.
- A randomly generated 256-bit repository master key.
- One or more recovery credential/key slots that wrap the repository master key.

Repository identifiers are not secrets. Repository master keys are secrets.

## 5. Top-Level Layout

A V1 repository uses the following logical layout:

```text
repository.json
keys/
  <slot-id>.gck
packs/
  <prefix>/
    <pack-id>.gcp
indexes/
  <generation>/
    <index-id>.gci
snapshots/
  <snapshot-id>.gcs
evidence/
  <evidence-id>.gce
```

Storage adapters may map these logical paths onto local filesystems, S3-compatible object stores, WebDAV-like stores, or future approved backends, but the logical identity and object semantics must remain equivalent.

Temporary/staging objects are backend-local implementation details and are not durable format authority.

## 6. Repository Descriptor

`repository.json` is a small immutable plaintext bootstrap descriptor created once.

It contains only information required to identify and open the repository:

- Repository identifier.
- Format major/minor version.
- Profile identifier.
- Creation software identifier.
- Required format feature identifiers, if any.

It must not contain protected filenames, source paths, snapshot descriptions, user labels, secrets, master keys, or reusable credentials.

The exact descriptor bytes are hashed with SHA-256. Every key slot binds the descriptor hash into its authenticated data and protected payload. Readers must reject a descriptor/key-slot mismatch.

The descriptor is immutable in V1. New capabilities that require incompatible descriptor changes require a governed format evolution rather than silent in-place mutation.

## 7. Structured Metadata Encoding

Encrypted structured repository metadata uses deterministic CBOR compatible with RFC 8949 deterministic-encoding requirements.

Deterministic encoding is required where metadata participates in content identifiers or compatibility validation.

Implementations must reject:

- Duplicate map keys.
- Non-canonical encodings where canonical encoding is required.
- Unknown required fields.
- Integer overflows.
- Excessive nesting or allocation requests.
- Length claims that exceed configured safety bounds.
- Trailing data where the object schema does not permit it.

Parsers must be fuzzable and must treat malformed repository input as untrusted.

## 8. Cryptographic Profile

Repository Format V1 defines the following initial cryptographic profile:

- Password/key-slot KDF: Argon2id.
- Key derivation/domain separation: HKDF-SHA-256.
- Keyed content identifiers: HMAC-SHA-256.
- Authenticated encryption: AES-256-GCM-SIV.
- Descriptor hashing: SHA-256.
- Repository master key: 256 random bits.

These are format interoperability choices, not custom cryptographic primitives.

Implementations must use maintained, externally reviewed cryptographic libraries. GoreeCloud Backups must not implement these primitives itself.

### 8.1 Password Key Slots

The required V1 recovery slot type is a password-protected key slot.

A password slot stores:

- Slot identifier.
- Repository identifier.
- Descriptor SHA-256 digest.
- Argon2id salt.
- Argon2id memory, iteration, and parallelism parameters.
- AES-256-GCM-SIV nonce.
- Wrapped repository-master-key ciphertext and authentication tag.
- Key-slot schema version.

Argon2id parameters are stored with the slot so future recovery does not depend on contemporary defaults. Creation policy may strengthen parameters over time without changing the repository format.

### 8.2 Key Separation

The repository master key is not used directly to encrypt repository objects.

Subkeys are derived with HKDF-SHA-256 using the repository identifier as salt and explicit ASCII domain labels.

Required domains include at least:

- `goreecloud-backups/v1/content-id`
- `goreecloud-backups/v1/pack`
- `goreecloud-backups/v1/snapshot`
- `goreecloud-backups/v1/index`
- `goreecloud-backups/v1/evidence`

Object-specific identifiers are included in derivation context where applicable.

### 8.3 Content Identifiers

Deduplicated content identifiers are:

```text
HMAC-SHA-256(content-id-key, object-type || canonical-plaintext)
```

The keyed identifier prevents the repository namespace from exposing raw plaintext hashes that could otherwise aid content-guessing attacks.

Deduplication is repository-local by default. Cross-user or cross-repository deduplication must not be inferred from V1.

## 9. Generic Encrypted Object Envelope

Every encrypted durable object uses an authenticated envelope containing a minimal plaintext header and encrypted payload.

The plaintext header includes:

- Object magic/type.
- Format major/minor version.
- Repository identifier.
- Object identifier.
- Cryptographic profile identifier.
- Encrypted payload length.
- Nonce where the object profile requires one.

The complete encoded header is authenticated as AEAD associated data.

Readers must reject:

- Wrong repository identifiers.
- Unsupported major versions.
- Unknown required profiles.
- Object-type mismatches.
- Length mismatches.
- Authentication failures.
- Truncated objects.
- Trailing data that violates the object schema.

Object headers must not contain protected path names or user metadata.

## 10. Packs

Pack objects use extension `.gcp`.

A pack is immutable after publication.

A pack contains encrypted records for:

- File-content chunks.
- File manifests.
- Directory/tree metadata.
- Other content-addressed repository metadata approved by V1.

Each pack has a random object identifier.

### 10.1 Record Encryption

Each pack record is independently authenticated so a reader can validate and retrieve bounded ranges without decrypting an entire pack.

A pack-specific encryption context is derived from the repository master key and pack identifier. Record nonces must be unique within that pack encryption context.

Implementations must derive or allocate nonces in a way that prevents reuse for the same encryption key. The initial implementation should prefer deterministic nonce derivation from a pack-specific nonce key and monotonically assigned record ordinal, rather than relying on collision-prone random nonce generation at very large record counts.

### 10.2 Pack Table of Contents

Each pack ends with an encrypted and authenticated table of contents.

The table of contents records, for each record:

- Keyed content identifier.
- Record type.
- Byte offset.
- Encrypted length.
- Plaintext logical length.
- Compression profile.
- Record ordinal.

A fixed-size authenticated trailer locates the encrypted table of contents from the end of the pack.

The pack table of contents allows derived indexes to be rebuilt by scanning packs after index loss.

## 11. Compression

Compression occurs before encryption.

V1 permits:

- `none`
- `zstd`

The compression profile is recorded per stored record so later readers do not depend on current backup-policy defaults.

Implementations must enforce decompression output limits and must reject compression bombs or declared sizes that violate safety policy.

Already-compressed or incompressible data may use `none`.

## 12. File and Directory Metadata

Repository metadata must preserve recoverability without assuming every target platform has identical filesystem semantics.

### 12.1 Paths

Paths are stored as ordered path segments, not as a slash-concatenated string.

A segment retains its source-platform encoding:

- Unix-like source: raw filename bytes.
- Windows source: UTF-16 code units encoded in a defined little-endian binary representation.

A UTF-8 display form may be stored when lossless, but the display form is not authoritative for exact same-platform restoration.

Cross-platform restore must explicitly map unsupported names and report collisions or lossy substitutions. It must not silently overwrite another restored path.

### 12.2 Metadata

V1 metadata may preserve, where available and allowed:

- File type.
- Logical size.
- Sparse extents.
- Modification/access/change/birth timestamps with precision metadata.
- POSIX mode.
- UID/GID.
- Windows attributes.
- ACL/security-descriptor data.
- Extended attributes.
- Symbolic-link target.
- Hard-link identity.
- Application-provided metadata extensions.

Unsupported metadata must be surfaced during restore planning rather than silently treated as restored.

## 13. Chunk and File Representation

Chunking policy is not hard-coded into the durable format.

A file manifest records the ordered logical content sequence required to reconstruct a file, including:

- Content-chunk identifiers.
- Logical chunk lengths.
- Sparse-hole extents where applicable.
- File logical length.

The snapshot may record the chunking profile used for future backup continuity and diagnostics, but a restore does not need the original chunking algorithm.

A chunk's keyed content identifier is calculated over its canonical plaintext before compression and encryption.

## 14. Snapshot Manifests

Snapshot objects use extension `.gcs`.

A snapshot manifest is immutable and encrypted.

It includes at least:

- Snapshot identifier.
- Repository identifier.
- Creation timestamp.
- Source identity and platform metadata.
- Root tree/content identifier.
- Parent snapshot identifier when applicable.
- Backup policy/profile identifiers where available.
- Required format features.
- Application-consistency metadata.
- Software version that created the snapshot.
- Optional references to recovery/verification evidence.

Protected path and source metadata remains encrypted.

## 15. Snapshot Atomicity

A snapshot is committed by writing its snapshot manifest **last**.

The writer must:

1. Produce required content records and packs.
2. Durably publish the packs.
3. Publish any derived index objects.
4. Verify required object availability according to backend semantics.
5. Publish the immutable snapshot manifest last.

If the operation fails before the snapshot manifest is published, the snapshot is not committed.

Unreferenced packs or index objects from interrupted operations are orphan candidates and may be reclaimed only after a safe grace period and reachability analysis.

This design avoids making a mutable central database the commit authority.

## 16. Indexes

Index objects use extension `.gci`.

Indexes accelerate lookup of keyed content identifiers to pack records.

Indexes are:

- Encrypted and authenticated.
- Immutable after publication.
- Generation identified.
- Rebuildable from pack tables of contents.

Indexes are derived recovery aids, not the sole authority for repository contents.

Loss or corruption of every index must not make intact packs and snapshots permanently unrecoverable.

## 17. Recovery Evidence Objects

Recovery evidence objects use extension `.gce`.

Evidence objects may record privacy-minimized technical proof such as:

- Repository verification results.
- Restore-test identifiers.
- Snapshot identifiers tested.
- Validation method identifiers.
- Software/tool version.
- Result state.
- Failure categories.
- Non-sensitive timing metadata.

Evidence objects must not become a substitute for the restored information itself and must avoid embedding unnecessary protected content.

A backup job completion event is not equivalent to a recovery evidence object showing a validated restore.

## 18. Clean-Environment Recovery

A compatible recovery CLI must be able to recover an intact repository using only:

- Repository storage access.
- `repository.json`.
- At least one usable key slot/recovery credential.
- The repository objects themselves.
- The open V1 format specification.

The recovery path must not require:

- The original GoreeCloud Manager instance.
- The original GoreeCloud Backups server database.
- The original agent database.
- Original scheduler state.
- A graphical interface.

If derived indexes are unavailable, recovery tooling must be able to rebuild required lookup state by scanning packs and validating pack tables of contents.

## 19. Retention and Garbage Collection

Snapshot retention is expressed by preserving or removing snapshot manifests according to policy.

Garbage collection determines reachability from every retained snapshot and any other protected roots.

A pack containing both live and unreachable records must not be deleted merely because some records are unreachable. Safe compaction requires:

1. Writing replacement packs containing live records.
2. Validating replacement packs.
3. Publishing replacement indexes where used.
4. Revalidating retained snapshot reachability.
5. Waiting any required repository/backend grace period.
6. Deleting old packs only after no retained root requires them.

Interrupted garbage collection must preserve previously recoverable repository state.

## 20. Backend Publication Requirements

A storage backend must provide a publication strategy with whole-object integrity.

For local filesystems, publication should use:

- A unique temporary file in the destination filesystem.
- Complete write.
- File sync where supported/required.
- Atomic rename into the final immutable name.
- Parent-directory sync where required for durability.

For object stores, durable objects use unique immutable keys and must not be treated as published until the backend acknowledges the complete object.

Snapshot manifests remain the final commit record.

Backend adapters must document weaker consistency or durability semantics and compensate where required.

## 21. Locking and Concurrency

Locks are operational coordination, not durable recovery authority.

Writers must use repository-scoped coordination to prevent unsafe concurrent maintenance or conflicting object publication.

A stale or lost lock must not make the repository unrecoverable.

Immutable object IDs and snapshot-last commit behavior must keep interrupted or concurrent operations fail-safe.

## 22. Integrity and Verification Levels

V1 distinguishes:

1. **Structural verification** — expected object/header/layout structure is parseable.
2. **Cryptographic verification** — AEAD authentication succeeds.
3. **Content verification** — decrypted canonical plaintext matches its keyed content identifier.
4. **Snapshot verification** — every object reachable from a snapshot is present and valid.
5. **Restore verification** — data is restored into an isolated target.
6. **Validation verification** — restored data/application behavior satisfies the required validation method.

Only the final levels establish recovery evidence appropriate to the protected workload.

## 23. Format Evolution

Readers and writers must treat version compatibility explicitly.

- Unknown major version: fail closed.
- Higher minor version with unknown required features: fail closed.
- Higher minor version using only known required features and safely ignorable optional features: may be accepted according to reader policy.
- Writers must not silently rewrite an old repository into a newer incompatible major format.
- Migrations must create verified new state while retaining an approved rollback/recovery path until migration validation completes.

Every durable object records enough format/profile information for independent validation.

## 24. Security Requirements

Implementations must:

- Treat all repository bytes as attacker-controlled until authenticated.
- Bound allocations and recursion.
- Validate all lengths and offsets before use.
- Prevent integer overflow.
- Prevent path traversal and absolute-path restore escape.
- Refuse restore paths outside the authorized target root.
- Prevent symlink/hard-link restoration from escaping the target root.
- Require explicit authorization for destructive operations.
- Zeroize key material where the selected language/library permits reliable handling.
- Avoid logging keys, passwords, protected filenames, or decrypted metadata unnecessarily.
- Fuzz parsers and object decoders.
- Maintain test corpora containing corrupted, truncated, reordered, duplicated, oversized, and malicious objects.

## 25. Privacy Requirements

The repository's plaintext surface should reveal only what storage operation requires.

Plaintext storage names may reveal:

- That a GoreeCloud Backups repository exists.
- Repository identifier.
- Approximate object counts.
- Object sizes.
- Object categories from extensions.
- Timing information exposed by the storage backend.

Protected user filenames, paths, memo text, labels, application data, file contents, and snapshot descriptions remain encrypted.

Future optional privacy modes may reduce metadata leakage further without breaking V1 recovery semantics.

## 26. Persistence Gate

Repository persistence remains blocked until implementation acceptance.

The current Rust repository crate must continue to fail closed.

Before persistence can be enabled, the implementation must demonstrate at minimum:

- Exact V1 descriptor/key-slot encoding fixtures.
- Deterministic CBOR fixtures.
- Cryptographic known-answer/interoperability tests.
- Malformed-input and fuzz coverage.
- Pack round-trip tests.
- Pack table-of-contents rebuild tests.
- Snapshot-last interruption tests.
- Index-loss rebuild tests.
- Wrong-password/wrong-key failure tests.
- Descriptor tamper tests.
- Object-header tamper tests.
- Truncation/corruption tests.
- Path traversal restore defenses.
- Cross-platform path metadata fixtures.
- Clean-environment repository-open tests.

No design document alone authorizes persistent repository writes.

## 27. Recovery-First Rule

Repository Format V1 exists to preserve recovery capability.

The format is acceptable only if a future clean implementation can open, verify, scan, rebuild indexes, enumerate snapshots, restore selected data, and validate integrity without relying on the original running GoreeCloud environment.

**A repository format is not complete because it can write backup data. It is complete when independently held recovery credentials and compatible recovery tooling can reliably read and restore that data.**
