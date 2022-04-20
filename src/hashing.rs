// use data_encoding::HEXUPPER;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;

use crate::errors::{ErrorType, SupergenError};

const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;

// Takes two password inputs and returns Ok if the password matches. Error if not.
pub fn check_matches(entry1: String, entry2: &str)
                            -> Result<&str, Box< dyn std::error::Error>> {
    
    match entry1.eq(&entry2) {
        true => Ok(entry2),
        false => Err(Box::new(
            SupergenError {
                error_type: ErrorType::MismatchError
            }
        ))
    }
}

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
pub fn encrypt_password(plain_text: &str)
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

    // Get rid of this, this is just a reminder to verify the password through bytes.
    // let should_succeed = pbkdf2::verify(
    //     pbkdf2::PBKDF2_HMAC_SHA512,
    //     n_iter,
    //     &salt,
    //     password.as_bytes(),
    //     &pbkdf2_hash,
    // );

    // Get rid of these when storing the data too. And remove the data_encoding crate.
    // The password can/should be stored and verified in raw bytes only.
    // let salt: String = HEXUPPER.encode(&salt);
    // let password_hash: String = HEXUPPER.encode(&pbkdf2_hash);

    Ok((salt, pbkdf2_hash))
}