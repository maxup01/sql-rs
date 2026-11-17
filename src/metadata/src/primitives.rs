use std::{collections::HashMap, vec::Vec};

pub struct SchemaMetadata {
    tables: Vec<String>,
}

pub struct TableMetadata {
    fields: HashMap<String, FieldMetadata>,
}

pub struct FieldMetadata {}
