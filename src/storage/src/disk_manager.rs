use std::{
    collections::{HashMap, hash_map::Entry},
    fs::{File, OpenOptions},
    os::unix::fs::FileExt,
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
    page_numbers: HashMap<RelationId, u64>,
}

impl DiskManager {
    pub fn open<P: AsRef<Path>>(root: P) -> crate::Result<Self> {
        let exists = std::fs::exists(&root)?;
        let path = root.as_ref().to_path_buf();

        if !exists {
            return Err(StorageError::TableDirectoryNotFound(path));
        }

        let mut files = HashMap::new();
        let mut page_numbers = HashMap::new();

        for entry in std::fs::read_dir(&path)? {
            let entry = entry?;

            if !entry.file_type()?.is_file() {
                continue;
            }

            // Skip files that are not segments, e.g. a future LOCK or header file
            let Some((rel_id, segment)) = entry.file_name().to_str().and_then(parse_segment_name)
            else {
                continue;
            };

            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(entry.path())?;
            let metadata = file.metadata()?;

            files.insert((rel_id, segment), file);
            page_numbers.insert(
                rel_id,
                *page_numbers.get(&rel_id).unwrap_or(&0) + (metadata.len() / (PAGE_SIZE as u64)),
            );
        }

        Ok(Self {
            root: path,
            files,
            page_numbers,
        })
    }

    pub fn read_page(
        &self,
        rel_id: RelationId,
        page: PageId,
        buf: &mut [u8; PAGE_SIZE],
    ) -> crate::Result<()> {
        let segment = page.0 / PAGES_PER_SEGMENT as u64;
        let pages = *self.page_numbers.get(&rel_id).unwrap_or(&0);
        let offset = page.0 % (PAGES_PER_SEGMENT as u64);

        if page.0 >= pages {
            return Err(StorageError::PageNotFound { rel_id, page });
        }

        let file = OpenOptions::new()
            .read(true)
            .create(false)
            .open(self.root.join(segment_name(rel_id, segment)))?;
        file.read_exact_at(buf, offset)?;

        Ok(())
    }

    pub fn write_page(
        &mut self,
        rel_id: RelationId,
        page: PageId,
        buf: &[u8; PAGE_SIZE],
    ) -> crate::Result<()> {
        let segment = page.0 / PAGES_PER_SEGMENT as u64;
        let offset = (page.0 % PAGES_PER_SEGMENT as u64) * PAGE_SIZE as u64;

        let file = self.segment_file(rel_id, segment, false)?;
        file.write_all_at(buf, offset)?;

        Ok(())
    }

    pub fn allocate_page(&mut self, rel_id: RelationId) -> crate::Result<PageId> {
        let page = *self.page_numbers.get(&rel_id).unwrap_or(&0);
        let segment = page / PAGES_PER_SEGMENT as u64;
        let offset = (page % PAGES_PER_SEGMENT as u64) * PAGE_SIZE as u64;

        let file = self.segment_file(rel_id, segment, true)?;
        file.write_all_at(&[0u8; PAGE_SIZE], offset)?;

        self.page_numbers.insert(rel_id, page + 1);
        Ok(PageId(page))
    }

    fn segment_file(
        &mut self,
        rel_id: RelationId,
        segment: u64,
        create: bool,
    ) -> crate::Result<&File> {
        match self.files.entry((rel_id, segment)) {
            Entry::Occupied(entry) => Ok(entry.into_mut()),
            Entry::Vacant(entry) => {
                let file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(create)
                    .open(self.root.join(segment_name(rel_id, segment)))?;

                Ok(entry.insert(file))
            }
        }
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
