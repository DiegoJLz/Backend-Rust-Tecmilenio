use actix_web::{web, HttpResponse, Result};

use crate::application::{
    dto::auth_dto::{RegisterUserRequest, VerifyEmailRequest, VerifyEmailQueryParams, LoginRequest, ForgotPasswordRequest, ResetPasswordRequest, ResetPasswordQueryParams, ResetPasswordBody, LogoutRequest, ResendEmailVerificationRequest},
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

    pub async fn login(&self, request: web::Json<LoginRequest>, req: actix_web::HttpRequest) -> Result<HttpResponse> {
        let ip_address = req.connection_info().peer_addr().map(|s| s.to_string());
        let user_agent = req.headers().get("User-Agent")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());

        match self.auth_handler.login_user(request.into_inner(), ip_address, user_agent).await {
            Ok(response) => {
                let api_response = ApiResponse::success_with_message(response, "Login successful");
                Ok(HttpResponse::Ok().json(api_response))
            }
            Err(error) => {
                let api_response: ApiResponse<()> = ApiResponse::error(error);
                Ok(HttpResponse::Unauthorized().json(api_response))
            }
        }
    }

    pub async fn verify_email(&self, query: web::Query<VerifyEmailQueryParams>) -> Result<HttpResponse> {
        let request = VerifyEmailRequest {
            token: query.token.clone(),
        };

        match self.auth_handler.verify_email(request).await {
            Ok(response) => {
                let api_response = ApiResponse::success_with_message(response, "Email verified successfully");
                Ok(HttpResponse::Ok().json(api_response))
            }
            Err(error) => {
                let api_response: ApiResponse<()> = ApiResponse::error(error);
                Ok(HttpResponse::BadRequest().json(api_response))
            }
        }
    }

    pub async fn forgot_password(&self, request: web::Json<ForgotPasswordRequest>) -> Result<HttpResponse> {
        match self.auth_handler.forgot_password(request.into_inner()).await {
            Ok(response) => {
                let api_response = ApiResponse::success_with_message(response, "Password reset token sent");
                Ok(HttpResponse::Ok().json(api_response))
            }
            Err(error) => {
                let api_response: ApiResponse<()> = ApiResponse::error(error);
                Ok(HttpResponse::BadRequest().json(api_response))
            }
        }
    }

    pub async fn reset_password(&self, query: web::Query<ResetPasswordQueryParams>, body: web::Json<ResetPasswordBody>) -> Result<HttpResponse> {
        let request = ResetPasswordRequest {
            token: query.token.clone(),
            new_password: body.new_password.clone(),
            confirm_password: body.confirm_password.clone(),
        };

        match self.auth_handler.reset_password(request).await {
            Ok(response) => {
                let api_response = ApiResponse::success_with_message(response, "Password reset successfully");
                Ok(HttpResponse::Ok().json(api_response))
            }
            Err(error) => {
                let api_response: ApiResponse<()> = ApiResponse::error(error);
                Ok(HttpResponse::BadRequest().json(api_response))
            }
        }
    }

    pub async fn logout(&self, request: web::Json<LogoutRequest>) -> Result<HttpResponse> {
        match self.auth_handler.logout(request.into_inner()).await {
            Ok(response) => {
                let api_response = ApiResponse::success_with_message(response, "Logged out successfully");
                Ok(HttpResponse::Ok().json(api_response))
            }
            Err(error) => {
                let api_response: ApiResponse<()> = ApiResponse::error(error);
                Ok(HttpResponse::BadRequest().json(api_response))
            }
        }
    }

    pub async fn resend_email_verification(&self, request: web::Json<ResendEmailVerificationRequest>) -> Result<HttpResponse> {
        match self.auth_handler.resend_email_verification(request.into_inner()).await {
            Ok(response) => {
                let api_response = ApiResponse::success_with_message(response, "Email verification token resent successfully");
                Ok(HttpResponse::Ok().json(api_response))
            }
            Err(error) => {
                let api_response: ApiResponse<()> = ApiResponse::error(error);
                Ok(HttpResponse::BadRequest().json(api_response))
            }
        }
    }
}
