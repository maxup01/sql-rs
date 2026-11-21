use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("table directory \"{}\" does not exist", .0.display())]
    TableDirectoryNotFound(PathBuf),
}

pub type Result<T> = std::result::Result<T, StorageError>;
