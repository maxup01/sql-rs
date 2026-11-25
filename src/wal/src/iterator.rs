use common::LogSequenceNumber;

pub struct WALIterator {
    file: File,
    current_position: LogSequenceNumber,
}

impl WALIterator {
    pub fn new(file: File, pos: LogSequenceNumber) -> Self {
        Self {
            file,
            current_position: pos,
        }
    }
}
