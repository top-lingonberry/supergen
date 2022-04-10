// use data_encoding::HEXUPPER;
use ring::error::Unspecified;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;

// Takes a plaintext password &str and returns a tuple of the randomly generated
// salt and the password hash.
pub fn hash_pasword(plaintext: &str) -> Result<([u8; 32], [u8; 32]), Unspecified> {
    const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
    let n_iter = NonZeroU32::new(100_000).unwrap();
    let rng = rand::SystemRandom::new();

    let mut salt = [0u8; CREDENTIAL_LEN];
    rng.fill(&mut salt)?;

    let password = plaintext;
    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        n_iter,
        &salt,
        password.as_bytes(),
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