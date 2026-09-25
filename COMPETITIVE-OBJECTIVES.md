# GoreeCloud Backups — Competitive Objectives

**Document Type:** Repository-Native Competitive Objectives Record  
**Status:** Planned objectives  
**Project:** GoreeCloud Backups  
**Last Updated:** 2026-09-24

> This document defines design and differentiation objectives. It does not claim that GoreeCloud Backups currently matches or exceeds any named product.

## 1. Native Recovery-First Architecture

Design GoreeCloud Backups around demonstrated recoverability, not only backup-job completion.

The product should make repository integrity, recovery credentials, restore testing, validation, and recovery evidence visible as distinct states.

## 2. Open and Portable Repository Control

Maintain an open, documented, versioned native repository format that can be recovered without a proprietary hosted control plane or the original installation.

Repository evolution should prioritize backward-readable recovery paths, controlled migration, and long-term portability.

## 3. First-Party GoreeCloud Ownership

Build and maintain the core engine, repository model, verification model, restore workflow, policy model, API, CLI, and user experience as GoreeCloud software.

Third-party backup products may inform engineering or provide bounded interoperability, but must not become the hidden upstream product identity.

## 4. Strong Recovery Independence

Support multiple independent repositories, clean-environment recovery, off-site and offline protection, restricted destructive authority, and provider-independent recovery procedures.

## 5. Verifiable Application Recovery

Treat application-consistent protection and representative restoration as product-level requirements rather than optional operational practices.

The objective is to validate usable application or dataset recovery, not merely copied files.

## 6. Privacy-Preserving Operations

Minimize the exposure of filenames, contents, secrets, encryption material, and personal information in monitoring, diagnostics, notifications, and administrative surfaces.

## 7. Efficient Large-Scale Storage

Use content-defined chunking, deduplication, compression, pack storage, incremental processing, and storage-aware transfer behavior to remain practical for large and frequently changing datasets.

## 8. Interoperability Without Dependency

Provide migration and recovery pathways for external or historical systems such as Kopia and Restic without requiring those products to remain part of the native runtime.

## 9. Evidence-Backed User Experience

Use Glaze UI and machine-readable APIs to present the strongest recovery state actually supported by evidence.

The interface must fail closed when protection, verification, credential, repository, or restore evidence is stale or incomplete.

## 10. Objective Evaluation

Future comparisons with other backup products must use documented, reproducible criteria such as restore success and validation, repository portability, corruption detection, failure recovery, storage efficiency, multi-repository independence, security controls, privacy behavior, automation and API coverage, and recovery without the original installation.

Competitive conclusions must be based on verified product behavior, not planned scope.
