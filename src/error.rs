use std::fmt;
use argon2::password_hash::Error as ArgonError;

#[derive(Debug)]
pub enum CustomError {
    Argon2Error(ArgonError),
    OtherError(String),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::Argon2Error(e) => write!(f, "Argon2 error: {}", e),
            CustomError::OtherError(e) => write!(f, "Other error: {}", e),
        }
    }
}

impl std::error::Error for CustomError {}

// Optional: Implement From trait for easier error conversion
impl From<ArgonError> for CustomError {
    fn from(err: ArgonError) -> Self {
        CustomError::Argon2Error(err)
    }
}