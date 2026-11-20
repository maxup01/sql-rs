use crate::primitives::{SchemaDetails, SchemaMetadata, TableMetadata};
use error::{Error, Result};
use redb::{Database, ReadableDatabase, ReadableTable, TableDefinition, TableHandle};
use std::{collections::HashMap, path::Path};

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

    pub fn get_schema_metadata(&self, schema_name: &str) -> Result<Option<SchemaMetadata>> {
        let rx = self.db.begin_read()?;

        let table = rx.open_table(SCHEMA_METADATA_TABLE)?;

        let entry = table.get(schema_name)?;
        let entry = entry.map(|e| e.value());

        Ok(entry)
    }

    pub fn create_schema(&self, schema_name: &str) -> Result<()> {
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

    pub fn delete_schema(&self, schema_name: &str) -> Result<()> {
        let name = format!("tables_{}", schema_name);
        let def: TableDefinition<&str, TableMetadata> = TableDefinition::new(&name);

        let tx = self.db.begin_write()?;

        {
            let mut table = tx.open_table(SCHEMA_METADATA_TABLE)?;
            table.remove(schema_name)?;

            tx.delete_table(def)?;
        }

        tx.commit()?;

        Ok(())
    }

    pub fn create_table(
        &self,
        schema_name: &str,
        table_name: &str,
        table_metadata: TableMetadata,
    ) -> Result<()> {
        let name = format!("tables_{}", schema_name);
        let def: TableDefinition<&str, TableMetadata> = TableDefinition::new(&name);

        let tx = self.db.begin_write()?;

        {
            let mut schemas = tx.open_table(SCHEMA_METADATA_TABLE)?;

            let mut schema_metadata = match schemas.get(schema_name)? {
                Some(entry) => entry.value(),
                None => return Err(Error::SchemaNotFound(schema_name.to_string())),
            };

            let mut tables = tx.open_table(def)?;

            if tables.get(table_name)?.is_some() {
                return Err(Error::TableAlreadyExists {
                    schema: schema_name.to_string(),
                    table: table_name.to_string(),
                });
            }

            tables.insert(table_name, table_metadata)?;

            schema_metadata.tables.push(table_name.to_string());
            schemas.insert(schema_name, schema_metadata)?;
        }

        tx.commit()?;

        Ok(())
    }

    pub fn get_schema_details(&self, schema_name: &str) -> Result<SchemaDetails> {
        let name = format!("tables_{}", schema_name);
        let def: TableDefinition<&str, TableMetadata> = TableDefinition::new(&name);

        let rx = self.db.begin_read()?;
        let table = rx.open_table(def)?;

        let entries = table
            .iter()?
            .map(|item| {
                let (key, value) = item?;
                Ok((key.value().to_string(), value.value()))
            })
            .collect::<Result<HashMap<_, _>>>()?;

        Ok(SchemaDetails { tables: entries })
    }

    pub fn delete_table(&self, schema_name: &str, table_name: &str) -> Result<()> {
        let name = format!("tables_{}", schema_name);
        let def: TableDefinition<&str, TableMetadata> = TableDefinition::new(&name);

        let tx = self.db.begin_write()?;

        {
            let mut schemas = tx.open_table(SCHEMA_METADATA_TABLE)?;

            let mut schema_metadata = match schemas.get(schema_name)? {
                Some(entry) => entry.value(),
                None => return Err(Error::SchemaNotFound(schema_name.to_string())),
            };

            let mut tables = tx.open_table(def)?;

            if tables.remove(table_name)?.is_none() {
                return Err(Error::TableNotFound {
                    schema: schema_name.to_string(),
                    table: table_name.to_string(),
                });
            }

            schema_metadata.tables.retain(|t| t != table_name);
            schemas.insert(schema_name, schema_metadata)?;
        }

        tx.commit()?;

        Ok(())
    }
}
