//! Application error types.

use std::io;

/// Application result type.
pub type Result<T> = std::result::Result<T, Error>;

/// Application error.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// IO error.
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}
