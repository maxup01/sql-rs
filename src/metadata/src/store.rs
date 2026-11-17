use crate::primitives::SchemaMetadata;
use redb::{Database, TableDefinition};

const SCHEMA_METADATA_TABLE: TableDefinition<&str, SchemaMetadata> =
    TableDefinition::new("schema_metadata_table");
const SCHEMA_METADATA_TABLE: TableDefinition<&str, SchemaMetadata> =
    TableDefinition::new("schema_metadata_table");

pub struct MetadataStore {
    db: Database,
}
