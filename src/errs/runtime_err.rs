use std::io;
#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("unmatched [")]
    UnmatchedOpenBracket,
    #[error("unmatched ]")]
    UnmatchedClosingBracket,
    #[error("runtime io error: {0}")]
    Io(#[from] io::Error),
}
