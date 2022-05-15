#[derive(Debug, Clone)]
pub enum ErrorType {
    CryptoError,
    MismatchError,
    InvalidUsernameError
}

#[derive(Debug, Clone)]
pub struct SupergenError {
    pub error_type: ErrorType
}

impl std::error::Error for SupergenError {}

impl std::fmt::Display for SupergenError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.error_type {
            ErrorType::CryptoError => write!(f, "An unknown error occuring during password encryption."),
            ErrorType::MismatchError => write!(f, "The given passwords did not match. Please try again."),
            ErrorType::InvalidUsernameError => write!(f, "The username cannot exceed 320 characters. Please try again.")
        }
        
    }
}

impl From<ring::error::Unspecified> for SupergenError {
    fn from(_: ring::error::Unspecified) -> Self {
        SupergenError { error_type: ErrorType::CryptoError }
    }
}