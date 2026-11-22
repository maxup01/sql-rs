use crate::disk_manager::{PageId, RelationId};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("table directory \"{}\" does not exist", .0.display())]
    TableDirectoryNotFound(PathBuf),

    #[error("column with relation id {} does not exist", .0.0)]
    ColumnNotFound(RelationId),

    #[error("page {} of relation id {} does not exist", .page.0, .rel_id.0)]
    PageNotFound { rel_id: RelationId, page: PageId },

    #[error("extent of {0} pages is invalid, it must be between 1 and pages per segment")]
    InvalidExtentSize(u64),
}

pub type Result<T> = std::result::Result<T, StorageError>;
