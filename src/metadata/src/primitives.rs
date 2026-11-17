use bitcode::{Decode, Encode};
use std::{collections::HashMap, vec::Vec};

#[derive(Debug, Encode, Decode)]
pub struct SchemaMetadata {
    pub tables: Vec<String>,
}

impl redb::Value for SchemaMetadata {
    type SelfType<'a> = SchemaMetadata;
    type AsBytes<'a> = Vec<u8>;

    fn fixed_width() -> Option<usize> {
        None
    }

    fn from_bytes<'a>(data: &'a [u8]) -> SchemaMetadata
    where
        Self: 'a,
    {
        bitcode::decode(data).expect("corrupted SchemaMetadata")
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a SchemaMetadata) -> Vec<u8>
    where
        Self: 'b,
    {
        bitcode::encode(value)
    }

    fn type_name() -> redb::TypeName {
        redb::TypeName::new("sql_rs::SchemaMetadata")
    }
}

#[derive(Debug, Encode, Decode)]
pub struct TableMetadata {
    pub fields: HashMap<String, FieldMetadata>,
}

impl redb::Value for TableMetadata {
    type SelfType<'a> = TableMetadata;
    type AsBytes<'a> = Vec<u8>;

    fn fixed_width() -> Option<usize> {
        None
    }

    fn from_bytes<'a>(data: &'a [u8]) -> TableMetadata
    where
        Self: 'a,
    {
        bitcode::decode(data).expect("corrupted TableMetadata")
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a TableMetadata) -> Vec<u8>
    where
        Self: 'b,
    {
        bitcode::encode(value)
    }

    fn type_name() -> redb::TypeName {
        redb::TypeName::new("sql_rs::TableMetadata")
    }
}

#[derive(Debug, Encode, Decode)]
pub struct FieldMetadata {}

impl redb::Value for FieldMetadata {
    type SelfType<'a> = FieldMetadata;
    type AsBytes<'a> = Vec<u8>;

    fn fixed_width() -> Option<usize> {
        None
    }

    fn from_bytes<'a>(data: &'a [u8]) -> FieldMetadata
    where
        Self: 'a,
    {
        bitcode::decode(data).expect("corrupted FieldMetadata")
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a FieldMetadata) -> Vec<u8>
    where
        Self: 'b,
    {
        bitcode::encode(value)
    }

    fn type_name() -> redb::TypeName {
        redb::TypeName::new("sql_rs::FieldMetadata")
    }
}
