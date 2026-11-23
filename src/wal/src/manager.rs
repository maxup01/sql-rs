use crate::{TransactionId, WALEntry, WALError, WALRecord};
use bitcode::{Decode, Encode};
use common::LogSequenceNumber;
use std::{
    fs::{File, OpenOptions},
    io::{Seek, SeekFrom, Write},
    marker::PhantomData,
    path::{Path, PathBuf},
};

// T is the type stored in WALRecord::Data, e.g. a page change defined by storage
pub struct WALManager<T> {
    root: PathBuf,
    file: File,
    // LSN of the next record, which is its byte position in the active file
    next_lsn: LogSequenceNumber,
    _record: PhantomData<T>,
}

impl<T: Encode + for<'a> Decode<'a>> WALManager<T> {
    pub fn open<P: AsRef<Path>>(root: P) -> crate::Result<Self> {
        let exists = std::fs::exists(&root)?;
        let path = root.as_ref().to_path_buf();

        if !exists {
            return Err(WALError::WALDirectoryNotFound(path));
        }

        // The newest WAL file is the one with the greatest number
        let mut newest = None;

        for entry in std::fs::read_dir(&path)? {
            let entry = entry?;

            if !entry.file_type()?.is_file() {
                continue;
            }

            // Skip files that are not WAL files, e.g. a future LOCK or header file
            let Some(number) = entry.file_name().to_str().and_then(parse_wal_file_name) else {
                continue;
            };

            newest = newest.max(Some(number));
        }

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path.join(wal_file_name(newest.unwrap_or(0))))?;

        // Records are appended, so writing starts at the end of the file
        let next_lsn = LogSequenceNumber(file.seek(SeekFrom::End(0))?);

        Ok(Self {
            root: path,
            file,
            next_lsn,
            _record: PhantomData,
        })
    }
}

// WAL files are named "wal_<number>", zero-padded so they sort by name, e.g. "wal_0000000000"
fn wal_file_name(number: u64) -> String {
    format!("wal_{number:010}")
}

fn parse_wal_file_name(name: &str) -> Option<u64> {
    name.strip_prefix("wal_")?.parse().ok()
}
