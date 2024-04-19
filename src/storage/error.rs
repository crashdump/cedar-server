use crate::storage;

#[derive(thiserror::Error, Debug)]
pub enum DatabaseError  {
    #[error(transparent)]
    Query(#[from] storage::diesel::result::Error),

    #[error(transparent)]
    Connection(#[from] diesel::result::ConnectionError),

    #[error(transparent)]
    ConnectionPool(#[from] diesel::r2d2::Error),

    #[error(transparent)]
    ConnectionPoolManagement(#[from] diesel::r2d2::PoolError),

    #[error(transparent)]
    Migration(#[from] diesel_migrations::MigrationError),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),

    #[error("Logical error: {0}")]
    Logical(String),

    /// Any other errors that are too trivial to be put here explicitly.
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}