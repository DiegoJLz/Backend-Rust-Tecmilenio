use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::Connection;
use diesel_migrations::{FileBasedMigrations, MigrationHarness};
use dotenvy::dotenv;
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
// pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn create_pool() -> Result<DbPool, Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    Ok(pool)
}

pub fn run_migrations() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut conn = PgConnection::establish(&database_url)?;

    // Load migrations from the migrations directory
    let migrations = FileBasedMigrations::from_path("migrations")?;

    // Run migrations
    conn.run_pending_migrations(migrations)
        .map_err(|e| format!("Migration error: {}", e))?;

    println!("✅ Database migrations completed successfully!");

    Ok(())
}
