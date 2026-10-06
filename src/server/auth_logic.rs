use argon2::{password_hash::PasswordHash, Argon2, PasswordVerifier};
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, PartialEq, Eq)]
pub enum PasswordCheckError {
    InvalidHash,
    Mismatch,
}

pub fn verify_password(password: &str, encoded_hash: &str) -> Result<(), PasswordCheckError> {
    let hash = PasswordHash::new(encoded_hash).map_err(|_| PasswordCheckError::InvalidHash)?;
    Argon2::default()
        .verify_password(password.as_bytes(), &hash)
        .map_err(|_| PasswordCheckError::Mismatch)
}

pub fn create_session_token(expiry: u64, secret: &str) -> Option<String> {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).ok()?;
    let expiry = expiry.to_string();
    mac.update(expiry.as_bytes());
    Some(format!(
        "{expiry}.{}",
        hex::encode(mac.finalize().into_bytes())
    ))
}

pub fn session_token_is_valid(token: &str, secret: &str, now: u64) -> bool {
    let Some((expiry, signature)) = token.split_once('.') else {
        return false;
    };
    let Ok(expiry) = expiry.parse::<u64>() else {
        return false;
    };
    if expiry <= now {
        return false;
    }

    let Ok(signature) = hex::decode(signature) else {
        return false;
    };
    let Ok(mut mac) = HmacSha256::new_from_slice(secret.as_bytes()) else {
        return false;
    };
    mac.update(expiry.to_string().as_bytes());
    mac.verify_slice(&signature).is_ok()
}

pub fn session_cookie_is_valid(cookie_header: &str, secret: &str, now: u64) -> bool {
    let Some(token) = cookie_header.split(';').find_map(|cookie| {
        let (name, value) = cookie.trim().split_once('=')?;
        (name == "kolibra_session").then_some(value)
    }) else {
        return false;
    };

    session_token_is_valid(token, secret, now)
}
