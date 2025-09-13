use actix_web::{web, Scope, HttpRequest, HttpResponse};
use crate::interfaces::controllers::auth_controller::AuthController;
use crate::application::dto::auth_dto::RegisterUserRequest;

pub fn auth_routes() -> Scope {
    web::scope("/auth")
        .route("/register", web::post().to(register_handler))
        .route("/login", web::post().to(login_handler))
        .route("/verify-email", web::post().to(verify_email_handler))
        .route("/forgot-password", web::post().to(forgot_password_handler))
        .route("/reset-password", web::post().to(reset_password_handler))
        .route("/logout", web::post().to(logout_handler))
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

async fn login_handler() -> HttpResponse {
    HttpResponse::NotImplemented().json(serde_json::json!({
        "message": "Login handler not implemented yet"
    }))
}

async fn verify_email_handler() -> HttpResponse {
    HttpResponse::NotImplemented().json(serde_json::json!({
        "message": "Verify email handler not implemented yet"
    }))
}

async fn forgot_password_handler() -> HttpResponse {
    HttpResponse::NotImplemented().json(serde_json::json!({
        "message": "Forgot password handler not implemented yet"
    }))
}

async fn reset_password_handler() -> HttpResponse {
    HttpResponse::NotImplemented().json(serde_json::json!({
        "message": "Reset password handler not implemented yet"
    }))
}

async fn logout_handler() -> HttpResponse {
    HttpResponse::NotImplemented().json(serde_json::json!({
        "message": "Logout handler not implemented yet"
    }))
}
