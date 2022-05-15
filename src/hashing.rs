use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;

use crate::errors::SupergenError;

const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;

// Generate the salt for the password hash.
// Seperate function to handle ring::error::Unspecified.
fn get_salt() -> Result<[u8; 32], SupergenError> {
    let rng = rand::SystemRandom::new();

    let mut salt = [0u8; CREDENTIAL_LEN];
    let _fill = rng.fill(&mut salt)?;
    Ok(salt)
}

// New function here to hash password on fill salt. Return the tuple to hash_password.
// Implemented purely to handle the "Unspecified" error from the ring crate.
pub fn encrypt_password(plain_text: String)
                    -> Result<([u8; 32], [u8; 32]), Box<dyn std::error::Error>> {
    
    let n_iter = NonZeroU32::new(100_000).unwrap();

    let salt = get_salt()?;

    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        n_iter,
        &salt,
        plain_text.as_bytes(),
        &mut pbkdf2_hash,
    );

    // Return the salt and the password hash.
    Ok((salt, pbkdf2_hash))
}