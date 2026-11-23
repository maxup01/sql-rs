mod error;
mod manager;
mod primitives;

pub use error::{Result, WALError};
pub use manager::WALManager;
pub use primitives::WALEntry;
