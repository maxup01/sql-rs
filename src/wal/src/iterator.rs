use common::LogSequenceNumber;

pub struct WALIterator {
    file: File,
    current_position: LogSequenceNumber,
}
