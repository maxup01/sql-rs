use crate::{
    WALEntry, WALError,
    manager::{FRAME_HEADER_SIZE, MAX_RECORD_SIZE},
};
use bitcode::Decode;
use common::LogSequenceNumber;
use std::{
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
    marker::PhantomData,
};

// Reads WAL records one after another, starting at a given LSN.
pub struct WALIterator<T> {
    reader: BufReader<File>,
    current_position: LogSequenceNumber,
    file_len: u64,
    finished: bool,
    _record: PhantomData<T>,
}

impl<T> WALIterator<T> {
    // pos must be the start of a record, e.g. an LSN returned by append or a checkpoint.
    pub fn new(file: File, pos: LogSequenceNumber) -> crate::Result<Self> {
        let file_len = file.metadata()?.len();

        let mut reader = BufReader::new(file);
        reader.seek(SeekFrom::Start(pos.0))?;

        Ok(Self {
            reader,
            current_position: pos,
            file_len,
            finished: false,
            _record: PhantomData,
        })
    }
}

impl<T: for<'a> Decode<'a>> WALIterator<T> {
    fn read_next(&mut self) -> crate::Result<Option<WALEntry<T>>> {
        let position = self.current_position.0;

        if position + FRAME_HEADER_SIZE as u64 > self.file_len {
            return Ok(None);
        }

        let mut len_bytes = [0u8; 4];
        self.reader.read_exact(&mut len_bytes)?;

        let mut crc_bytes = [0u8; 4];
        self.reader.read_exact(&mut crc_bytes)?;

        let len = u32::from_le_bytes(len_bytes) as usize;
        let crc = u32::from_le_bytes(crc_bytes);

        let record_end = position + (FRAME_HEADER_SIZE + len) as u64;

        if len == 0 || len > MAX_RECORD_SIZE || record_end > self.file_len {
            return Ok(None);
        }

        let mut payload = vec![0u8; len];
        self.reader.read_exact(&mut payload)?;

        if crc32fast::hash(&payload) != crc {
            return Ok(None);
        }

        let entry = bitcode::decode(&payload)
            .map_err(|_| WALError::UndecodableRecord(self.current_position))?;

        self.current_position = LogSequenceNumber(record_end);

        Ok(Some(entry))
    }
}

impl<T: for<'a> Decode<'a>> Iterator for WALIterator<T> {
    type Item = crate::Result<WALEntry<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let result = self.read_next().transpose();

        if let None = result {
            self.finished = true;
        }

        result
    }
}
