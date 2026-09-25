#![forbid(unsafe_code)]

//! Core GoreeCloud Backups domain invariants.
//!
//! Milestone 0 intentionally contains no backup engine. This crate begins by
//! encoding the recovery-first evidence boundary so later components cannot
//! equate a completed backup operation with a verified restore.

/// Canonical product name.
pub const PRODUCT_NAME: &str = "GoreeCloud Backups";

/// Development lifecycle represented by the current source foundation.
pub const LIFECYCLE: &str = "development";

/// Evidence required before GoreeCloud Backups may describe a recovery point
/// as restore verified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RecoveryEvidence {
    pub recovery_point_exists: bool,
    pub repository_accessible: bool,
    pub repository_integrity_verified: bool,
    pub recovery_credentials_available: bool,
    pub restore_succeeded: bool,
    pub restored_data_validated: bool,
}

impl RecoveryEvidence {
    /// Returns true only when every recovery-first requirement is satisfied.
    #[must_use]
    pub const fn is_restore_verified(self) -> bool {
        self.recovery_point_exists
            && self.repository_accessible
            && self.repository_integrity_verified
            && self.recovery_credentials_available
            && self.restore_succeeded
            && self.restored_data_validated
    }

    /// Returns the requirements that are still missing from the supplied
    /// recovery evidence.
    #[must_use]
    pub fn missing_requirements(self) -> Vec<EvidenceRequirement> {
        let checks = [
            (
                self.recovery_point_exists,
                EvidenceRequirement::RecoveryPointExists,
            ),
            (
                self.repository_accessible,
                EvidenceRequirement::RepositoryAccessible,
            ),
            (
                self.repository_integrity_verified,
                EvidenceRequirement::RepositoryIntegrityVerified,
            ),
            (
                self.recovery_credentials_available,
                EvidenceRequirement::RecoveryCredentialsAvailable,
            ),
            (
                self.restore_succeeded,
                EvidenceRequirement::RestoreSucceeded,
            ),
            (
                self.restored_data_validated,
                EvidenceRequirement::RestoredDataValidated,
            ),
        ];

        checks
            .into_iter()
            .filter_map(|(present, requirement)| (!present).then_some(requirement))
            .collect()
    }
}

/// Individual recovery-first evidence requirements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceRequirement {
    RecoveryPointExists,
    RepositoryAccessible,
    RepositoryIntegrityVerified,
    RecoveryCredentialsAvailable,
    RestoreSucceeded,
    RestoredDataValidated,
}

#[cfg(test)]
mod tests {
    use super::{EvidenceRequirement, RecoveryEvidence};

    #[test]
    fn complete_evidence_is_restore_verified() {
        let evidence = RecoveryEvidence {
            recovery_point_exists: true,
            repository_accessible: true,
            repository_integrity_verified: true,
            recovery_credentials_available: true,
            restore_succeeded: true,
            restored_data_validated: true,
        };

        assert!(evidence.is_restore_verified());
        assert!(evidence.missing_requirements().is_empty());
    }

    #[test]
    fn backup_existence_alone_is_not_restore_verified() {
        let evidence = RecoveryEvidence {
            recovery_point_exists: true,
            ..RecoveryEvidence::default()
        };

        assert!(!evidence.is_restore_verified());
        assert_eq!(
            evidence.missing_requirements(),
            vec![
                EvidenceRequirement::RepositoryAccessible,
                EvidenceRequirement::RepositoryIntegrityVerified,
                EvidenceRequirement::RecoveryCredentialsAvailable,
                EvidenceRequirement::RestoreSucceeded,
                EvidenceRequirement::RestoredDataValidated,
            ]
        );
    }

    #[test]
    fn successful_restore_without_validation_is_not_restore_verified() {
        let evidence = RecoveryEvidence {
            recovery_point_exists: true,
            repository_accessible: true,
            repository_integrity_verified: true,
            recovery_credentials_available: true,
            restore_succeeded: true,
            restored_data_validated: false,
        };

        assert!(!evidence.is_restore_verified());
        assert_eq!(
            evidence.missing_requirements(),
            vec![EvidenceRequirement::RestoredDataValidated]
        );
    }
}
