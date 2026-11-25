use common::LogSequenceNumber;
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

    #[error("wal record at lsn {} has a valid checksum but could not be decoded", .0.0)]
    UndecodableRecord(LogSequenceNumber),
}

pub type Result<T> = std::result::Result<T, WALError>;
