use bitcode::{Decode, Encode};
use std::vec::Vec;

#[derive(Debug, Encode, Decode)]
pub struct WALEntry {
    pub log_sequence_number: u64,
    pub transaction_id: u64,
    pub record: WALRecord,
}

pub enum WALRecord {}
