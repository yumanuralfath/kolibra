#[cfg(feature = "server")]
use super::auth_logic::{create_session_token, session_cookie_is_valid, verify_password};
#[cfg(feature = "server")]
use dioxus::fullstack::HeaderMap;
#[cfg(feature = "server")]
use dioxus::logger::tracing;
use dioxus::{
    fullstack::{Json, SetCookie, SetHeader},
    prelude::*,
};
#[cfg(feature = "server")]
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(feature = "server")]
const SESSION_COOKIE: &str = "kolibra_session";
#[cfg(feature = "server")]
const SESSION_TTL_SECS: u64 = 8 * 60 * 60;

#[cfg(feature = "server")]
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(feature = "server")]
fn signed_session(expiry: u64) -> Option<String> {
    let secret = std::env::var("SESSION_SECRET").ok()?;
    create_session_token(expiry, &secret)
}

#[cfg(feature = "server")]
fn valid_session(headers: &HeaderMap) -> bool {
    let Some(cookie_header) = headers.get("cookie").and_then(|value| value.to_str().ok()) else {
        return false;
    };

    let Ok(secret) = std::env::var("SESSION_SECRET") else {
        return false;
    };
    session_cookie_is_valid(cookie_header, &secret, current_timestamp())
}

#[cfg(feature = "server")]
fn session_cookie(value: &str) -> SetHeader<SetCookie> {
    SetHeader::new(value).expect("session cookie must be a valid Set-Cookie header")
}

#[cfg(feature = "server")]
fn clear_session_cookie() -> SetHeader<SetCookie> {
    session_cookie(&format!(
        "{SESSION_COOKIE}=; HttpOnly; Path=/; SameSite=Strict; Max-Age=0; Secure"
    ))
}

#[post("/api/auth/login")]
pub async fn login_password(password: String) -> Result<(SetHeader<SetCookie>, Json<bool>)> {
    let verified = match std::env::var("APP_PASSWORD_HASH") {
        Ok(encoded_hash) => {
            match verify_password(&password, &encoded_hash) {
                Ok(()) => true,
                Err(super::auth_logic::PasswordCheckError::InvalidHash) => {
                    tracing::error!("APP_PASSWORD_HASH is not a valid Argon2 PHC hash");
                    false
                }
                Err(super::auth_logic::PasswordCheckError::Mismatch) => {
                    tracing::warn!("Password verification failed; entered password does not match configured hash");
                    false
                }
            }
        }
        Err(_) => {
            tracing::error!("APP_PASSWORD_HASH is not set in the server environment");
            false
        }
    };

    if !verified {
        return Ok((clear_session_cookie(), Json(false)));
    }

    let Some(token) = signed_session(current_timestamp() + SESSION_TTL_SECS) else {
        tracing::error!("SESSION_SECRET is not set or could not create session signature");
        return Ok((clear_session_cookie(), Json(false)));
    };

    let cookie = format!(
		"{SESSION_COOKIE}={token}; HttpOnly; Path=/; SameSite=Strict; Max-Age={SESSION_TTL_SECS}; Secure"
	);
    Ok((session_cookie(&cookie), Json(true)))
}

#[get("/api/auth/status", headers: HeaderMap)]
pub async fn check_session() -> Result<bool> {
    Ok(valid_session(&headers))
}

#[post("/api/auth/logout")]
pub async fn logout() -> Result<(SetHeader<SetCookie>, Json<bool>)> {
    Ok((clear_session_cookie(), Json(true)))
}
