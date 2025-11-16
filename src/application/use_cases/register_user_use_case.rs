use uuid::Uuid;

use crate::application::dto::auth_dto::{RegisterUserRequest, RegisterUserResponse, UserDto};
use crate::domain::entities::{email_verification_token::EmailVerificationToken, user::User};
use crate::domain::{
    repositories::{
        email_verification_repository::EmailVerificationRepository, user_repository::UserRepository,
    },
    services::{
        password_service::{BcryptPasswordService, PasswordService},
        token_service::{DefaultTokenService, TokenService},
    },
};
use crate::infrastructure::repositories::{
    postgres_email_verification_repository::PostgresEmailVerificationRepository,
    postgres_user_repository::PostgresUserRepository,
};
use crate::shared::error_types::{ApiError, ERROR_USER_ALREADY_EXISTS, ERROR_VALIDATION_ERROR};
use crate::shared::validation_utils::ValidationUtils;

#[derive(Clone)]
pub struct RegisterUserUseCase {
    user_repository: PostgresUserRepository,
    email_verification_repository: PostgresEmailVerificationRepository,
    password_service: BcryptPasswordService,
    token_service: DefaultTokenService,
    send_email_use_case: crate::application::use_cases::send_email_use_case::SendEmailUseCase,
}

impl RegisterUserUseCase {
    pub fn new(
        user_repository: PostgresUserRepository,
        email_verification_repository: PostgresEmailVerificationRepository,
        password_service: BcryptPasswordService,
        token_service: DefaultTokenService,
        send_email_use_case: crate::application::use_cases::send_email_use_case::SendEmailUseCase,
    ) -> Self {
        Self {
            user_repository,
            email_verification_repository,
            password_service,
            token_service,
            send_email_use_case,
        }
    }

    pub async fn execute(
        &self,
        request: RegisterUserRequest,
    ) -> Result<RegisterUserResponse, ApiError> {
        // Validate input
        self.validate_request(&request)?;

        // Check if email already exists
        if self.user_repository.email_exists(&request.email).await? {
            return Err(ApiError::new(
                ERROR_USER_ALREADY_EXISTS,
                "Email already exists",
            ));
        }

        // Generate username from email (simple approach)
        let username = self.generate_username_from_email(&request.email)?;

        // Check if username already exists
        if self.user_repository.username_exists(&username).await? {
            return Err(ApiError::new(
                ERROR_USER_ALREADY_EXISTS,
                "Username already exists",
            ));
        }

        // Hash password
        let password_hash = self
            .password_service
            .hash_password(&request.password)
            .await?;

        // Clone values before moving them
        let first_name = request.first_name.clone();
        let last_name = request.last_name.clone();
        let email = request.email.clone();

        // Create user entity
        let user_id = Uuid::new_v4();
        let user = User::new(
            user_id,
            request.email,
            username,
            request.first_name,
            request.last_name,
            request.phone,
            password_hash.clone(),
        );

        // Save user to database
        let created_user = self.user_repository.create(&user, &password_hash).await?;

        // Generate email verification JWT token
        let full_name = format!("{} {}", first_name, last_name);
        let verification_token = self
            .token_service
            .generate_email_verification_jwt(user_id, email.clone(), full_name.clone())
            .await?;

        // Parse expiration from JWT (24 hours from now)
        let expires_at = chrono::Utc::now() + chrono::Duration::hours(24);
        let email_verification = EmailVerificationToken {
            id: uuid::Uuid::new_v4(),
            user_id,
            token: verification_token,
            expires_at,
            is_used: Some(false),
            created_at: Some(chrono::Utc::now()),
        };

        // Save verification token
        self.email_verification_repository
            .create(&email_verification)
            .await?;

        // Send verification email
        if let Err(email_error) = self
            .send_email_use_case
            .send_email_verification(
                email.clone(),
                full_name.clone(),
                email_verification.token.clone(),
            )
            .await
        {
            // Log the error but don't fail the registration
            println!(
                "Failed to send verification email to {}: {}",
                email, email_error
            );
        }

        // Convert to DTO
        let user_dto = UserDto::from_domain(&created_user);

        Ok(RegisterUserResponse {
            user: user_dto,
            message: "User registered successfully. Please check your email for verification."
                .to_string(),
        })
    }

    fn validate_request(&self, request: &RegisterUserRequest) -> Result<(), ApiError> {
        // Validate first name
        ValidationUtils::validate_name(&request.first_name, "First name")?;

        // Validate last name
        ValidationUtils::validate_name(&request.last_name, "Last name")?;

        // Validate email
        ValidationUtils::validate_email(&request.email)?;

        // Validate password
        ValidationUtils::validate_password(&request.password)?;

        // Validate password confirmation
        ValidationUtils::validate_password_confirmation(
            &request.password,
            &request.confirm_password,
        )?;

        // Validate phone if provided
        if let Some(phone) = &request.phone {
            ValidationUtils::validate_phone(phone)?;
        }

        Ok(())
    }

    fn generate_username_from_email(&self, email: &str) -> Result<String, ApiError> {
        let local_part = email.split('@').next().unwrap_or("");

        if local_part.is_empty() {
            return Err(ApiError::new(
                ERROR_VALIDATION_ERROR,
                "Invalid email format",
            ));
        }

        // Clean the username (remove special characters, keep only alphanumeric)
        let clean_username: String = local_part.chars().filter(|c| c.is_alphanumeric()).collect();

        if clean_username.len() < 3 {
            return Err(ApiError::new(
                ERROR_VALIDATION_ERROR,
                "Email local part too short for username",
            ));
        }

        // Add random suffix to make it unique
        let random_suffix = Uuid::new_v4().to_string()[0..8].to_string();
        let username = format!("{}_{}", clean_username, random_suffix);

        Ok(username)
    }
}
