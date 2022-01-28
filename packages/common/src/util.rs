use std::time::Duration;

use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand::{rngs::OsRng, Rng};

use crate::Error;

/// Timeout for DB connection
pub const DB_CONNECTION_TIMEOUT: Duration = Duration::from_secs(1);

/// Lenght of paste ID
pub const ID_LEN: u8 = 10;

/// Lenght of deletion password
pub const DELETEPW_LEN: u8 = 32;

/// All chars that a ID can contain
pub const ID_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

/// Generated valid paste ID
pub fn gen_id() -> String {
    let mut rng = rand::thread_rng();
    // Generate random chars
    (0..ID_LEN)
        .map(|_| ID_CHARSET[rng.gen_range(0..ID_CHARSET.len())] as char)
        .collect()
}

/// Generate deletion pw
pub fn gen_deletion_pw() -> String {
    let mut rng = rand::thread_rng();

    (0..DELETEPW_LEN)
        .map(|_| ID_CHARSET[rng.gen_range(0..ID_CHARSET.len())] as char)
        .collect()
}

/// Hash string using argon2
pub fn hash_string(input: String) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    Ok(argon2.hash_password(input.as_bytes(), &salt)?.to_string())
}

/// Verify argon2 hash of password
pub fn check_password(password: String, hash: String) -> Result<bool, Error> {
    Ok(Argon2::default()
        .verify_password(&password.as_bytes(), &PasswordHash::new(&hash)?)
        .is_ok())
}
