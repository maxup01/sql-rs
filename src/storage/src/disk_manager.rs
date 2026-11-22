use std::{
    collections::{HashMap, hash_map::Entry},
    fs::{File, OpenOptions},
    io,
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

    // Allocates n contiguous pages and returns the first one. Either all n pages
    // are allocated or none
    pub fn allocate_extent(&mut self, rel_id: RelationId, n: u64) -> crate::Result<PageId> {
        if n == 0 || n > PAGES_PER_SEGMENT as u64 {
            return Err(StorageError::InvalidExtentSize(n));
        }

        let first_page = *self.page_numbers.get(&rel_id).unwrap_or(&0);
        let mut written_parts: Vec<(u64, u64)> = Vec::new();
        let mut allocated = 0;

        while allocated < n {
            let page = first_page + allocated;
            let segment = page / PAGES_PER_SEGMENT as u64;
            let page_in_segment = page % PAGES_PER_SEGMENT as u64;
            let count = (n - allocated).min(PAGES_PER_SEGMENT as u64 - page_in_segment);
            let offset = page_in_segment * PAGE_SIZE as u64;

            let result = self
                .segment_file(rel_id, segment, true)
                .and_then(|file| Ok(zero_pages(file, offset, count)?));

            if let Err(e) = result {
                for (segment, offset) in written_parts {
                    if let Some(file) = self.files.get(&(rel_id, segment)) {
                        let _ = file.set_len(offset);
                    }
                }
                return Err(e);
            }

            written_parts.push((segment, offset));
            allocated += count;
        }

        self.page_numbers.insert(rel_id, first_page + n);
        Ok(PageId(first_page))
    }

    pub fn number_of_pages(&self, rel_id: RelationId) -> crate::Result<u64> {
        self.page_numbers
            .get(&rel_id)
            .map(|v| *v)
            .ok_or(StorageError::ColumnNotFound(rel_id))
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

// Appends count zeroed pages at offset, which must be the current end of the file.
// On failure the file is truncated back to offset, so no partial pages are left behind
fn zero_pages(file: &File, offset: u64, count: u64) -> io::Result<()> {
    for i in 0..count {
        if let Err(e) = file.write_all_at(&[0; PAGE_SIZE], offset + i * PAGE_SIZE as u64) {
            let _ = file.set_len(offset);
            return Err(e);
        }
    }

    Ok(())
}

// Segment files are named "rel_<relation id>.<segment number>", e.g. "rel_17.0"
fn segment_name(relation: RelationId, segment: u64) -> String {
    format!("rel_{}.{}", relation.0, segment)
}

fn parse_segment_name(name: &str) -> Option<(RelationId, u64)> {
    let (relation, segment) = name.strip_prefix("rel_")?.split_once('.')?;
    Some((RelationId(relation.parse().ok()?), segment.parse().ok()?))
}
