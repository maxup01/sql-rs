use std::vec::Vec;

pub struct WALEntry {
    pub row_id: u64,
    pub record_bytes: Vec<u8>,
}
