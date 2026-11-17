use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("schema \"{0}\" already exists")]
    SchemaAlreadyExists(String),

    #[error("schema \"{0}\" does not exist")]
    SchemaNotFound(String),

    #[error("table \"{schema}.{table}\" already exists")]
    TableAlreadyExists { schema: String, table: String },

    #[error("table \"{schema}.{table}\" does not exist")]
    TableNotFound { schema: String, table: String },

    #[error("metadata storage error: {0}")]
    Storage(#[from] redb::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

// redb returns a different error type from each kind of call
macro_rules! impl_from_redb {
    ($($ty:ty),*) => {
        $(
            impl From<$ty> for Error {
                fn from(e: $ty) -> Self {
                    Self::Storage(e.into())
                }
            }
        )*
    };
}

impl_from_redb!(
    redb::DatabaseError,
    redb::TransactionError,
    redb::TableError,
    redb::StorageError,
    redb::CommitError
);
