#[derive(Debug, Clone)]
pub enum ErrorType {
    CryptoError,
    MismatchError
}

#[derive(Debug, Clone)]
pub struct SupergenError {
    pub error_type: ErrorType
}

impl std::error::Error for SupergenError {}

impl std::fmt::Display for SupergenError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.error_type {
            ErrorType::CryptoError => write!(f, "An error occuring during password encryption. Please try again."),
            ErrorType::MismatchError => write!(f, "The given passwords did not match. Please try again.")
        }
        
    }
}

impl From<ring::error::Unspecified> for SupergenError {
    fn from(_: ring::error::Unspecified) -> Self {
        SupergenError { error_type: ErrorType::CryptoError }
    }
}