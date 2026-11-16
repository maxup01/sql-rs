use bitcode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Encode, Decode)]
pub enum Request {
    Shutdown,
}
