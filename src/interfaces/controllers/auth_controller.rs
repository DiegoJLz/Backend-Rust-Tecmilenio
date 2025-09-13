use actix_web::{web, HttpResponse, Result};
use serde_json::json;

use crate::application::{
    dto::auth_dto::RegisterUserRequest,
    handlers::auth_handler::AuthHandler,
};
use crate::shared::response_types::ApiResponse;

#[derive(Clone)]
pub struct AuthController {
    auth_handler: AuthHandler,
}

impl AuthController {
    pub fn new(auth_handler: AuthHandler) -> Self {
        Self { auth_handler }
    }

    pub async fn register(&self, request: web::Json<RegisterUserRequest>) -> Result<HttpResponse> {
        match self.auth_handler.register_user(request.into_inner()).await {
            Ok(response) => {
                let api_response = ApiResponse::success_with_message(response, "User registered successfully");
                Ok(HttpResponse::Created().json(api_response))
            }
            Err(error) => {
                let api_response: ApiResponse<()> = ApiResponse::error(error);
                Ok(HttpResponse::BadRequest().json(api_response))
            }
        }
    }

    pub async fn login(&self, _request: web::Json<serde_json::Value>) -> Result<HttpResponse> {
        // TODO: Implement login
        let response = json!({
            "message": "Login endpoint not implemented yet"
        });
        Ok(HttpResponse::NotImplemented().json(response))
    }

    pub async fn verify_email(&self, _request: web::Json<serde_json::Value>) -> Result<HttpResponse> {
        // TODO: Implement email verification
        let response = json!({
            "message": "Email verification endpoint not implemented yet"
        });
        Ok(HttpResponse::NotImplemented().json(response))
    }

    pub async fn forgot_password(&self, _request: web::Json<serde_json::Value>) -> Result<HttpResponse> {
        // TODO: Implement forgot password
        let response = json!({
            "message": "Forgot password endpoint not implemented yet"
        });
        Ok(HttpResponse::NotImplemented().json(response))
    }

    pub async fn reset_password(&self, _request: web::Json<serde_json::Value>) -> Result<HttpResponse> {
        // TODO: Implement reset password
        let response = json!({
            "message": "Reset password endpoint not implemented yet"
        });
        Ok(HttpResponse::NotImplemented().json(response))
    }

    pub async fn logout(&self, _request: web::Json<serde_json::Value>) -> Result<HttpResponse> {
        // TODO: Implement logout
        let response = json!({
            "message": "Logout endpoint not implemented yet"
        });
        Ok(HttpResponse::NotImplemented().json(response))
    }
}
