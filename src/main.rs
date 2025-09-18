use dotenvy::dotenv;
use log::info;
use actix_web::{web, App, HttpServer, middleware::Logger};

mod domain;
mod application;
mod infrastructure;
mod interfaces;
mod shared;

use infrastructure::database::{create_pool, run_migrations};
use infrastructure::repositories::{
    postgres_user_repository::PostgresUserRepository,
    postgres_email_verification_repository::PostgresEmailVerificationRepository,
    postgres_session_repository::PostgresSessionRepository,
    postgres_access_token_repository::PostgresAccessTokenRepository,
};
use domain::services::{
    password_service::BcryptPasswordService,
    token_service::DefaultTokenService,
};
use application::handlers::auth_handler::AuthHandler;
use interfaces::controllers::auth_controller::AuthController;
use interfaces::rest::auth_routes::auth_routes;

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
    let pool = match create_pool() {
        Ok(pool) => {
            info!("✅ Database connection pool created successfully");
            pool
        }
        Err(e) => {
            eprintln!("❌ Failed to create database pool: {}", e);
            return Err(e);
        }
    };

    info!("🎉 Backend initialized successfully!");
    info!("📊 Database schema ready for Marketplace de Experiencias Locales");

    // Initialize repositories
    let user_repository = PostgresUserRepository::new(pool.clone());
    let email_verification_repository = PostgresEmailVerificationRepository::new(pool.clone());
    let session_repository = PostgresSessionRepository::new(pool.clone());
    let access_token_repository = PostgresAccessTokenRepository::new(pool.clone());

    // Initialize services
    let password_service = BcryptPasswordService;
    let token_service = DefaultTokenService;

    // Initialize handlers
    let auth_handler = AuthHandler::new(
        user_repository,
        email_verification_repository,
        session_repository,
        access_token_repository,
        password_service,
        token_service,
    )?;

    // Initialize controllers
    let auth_controller = AuthController::new(auth_handler);

    // Start web server
    info!("🚀 Starting web server on http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(auth_controller.clone()))
            .service(
                web::scope("/api/v1")
                    .service(auth_routes())
            )
            .route("/health", web::get().to(health_check))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}

async fn health_check() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "Marketplace de Experiencias Locales Backend",
        "version": "0.1.0"
    }))
}
