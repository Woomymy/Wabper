use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand::{rngs::OsRng, Rng};

use crate::Error;
// All chars allowed in url IG
pub const ID_LEN: u8 = 8; // Lenght of paste ID
pub const REVOKE_LEN: u8 = 31;
pub const ID_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
/// Generated valid paste ID
pub fn gen_id() -> String {
    let mut rng = rand::thread_rng();
    // Generate 9 chars
    (0..=ID_LEN)
        .map(|_| ID_CHARSET[rng.gen_range(0..ID_CHARSET.len())] as char)
        .collect()
}
/// Generate deletion pw
pub fn gen_deletion_pw() -> String {
    let mut rng = rand::thread_rng();

    (0..=REVOKE_LEN)
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
