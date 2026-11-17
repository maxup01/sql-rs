use bitcode::{Decode, Encode};
use std::{collections::HashMap, vec::Vec};

#[derive(Debug, Encode, Decode)]
pub struct SchemaMetadata {
    pub tables: Vec<String>,
}

#[derive(Debug, Encode, Decode)]
pub struct TableMetadata {
    pub fields: HashMap<String, FieldMetadata>,
}

#[derive(Debug, Encode, Decode)]
pub struct FieldMetadata {}
