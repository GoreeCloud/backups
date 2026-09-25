#![forbid(unsafe_code)]

//! GoreeCloud Backups repository-format boundary.
//!
//! Milestone 0 deliberately blocks persistent repository writes until the
//! initial native repository format and compatibility rules are specified and
//! accepted.

use std::error::Error;
use std::fmt::{Display, Formatter};

/// Current persistence disposition for the native repository format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryPersistenceState {
    /// No persistent repository format has been accepted yet.
    BlockedUntilFormatAccepted,
}

/// Returns the current repository persistence state.
#[must_use]
pub const fn persistence_state() -> RepositoryPersistenceState {
    RepositoryPersistenceState::BlockedUntilFormatAccepted
}

/// Fails closed while the persistent repository format is not accepted.
pub fn require_persistence_ready() -> Result<(), RepositoryFormatUnavailable> {
    Err(RepositoryFormatUnavailable)
}

/// Error returned when code attempts to use repository persistence before the
/// initial format has been accepted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RepositoryFormatUnavailable;

impl Display for RepositoryFormatUnavailable {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("persistent GoreeCloud Backups repository format is not yet accepted")
    }
}

impl Error for RepositoryFormatUnavailable {}

#[cfg(test)]
mod tests {
    use super::{RepositoryPersistenceState, persistence_state, require_persistence_ready};

    #[test]
    fn milestone_zero_blocks_repository_persistence() {
        assert_eq!(
            persistence_state(),
            RepositoryPersistenceState::BlockedUntilFormatAccepted
        );
        assert!(require_persistence_ready().is_err());
    }
}
