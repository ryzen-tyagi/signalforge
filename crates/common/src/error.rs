use thiserror::Error;

pub type Result<T> = std::result::Result<T, SignalForgeError>;

#[derive(Debug, Error)]
pub enum SignalForgeError {
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("not found")]
    NotFound,
    #[error("dependency failed: {0}")]
    Dependency(String),
}
