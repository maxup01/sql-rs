use crate::{TransactionId, WALEntry, WALError, WALRecord};
use bitcode::{Decode, Encode};
use common::{LogSequenceNumber, PAGE_SIZE};
use std::{
    fs::{File, OpenOptions},
    io::{Seek, SeekFrom, Write},
    marker::PhantomData,
    path::{Path, PathBuf},
};

const FRAME_HEADER_SIZE: usize = 8;

// Largest payload a record may have: a full page image plus room for the rest of the entry.
// Recovery treats a len of 0 or above this as the end of the valid log, so append never writes one
const MAX_RECORD_SIZE: usize = 2 * PAGE_SIZE;

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

    // Appends a record to the end of the active WAL file and returns its LSN.
    // The record is handed to the OS but not fsynced
    pub fn append(
        &mut self,
        transaction_id: TransactionId,
        record: WALRecord<T>,
    ) -> crate::Result<LogSequenceNumber> {
        let log_sequence_number = self.next_lsn;
        let entry = WALEntry {
            log_sequence_number,
            transaction_id,
            record,
        };

        let payload = bitcode::encode(&entry);

        if payload.is_empty() || payload.len() > MAX_RECORD_SIZE {
            return Err(WALError::InvalidRecordSize {
                size: payload.len(),
                max: MAX_RECORD_SIZE,
            });
        }

        let mut frame = Vec::with_capacity(FRAME_HEADER_SIZE + payload.len());
        frame.extend_from_slice(&(payload.len() as u32).to_le_bytes());
        frame.extend_from_slice(&crc32fast::hash(&payload).to_le_bytes());
        frame.extend_from_slice(&payload);

        if let Err(e) = self.file.write_all(&frame) {
            let _ = self.file.set_len(log_sequence_number.0);
            let _ = self.file.seek(SeekFrom::Start(log_sequence_number.0));
            return Err(e.into());
        }

        self.next_lsn = LogSequenceNumber(log_sequence_number.0 + frame.len() as u64);
        Ok(log_sequence_number)
    }
}

// WAL files are named "wal_<number>", zero-padded so they sort by name, e.g. "wal_0000000000"
fn wal_file_name(number: u64) -> String {
    format!("wal_{number:010}")
}

fn parse_wal_file_name(name: &str) -> Option<u64> {
    name.strip_prefix("wal_")?.parse().ok()
}
