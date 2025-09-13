use crate::application::{
    dto::auth_dto::{RegisterUserRequest, RegisterUserResponse},
    use_cases::register_user_use_case::RegisterUserUseCase,
};
use crate::infrastructure::repositories::{
    postgres_user_repository::PostgresUserRepository,
    postgres_email_verification_repository::PostgresEmailVerificationRepository,
};
use crate::domain::{
    repositories::{user_repository::UserRepository, email_verification_repository::EmailVerificationRepository},
    services::{
        password_service::{PasswordService, BcryptPasswordService},
        token_service::{TokenService, DefaultTokenService},
    },
};
use crate::shared::error_types::ApiError;

#[derive(Clone)]
pub struct AuthHandler {
    register_user_use_case: RegisterUserUseCase,
}

impl AuthHandler {
    pub fn new(
        user_repository: PostgresUserRepository,
        email_verification_repository: PostgresEmailVerificationRepository,
        password_service: BcryptPasswordService,
        token_service: DefaultTokenService,
    ) -> Self {
        let register_user_use_case = RegisterUserUseCase::new(
            user_repository,
            email_verification_repository,
            password_service,
            token_service,
        );

        Self {
            register_user_use_case,
        }
    }

    pub async fn register_user(&self, request: RegisterUserRequest) -> Result<RegisterUserResponse, ApiError> {
        self.register_user_use_case.execute(request).await
    }
}
