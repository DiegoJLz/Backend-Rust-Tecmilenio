use crate::application::{
    dto::auth_dto::{RegisterUserRequest, RegisterUserResponse, VerifyEmailRequest, VerifyEmailResponse, LoginRequest, LoginResponse, ForgotPasswordRequest, ForgotPasswordResponse, ResetPasswordRequest, ResetPasswordResponse, LogoutRequest, LogoutResponse, ResendEmailVerificationRequest, ResendEmailVerificationResponse},
    use_cases::{register_user_use_case::RegisterUserUseCase, verify_email_use_case::VerifyEmailUseCase, login_user_use_case::LoginUserUseCase, forgot_password_use_case::ForgotPasswordUseCase, reset_password_use_case::ResetPasswordUseCase, logout_use_case::LogoutUseCase, resend_email_verification_use_case::ResendEmailVerificationUseCase, send_email_use_case::SendEmailUseCase},
};
use crate::infrastructure::repositories::{
    postgres_user_repository::PostgresUserRepository,
    postgres_email_verification_repository::PostgresEmailVerificationRepository,
    postgres_session_repository::PostgresSessionRepository,
    postgres_access_token_repository::PostgresAccessTokenRepository,
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
    verify_email_use_case: VerifyEmailUseCase,
    login_user_use_case: LoginUserUseCase,
    forgot_password_use_case: ForgotPasswordUseCase,
    reset_password_use_case: ResetPasswordUseCase,
    logout_use_case: LogoutUseCase,
    resend_email_verification_use_case: ResendEmailVerificationUseCase,
}

impl AuthHandler {
    pub fn new(
        user_repository: PostgresUserRepository,
        email_verification_repository: PostgresEmailVerificationRepository,
        session_repository: PostgresSessionRepository,
        access_token_repository: PostgresAccessTokenRepository,
        password_service: BcryptPasswordService,
        token_service: DefaultTokenService,
    ) -> Result<Self, ApiError> {
        let send_email_use_case = SendEmailUseCase::new(
            crate::infrastructure::services::resend_official_service::ResendOfficialService::new()
                .map_err(|e| {
                    eprintln!("❌ Error inicializando Resend Official: {}", e);
                    e
                })?,
        );

        let register_user_use_case = RegisterUserUseCase::new(
            user_repository.clone(),
            email_verification_repository.clone(),
            password_service.clone(),
            token_service.clone(),
            send_email_use_case.clone(),
        );

        let verify_email_use_case = VerifyEmailUseCase::new(
            user_repository.clone(),
            email_verification_repository.clone(),
            token_service.clone(),
        );

        let login_user_use_case = LoginUserUseCase::new(
            user_repository.clone(),
            session_repository.clone(),
            access_token_repository.clone(),
            password_service.clone(),
            token_service.clone(),
        );

        let forgot_password_use_case = ForgotPasswordUseCase::new(
            user_repository.clone(),
            access_token_repository.clone(),
            token_service.clone(),
            send_email_use_case.clone(),
        );

        let reset_password_use_case = ResetPasswordUseCase::new(
            user_repository.clone(),
            access_token_repository.clone(),
            password_service.clone(),
            token_service.clone(),
        );

        let logout_use_case = LogoutUseCase::new(
            session_repository.clone(),
            access_token_repository.clone(),
        );

        let resend_email_verification_use_case = ResendEmailVerificationUseCase::new(
            user_repository,
            email_verification_repository,
            token_service,
            send_email_use_case,
        );

        Ok(Self {
            register_user_use_case,
            verify_email_use_case,
            login_user_use_case,
            forgot_password_use_case,
            reset_password_use_case,
            logout_use_case,
            resend_email_verification_use_case,
        })
    }

    pub async fn register_user(&self, request: RegisterUserRequest) -> Result<RegisterUserResponse, ApiError> {
        self.register_user_use_case.execute(request).await
    }

    pub async fn verify_email(&self, request: VerifyEmailRequest) -> Result<VerifyEmailResponse, ApiError> {
        let user = self.verify_email_use_case.execute(request.token).await?;

        let user_dto = crate::application::dto::auth_dto::UserDto::from_domain(&user);

        Ok(VerifyEmailResponse {
            message: "Email verified successfully".to_string(),
            user: user_dto,
        })
    }

    pub async fn login_user(&self, request: LoginRequest, ip_address: Option<String>, user_agent: Option<String>) -> Result<LoginResponse, ApiError> {
        let (user, access_token, session_token) = self.login_user_use_case.execute(request.email, request.password, ip_address, user_agent).await?;

        let user_dto = crate::application::dto::auth_dto::UserDto::from_domain(&user);

        Ok(LoginResponse {
            user: user_dto,
            access_token,
            token_type: "Bearer".to_string(),
            expires_in: 86400, // 24 hours in seconds
            session_token: Some(session_token),
        })
    }

    pub async fn forgot_password(&self, request: ForgotPasswordRequest) -> Result<ForgotPasswordResponse, ApiError> {
        let token = self.forgot_password_use_case.execute(request.email).await?;

        Ok(ForgotPasswordResponse {
            message: "Password reset token sent to your email".to_string(),
            token: Some(token), // En producción, esto no debería incluirse
        })
    }

    pub async fn reset_password(&self, request: ResetPasswordRequest) -> Result<ResetPasswordResponse, ApiError> {
        let user = self.reset_password_use_case.execute(request.token, request.new_password, request.confirm_password).await?;

        Ok(ResetPasswordResponse {
            message: "Password reset successfully".to_string(),
            user: crate::application::dto::auth_dto::UserDto::from_domain(&user),
        })
    }

    pub async fn logout(&self, request: LogoutRequest) -> Result<LogoutResponse, ApiError> {
        self.logout_use_case.execute(request.session_token).await?;

        Ok(LogoutResponse {
            message: "Logged out successfully".to_string(),
        })
    }

    pub async fn resend_email_verification(&self, request: ResendEmailVerificationRequest) -> Result<ResendEmailVerificationResponse, ApiError> {
        let token = self.resend_email_verification_use_case.execute(request.email).await?;

        Ok(ResendEmailVerificationResponse {
            message: "Email verification token resent successfully".to_string(),
            token: Some(token), // En producción, esto no debería incluirse
        })
    }
}
