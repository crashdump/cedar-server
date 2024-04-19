use diesel::{prelude::*, r2d2};

use crate::storage::error::DatabaseError;


pub type ConnectionPoolResult<P: ConnectionPool> = Result<r2d2::Pool<P::Manager>, DatabaseError>;


pub trait ConnectionPool: Send + Sized + 'static {
    type Manager: r2d2::ManageConnection<Connection=Self>;

    type Error: std::fmt::Debug;

    fn init_pool(url: String) -> ConnectionPoolResult<Self>;
}

#[cfg(feature = "db_sqlite")]
impl ConnectionPool for SqliteConnection {
    type Manager = r2d2::ConnectionManager<SqliteConnection>;
    type Error = std::convert::Infallible;

    // Set this to ":memory:" for in-memory storage.
    fn init_pool(url: String) -> ConnectionPoolResult<Self> {
        let manager = r2d2::ConnectionManager::new(url);
        let pool = r2d2::Pool::builder()
            .max_size(1) // A new database is created for each connection to the in-memory SQLite
            .test_on_check_out(true)
            .build(manager)
            .expect("Could not build connection pool");

        Ok(pool)
    }
}

#[cfg(feature = "db_postgres")]
impl ConnectionPool for PgConnection {
    type Manager = r2d2::ConnectionManager<PgConnection>;
    type Error = std::convert::Infallible;

    fn init_pool(url: String) -> ConnectionPoolResult<Self> {
        const POOL_MIN_IDLE_CONN: u32 = 1;
        const POOL_MAX_CONN: usize = 20;

        let cpu_num = num_cpus::get();
        let pool_num = std::cmp::max(POOL_MAX_CONN, cpu_num * 2 + 1) as u32;
        let manager = r2d2::ConnectionManager::new(url);

        let pool = r2d2::Pool::builder()
            .min_idle(Some(POOL_MIN_IDLE_CONN))
            .max_size(pool_num)
            .test_on_check_out(true)
            .build(manager)
            .expect("Could not build connection pool");

        Ok(pool)
    }
}

