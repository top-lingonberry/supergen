#[derive(Debug, Clone)]
pub struct MismatchError;

impl std::fmt::Display for MismatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "The entered passwords do not match.")
    }
}

impl std::error::Error for MismatchError {}

#[derive(Debug, Clone)]
pub struct HashingError;

impl std::fmt::Display for HashingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "An unspecified error occured while encrypting the password.")
    }
}

impl std::error::Error for HashingError {}