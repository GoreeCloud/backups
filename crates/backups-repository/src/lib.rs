#![forbid(unsafe_code)]

//! GoreeCloud Backups repository-format boundary.
//!
//! Repository Format V1 is specified for implementation, but persistent writes
//! remain blocked until the format implementation and recovery tests are
//! separately accepted.

use std::error::Error;
use std::fmt::{Display, Formatter};

/// Native repository format major version.
pub const FORMAT_MAJOR: u16 = 1;

/// Native repository format minor version.
pub const FORMAT_MINOR: u16 = 0;

/// Stable identifier for the V1 repository profile.
pub const FORMAT_PROFILE: &str = "goreecloud-backups/repository-v1";

/// Current persistence disposition for the native repository format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryPersistenceState {
    /// The V1 format is specified, but its implementation is not yet accepted
    /// for persistent repository writes.
    BlockedUntilImplementationAccepted,
}

/// Returns the current repository persistence state.
#[must_use]
pub const fn persistence_state() -> RepositoryPersistenceState {
    RepositoryPersistenceState::BlockedUntilImplementationAccepted
}

/// Fails closed while the persistent repository implementation is not accepted.
pub fn require_persistence_ready() -> Result<(), RepositoryFormatUnavailable> {
    Err(RepositoryFormatUnavailable)
}

/// Error returned when code attempts to use repository persistence before the
/// V1 implementation and recovery tests have been accepted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RepositoryFormatUnavailable;

impl Display for RepositoryFormatUnavailable {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(
            "GoreeCloud Backups repository format V1 is specified but persistence is not yet accepted",
        )
    }
}

impl Error for RepositoryFormatUnavailable {}

#[cfg(test)]
mod tests {
    use super::{
        FORMAT_MAJOR, FORMAT_MINOR, FORMAT_PROFILE, RepositoryPersistenceState,
        persistence_state, require_persistence_ready,
    };

    #[test]
    fn repository_format_identity_is_v1() {
        assert_eq!(FORMAT_MAJOR, 1);
        assert_eq!(FORMAT_MINOR, 0);
        assert_eq!(FORMAT_PROFILE, "goreecloud-backups/repository-v1");
    }

    #[test]
    fn milestone_one_keeps_repository_persistence_blocked() {
        assert_eq!(
            persistence_state(),
            RepositoryPersistenceState::BlockedUntilImplementationAccepted
        );
        assert!(require_persistence_ready().is_err());
    }
}
