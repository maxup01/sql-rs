use std::{collections::HashMap, vec::Vec};

pub struct SchemaMetadata {
    pub tables: Vec<String>,
}

pub struct TableMetadata {
    pub fields: HashMap<String, FieldMetadata>,
}

pub struct FieldMetadata {}
