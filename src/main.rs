use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenvy::dotenv;
use log::info;

mod application;
mod domain;
mod infrastructure;
mod interfaces;
mod shared;

use application::handlers::{
    auth_handler::AuthHandler, booking_handler::BookingHandler, landing_handler::LandingHandler,
    promotion_handler::PromotionHandler,
};
use domain::services::{
    password_service::BcryptPasswordService, token_service::DefaultTokenService,
};
use infrastructure::database::{create_pool, run_migrations};
use infrastructure::repositories::{
    postgres_access_token_repository::PostgresAccessTokenRepository,
    postgres_booking_repository::PostgresBookingRepository,
    postgres_email_verification_repository::PostgresEmailVerificationRepository,
    postgres_landing_repository::PostgresLandingRepository,
    postgres_promotion_repository::PostgresPromotionRepository,
    postgres_session_repository::PostgresSessionRepository,
    postgres_user_repository::PostgresUserRepository,
};
use interfaces::controllers::{
    auth_controller::AuthController, booking_controller::BookingController,
    landing_controller::LandingController, promotion_controller::PromotionController,
};
use interfaces::rest::{
    auth_routes::auth_routes, booking_routes::{booking_routes, user_bookings_routes},
    landing_routes::landing_routes, promotion_routes::promotion_routes,
};

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
    let landing_repository = PostgresLandingRepository::new(pool.clone());
    let booking_repository = PostgresBookingRepository::new(pool.clone());
    let promotion_repository = PostgresPromotionRepository::new(pool.clone());

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
    let landing_handler = LandingHandler::new(landing_repository, None);
    let booking_handler = BookingHandler::new(booking_repository);
    let promotion_handler = PromotionHandler::new(promotion_repository);

    // Initialize controllers
    let auth_controller = AuthController::new(auth_handler);
    let landing_controller = LandingController::new(landing_handler);
    let booking_controller = BookingController::new(booking_handler);
    let promotion_controller = PromotionController::new(promotion_handler);

    // Start web server
    info!("🚀 Starting web server on http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(auth_controller.clone()))
            .app_data(web::Data::new(landing_controller.clone()))
            .app_data(web::Data::new(booking_controller.clone()))
            .app_data(web::Data::new(promotion_controller.clone()))
            .service(
                web::scope("/api/v1")
                    .service(auth_routes())
                    .service(landing_routes())
                    .service(booking_routes())
                    .service(user_bookings_routes())
                    .service(promotion_routes()),
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
