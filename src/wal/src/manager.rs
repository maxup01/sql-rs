use crate::{TransactionId, WALEntry, WALError, WALRecord};
use bitcode::{Decode, Encode};
use common::LogSequenceNumber;
use std::{
    collections::HashMap,
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

        // Names are zero-padded, so the newest WAL file is the greatest name
        let name = match files.keys().filter(|name| name.starts_with("wal_")).max() {
            Some(name) => name.clone(),
            None => wal_file_name(0),
        };

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path.join(&name))?;

        let next_lsn = LogSequenceNumber(files[&active].metadata()?.len());

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
