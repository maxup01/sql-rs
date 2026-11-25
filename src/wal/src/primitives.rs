use bitcode::{Decode, Encode};
use common::{LogSequenceNumber, PageId, RelationId};
use std::vec::Vec;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Encode, Decode)]
pub struct TransactionId(pub u64);

#[derive(Debug, Encode, Decode)]
pub struct WALEntry<P> {
    pub log_sequence_number: LogSequenceNumber,
    pub record: WALRecord<P>,
}

#[derive(Debug, Encode, Decode)]
pub enum WALRecord<T> {
    Data {
        transaction_id: TransactionId,
        change: T,
    },
    Commit(TransactionId),
    Checkpoint {
        redo_from: LogSequenceNumber,
    },
}

#[derive(Debug, Encode, Decode)]
pub enum PageChange {
    PageWrite {
        rel_id: RelationId,
        page: PageId,
        offset: u16,
        bytes: Vec<u8>,
    },
    FullPage {
        rel_id: RelationId,
        page: PageId,
        image: Vec<u8>,
    },
    AllocateExtent {
        rel_id: RelationId,
        first_page: PageId,
        n: u64,
    },
}
