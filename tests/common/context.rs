use diesel::{Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub(crate) const MIGRATIONS: EmbeddedMigrations = embed_migrations!("tests/fixtures/");

pub const TEST_PG_URL: &str = "postgres://root:local_only@localhost/cedar_server";

pub struct TestContext {}

impl TestContext {
    pub fn new() -> Self {
        println!("Set up resources");

        let mut conn = PgConnection::establish(TEST_PG_URL)
            .expect(&format!("Cannot connect to database: {}", TEST_PG_URL));

        conn.run_pending_migrations(MIGRATIONS).unwrap();

        Self {}
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        println!("Clean up resources");

        let mut conn = PgConnection::establish(TEST_PG_URL)
            .expect(&format!("Cannot connect to database: {}", TEST_PG_URL));

        conn.revert_all_migrations(MIGRATIONS).unwrap();
    }
}
