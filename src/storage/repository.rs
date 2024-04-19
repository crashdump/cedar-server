use diesel::{prelude::*, r2d2, ExpressionMethods};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel_migrations::{
    embed_migrations, EmbeddedMigrations, HarnessWithOutput, MigrationHarness,
};
use tracing::info;
use uuid::Uuid;

use crate::storage::error::DatabaseError;
use crate::storage::model;
use crate::storage::model::Store;
use crate::storage::schema::policies;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub type DatabaseResult<T> = Result<T, DatabaseError>;

#[derive(Clone)]
pub struct Repository {
    db_pool: r2d2::Pool<r2d2::ConnectionManager<PgConnection>>,
}

impl Repository {
    pub fn new(url: String) -> Repository {
        let manager = r2d2::ConnectionManager::new(url);
        let pool = r2d2::Pool::builder()
            .max_size(1) // A new database is created for each connection to the in-memory SQLite
            .test_on_check_out(true)
            .build(manager)
            .expect("Could not build connection pool");
        Repository {
            db_pool: pool,
        }
    }

    pub fn get_connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, DatabaseError> {
        self.db_pool.get().map_err(Into::into)
    }


    pub async fn run_migrations(&self) {
        let conn = &mut self.get_connection().unwrap();

        info!("Running migrations");
        HarnessWithOutput::write_to_stdout(conn)
            .run_pending_migrations(MIGRATIONS)
            .unwrap();
        info!("Migrations finished");
    }
}