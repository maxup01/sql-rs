use std::path::{Path, PathBuf};

use crate::StorageError;

// 16KB
pub const PAGE_SIZE: usize = 16_384;

// 1GB which fits 65,536 pages
pub const SEGMENT_SIZE: usize = 1_073_741_824;
pub const PAGES_PER_SEGMENT: usize = SEGMENT_SIZE / PAGE_SIZE;

// Identifier of a column which is unique across database
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct RelationId(pub u32);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct PageId(pub u64);

pub struct DiskManager {
    table_dir: PathBuf,
}

impl DiskManager {
    pub fn open<P: AsRef<Path>>(table_dir: P) -> crate::Result<Self> {
        let exists = std::fs::exists(&table_dir)?;
        let path = table_dir.as_ref().to_path_buf();

        if !exists {
            return Err(StorageError::TableDirectoryNotFound(path));
        }

        Ok(Self { table_dir: path })
    }
}
