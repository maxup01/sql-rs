use bitcode::{Decode, Encode};

// 16KB
pub const PAGE_SIZE: usize = 16_384;

// Identifier of a column which is unique across database
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Encode, Decode)]
pub struct RelationId(pub u32);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Encode, Decode)]
pub struct PageId(pub u64);

// Position of a record in the WAL, only grows
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Encode, Decode)]
pub struct LogSequenceNumber(pub u64);
