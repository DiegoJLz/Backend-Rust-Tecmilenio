use actix_web::{web, Scope, HttpRequest, HttpResponse};
use crate::interfaces::controllers::auth_controller::AuthController;
use crate::application::dto::auth_dto::{RegisterUserRequest, VerifyEmailRequest, VerifyEmailQueryParams, LoginRequest, ForgotPasswordRequest, ResetPasswordRequest, ResetPasswordQueryParams, ResetPasswordBody, LogoutRequest, ResendEmailVerificationRequest};

pub fn auth_routes() -> Scope {
    web::scope("/auth")
        .route("/register", web::post().to(register_handler))
        .route("/login", web::post().to(login_handler))
        .route("/verify-email", web::get().to(verify_email_handler))
        .route("/forgot-password", web::post().to(forgot_password_handler))
        .route("/reset-password", web::post().to(reset_password_handler))
        .route("/logout", web::post().to(logout_handler))
        .route("/resend-email-verification", web::post().to(resend_email_verification_handler))
}

async fn register_handler(
    req: HttpRequest,
    data: web::Json<RegisterUserRequest>,
) -> HttpResponse {
    let auth_controller = req.app_data::<web::Data<AuthController>>()
        .expect("AuthController not found in app data");

    auth_controller.register(data).await.unwrap_or_else(|e| {
        HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))
    })
}

async fn login_handler(
    req: HttpRequest,
    data: web::Json<LoginRequest>,
) -> HttpResponse {
    let auth_controller = req.app_data::<web::Data<AuthController>>()
        .expect("AuthController not found in app data");

    // Clone the request to avoid move issues
    let req_clone = req.clone();
    auth_controller.login(data, req_clone).await.unwrap_or_else(|e| {
        HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))
    })
}

async fn verify_email_handler(
    req: HttpRequest,
    query: web::Query<VerifyEmailQueryParams>,
) -> HttpResponse {
    let auth_controller = req.app_data::<web::Data<AuthController>>()
        .expect("AuthController not found in app data");

    auth_controller.verify_email(query).await.unwrap_or_else(|e| {
        HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))
    })
}

async fn forgot_password_handler(
    req: HttpRequest,
    data: web::Json<ForgotPasswordRequest>,
) -> HttpResponse {
    let auth_controller = req.app_data::<web::Data<AuthController>>()
        .expect("AuthController not found in app data");

    auth_controller.forgot_password(data).await.unwrap_or_else(|e| {
        HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))
    })
}

async fn reset_password_handler(
    req: HttpRequest,
    query: web::Query<ResetPasswordQueryParams>,
    body: web::Json<ResetPasswordBody>,
) -> HttpResponse {
    let auth_controller = req.app_data::<web::Data<AuthController>>()
        .expect("AuthController not found in app data");

    auth_controller.reset_password(query, body).await.unwrap_or_else(|e| {
        HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))
    })
}

async fn logout_handler(
    req: HttpRequest,
    data: web::Json<LogoutRequest>,
) -> HttpResponse {
    let auth_controller = req.app_data::<web::Data<AuthController>>()
        .expect("AuthController not found in app data");

    auth_controller.logout(data).await.unwrap_or_else(|e| {
        HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))
    })
}

async fn resend_email_verification_handler(
    req: HttpRequest,
    data: web::Json<ResendEmailVerificationRequest>,
) -> HttpResponse {
    let auth_controller = req.app_data::<web::Data<AuthController>>()
        .expect("AuthController not found in app data");

    auth_controller.resend_email_verification(data).await.unwrap_or_else(|e| {
        HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.to_string()
        }))
    })
}
