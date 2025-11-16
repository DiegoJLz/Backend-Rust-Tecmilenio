use crate::shared::error_types::{ApiError, ERROR_VALIDATION_ERROR};
use regex::Regex;

pub struct ValidationUtils;

impl ValidationUtils {
    /// Validates email format
    pub fn validate_email(email: &str) -> Result<(), ApiError> {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
            .map_err(|_| ApiError::new(ERROR_VALIDATION_ERROR, "Invalid email regex pattern"))?;

        if email.is_empty() {
            return Err(ApiError::new(ERROR_VALIDATION_ERROR, "Email is required"));
        }

        if !email_regex.is_match(email) {
            return Err(ApiError::new(
                ERROR_VALIDATION_ERROR,
                "Invalid email format",
            ));
        }

        if email.len() > 255 {
            return Err(ApiError::new(ERROR_VALIDATION_ERROR, "Email is too long"));
        }

        Ok(())
    }

    /// Validates password strength
    pub fn validate_password(password: &str) -> Result<(), ApiError> {
        if password.is_empty() {
            return Err(ApiError::new(
                ERROR_VALIDATION_ERROR,
                "Password is required",
            ));
        }

        if password.len() < 8 {
            return Err(ApiError::new(
                ERROR_VALIDATION_ERROR,
                "Password must be at least 8 characters long",
            ));
        }

        if password.len() > 128 {
            return Err(ApiError::new(
                ERROR_VALIDATION_ERROR,
                "Password is too long",
            ));
        }

        // Check for at least one uppercase letter
        if !password.chars().any(|c| c.is_uppercase()) {
            return Err(ApiError::new(
                ERROR_VALIDATION_ERROR,
                "Password must contain at least one uppercase letter",
            ));
        }

        // Check for at least one lowercase letter
        if !password.chars().any(|c| c.is_lowercase()) {
            return Err(ApiError::new(
                ERROR_VALIDATION_ERROR,
                "Password must contain at least one lowercase letter",
            ));
        }

        // Check for at least one digit
        if !password.chars().any(|c| c.is_numeric()) {
            return Err(ApiError::new(
                ERROR_VALIDATION_ERROR,
                "Password must contain at least one number",
            ));
        }

        // Check for at least one special character
        let special_chars = "!@#$%^&*()_+-=[]{}|;:,.<>?";
        if !password.chars().any(|c| special_chars.contains(c)) {
            return Err(ApiError::new(
                ERROR_VALIDATION_ERROR,
                "Password must contain at least one special character",
            ));
        }

        Ok(())
    }

    /// Validates username format
    pub fn validate_username(username: &str) -> Result<(), ApiError> {
        if username.is_empty() {
            return Err(ApiError::new(
                ERROR_VALIDATION_ERROR,
                "Username is required",
            ));
        }

        if username.len() < 3 {
            return Err(ApiError::new(
                ERROR_VALIDATION_ERROR,
                "Username must be at least 3 characters long",
            ));
        }

        if username.len() > 50 {
            return Err(ApiError::new(
                ERROR_VALIDATION_ERROR,
                "Username is too long",
            ));
        }

        // Username should only contain alphanumeric characters and underscores
        let username_regex = Regex::new(r"^[a-zA-Z0-9_]+$")
            .map_err(|_| ApiError::new(ERROR_VALIDATION_ERROR, "Invalid username regex pattern"))?;

        if !username_regex.is_match(username) {
            return Err(ApiError::new(
                ERROR_VALIDATION_ERROR,
                "Username can only contain letters, numbers, and underscores",
            ));
        }

        Ok(())
    }

    /// Validates name fields
    pub fn validate_name(name: &str, field_name: &str) -> Result<(), ApiError> {
        if name.is_empty() {
            return Err(ApiError::new(
                ERROR_VALIDATION_ERROR,
                &format!("{} is required", field_name),
            ));
        }

        if name.len() < 2 {
            return Err(ApiError::new(
                ERROR_VALIDATION_ERROR,
                &format!("{} must be at least 2 characters long", field_name),
            ));
        }

        if name.len() > 100 {
            return Err(ApiError::new(
                ERROR_VALIDATION_ERROR,
                &format!("{} is too long", field_name),
            ));
        }

        // Name should only contain letters (including accented characters), spaces, hyphens, and apostrophes
        let name_regex = Regex::new(r"^[\p{L}\s\-']+$")
            .map_err(|_| ApiError::new(ERROR_VALIDATION_ERROR, "Invalid name regex pattern"))?;

        if !name_regex.is_match(name) {
            return Err(ApiError::new(ERROR_VALIDATION_ERROR, &format!("{} can only contain letters (including accented characters), spaces, hyphens, and apostrophes", field_name)));
        }

        Ok(())
    }

    /// Validates phone number format
    pub fn validate_phone(phone: &str) -> Result<(), ApiError> {
        if phone.is_empty() {
            return Ok(()); // Phone is optional
        }

        // Remove all non-digit characters for validation
        let digits_only: String = phone.chars().filter(|c| c.is_numeric()).collect();

        if digits_only.len() < 10 {
            return Err(ApiError::new(
                ERROR_VALIDATION_ERROR,
                "Phone number must have at least 10 digits",
            ));
        }

        if digits_only.len() > 15 {
            return Err(ApiError::new(
                ERROR_VALIDATION_ERROR,
                "Phone number is too long",
            ));
        }

        Ok(())
    }

    /// Validates that two passwords match
    pub fn validate_password_confirmation(
        password: &str,
        confirm_password: &str,
    ) -> Result<(), ApiError> {
        if password != confirm_password {
            return Err(ApiError::new(
                ERROR_VALIDATION_ERROR,
                "Passwords do not match",
            ));
        }

        Ok(())
    }
}
