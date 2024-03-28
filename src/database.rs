use diesel::{
    r2d2::{ConnectionManager, Pool},
    sqlite::{Sqlite, SqliteConnection},
};
use dotenvy::dotenv;
use std::{env, error::Error};

pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn get_connection_pool() -> SqlitePool {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL does not exist in .env");

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn run_migrations(
    connection: &mut impl MigrationHarness<Sqlite>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}
