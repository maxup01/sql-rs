use crate::primitives::SchemaMetadata;
use redb::{Database, TableDefinition};
use std::path::Path;

const SCHEMA_METADATA_TABLE: TableDefinition<&str, SchemaMetadata> =
    TableDefinition::new("schema_metadata_table");
const TABLE_METADATA_TABLE: TableDefinition<&str, SchemaMetadata> =
    TableDefinition::new("table_metadata_table");

pub struct MetadataStore {
    db: Database,
}

impl MetadataStore {
    pub fn build<P: AsRef<Path>>(directory: P) -> Result<Self, redb::Error> {
        const METADATA_STORE_NAME: &str = "sql_rs_metadata_store";

        let db_path = directory.as_ref().to_path_buf().join(METADATA_STORE_NAME);
        let db = Database::create(&db_path)?;

        let tx = db.begin_write()?;

        {
            tx.open_table(SCHEMA_METADATA_TABLE)?;
        };

        tx.commit()?;

        Ok(Self { db })
    }
}
