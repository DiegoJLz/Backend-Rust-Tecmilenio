use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, Pool, PooledConnection};
use dotenvy::dotenv;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn create_pool() -> Result<DbPool, Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    Ok(pool)
}

pub fn run_migrations() -> Result<(), Box<dyn std::error::Error>> {
    use diesel_migrations::{embed_migrations, MigrationHarness};

    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let mut conn = PgConnection::establish(&database_url)?;

    // Embed migrations at compile time
    embed_migrations!("migrations/");

    // Run migrations
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| format!("Migration error: {}", e))?;

    println!("✅ Database migrations completed successfully!");

    Ok(())
}

#[cfg(test)]
pub fn create_test_pool() -> DbPool {
    dotenv().ok();

    let database_url = env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://username:password@localhost:5432/marketplace_experiences_test".to_string());

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create test pool.")
}