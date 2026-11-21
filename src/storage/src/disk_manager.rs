use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    path::{Path, PathBuf},
};

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
    root: PathBuf,
    // Open segment files, keyed by (relation, segment number)
    files: HashMap<(RelationId, u64), File>,
}

impl DiskManager {
    pub fn open<P: AsRef<Path>>(root: P) -> crate::Result<Self> {
        let exists = std::fs::exists(&root)?;
        let path = root.as_ref().to_path_buf();

        if !exists {
            return Err(StorageError::TableDirectoryNotFound(path));
        }

        let mut files = HashMap::new();

        for entry in std::fs::read_dir(&path)? {
            let entry = entry?;

            if !entry.file_type()?.is_file() {
                continue;
            }

            // Skip files that are not segments, e.g. a future LOCK or header file
            let Some(key) = entry.file_name().to_str().and_then(parse_segment_name) else {
                continue;
            };

            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(entry.path())?;
            files.insert(key, file);
        }

        Ok(Self { root: path, files })
    }
}

// Segment files are named "rel_<relation id>.<segment number>", e.g. "rel_17.0"
fn segment_name(relation: RelationId, segment: u64) -> String {
    format!("rel_{}.{}", relation.0, segment)
}

fn parse_segment_name(name: &str) -> Option<(RelationId, u64)> {
    let (relation, segment) = name.strip_prefix("rel_")?.split_once('.')?;
    Some((RelationId(relation.parse().ok()?), segment.parse().ok()?))
}
