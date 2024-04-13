use thiserror::Error;

/// A specialized `Result` type for application.
pub type Result<T> = std::result::Result<T, self::Error>;

/// Custom error type for application.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("Source code is too long")]
    TooLong,
    #[error("Unable to find source code")]
    NotFound,
    #[error("Unknown platform")]
    UnknownPlatform,
}
