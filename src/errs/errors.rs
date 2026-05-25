use std::io;

use super::RuntimeError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Runtime(#[from] RuntimeError),
    #[error("Io Error: {0}")]
    IoError(#[from] io::Error),
    #[error("Expected Argument: {0}")]
    ArgumentError(String),
}
