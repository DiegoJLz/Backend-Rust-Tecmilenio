use dotenvy::dotenv;
use log::info;

mod domain;
mod application;
mod infrastructure;
mod interfaces;
mod shared;

use infrastructure::database::{create_pool, run_migrations};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv().ok();

    // Initialize logger
    env_logger::init();

    info!("🚀 Starting Marketplace de Experiencias Locales Backend...");

    // Run database migrations
    match run_migrations() {
        Ok(_) => info!("✅ Database migrations completed successfully"),
        Err(e) => {
            eprintln!("❌ Failed to run migrations: {}", e);
            return Err(e);
        }
    }

    // Create database pool
    match create_pool() {
        Ok(_pool) => info!("✅ Database connection pool created successfully"),
        Err(e) => {
            eprintln!("❌ Failed to create database pool: {}", e);
            return Err(e);
        }
    }

    info!("🎉 Backend initialized successfully!");
    info!("📊 Database schema ready for Marketplace de Experiencias Locales");

    // TODO: Start web server here
    println!("¡Backend Rust funcionando correctamente!");
    println!("El programa se ejecutó sin errores.");

    Ok(())
}
