use crate::primitives::{SchemaMetadata, TableMetadata};
use error::{Error, Result};
use redb::{Database, ReadableDatabase, ReadableTable, TableDefinition, TableHandle};
use std::path::Path;

const SCHEMA_METADATA_TABLE: TableDefinition<&str, SchemaMetadata> =
    TableDefinition::new("schema_metadata_table");

pub struct MetadataStore {
    db: Database,
}

impl MetadataStore {
    pub fn build<P: AsRef<Path>>(directory: P) -> Result<Self> {
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

    pub fn add_schema(&self, schema_name: &str) -> Result<()> {
        let name = format!("tables_{}", schema_name);
        let def: TableDefinition<&str, TableMetadata> = TableDefinition::new(&name);

        let tx = self.db.begin_write()?;

        {
            let mut table = tx.open_table(SCHEMA_METADATA_TABLE)?;

            if table.get(schema_name)?.is_some()
                || tx.list_tables()?.any(|t| t.name() == def.name())
            {
                return Err(Error::SchemaAlreadyExists(schema_name.to_string()));
            }

            table.insert(schema_name, SchemaMetadata::default())?;

            tx.open_table(def)?;
        }

        tx.commit()?;

        Ok(())
    }

    pub fn get_schema_metadata(&self, schema_name: &str) -> Result<Option<SchemaMetadata>> {
        let rx = self.db.begin_read()?;

        let table = rx.open_table(SCHEMA_METADATA_TABLE)?;

        let entry = table.get(schema_name)?;
        let entry = entry.map(|e| e.value());

        Ok(entry)
    }
}
