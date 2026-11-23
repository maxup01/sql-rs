use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WALError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("wal directory \"{}\" does not exist", .0.display())]
    WALDirectoryNotFound(PathBuf),

    #[error("wal record of {size} bytes is invalid, it must be between 1 and {max} bytes")]
    InvalidRecordSize { size: usize, max: usize },
}

pub type Result<T> = std::result::Result<T, WALError>;
