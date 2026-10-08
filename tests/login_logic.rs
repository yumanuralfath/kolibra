#[path = "../src/server/auth_logic.rs"]
mod auth_logic;
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use auth_logic::{
    create_session_token, session_cookie_is_valid, verify_password, PasswordCheckError,
};
use rand_core::OsRng;
fn password_hash_for(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("test password should hash")
        .to_string()
}
#[test]
fn accepts_the_password_matching_the_argon2_hash() {
    let hash = password_hash_for("correct horse battery staple");
    assert!(verify_password("correct horse battery staple", &hash).is_ok());
}
#[test]
fn rejects_wrong_password_and_malformed_hash() {
    let hash = password_hash_for("correct horse battery staple");
    assert_eq!(
        verify_password("incorrect password", &hash),
        Err(PasswordCheckError::Mismatch),
    );
    assert_eq!(
        verify_password("correct horse battery staple", "not-an-argon2-hash"),
        Err(PasswordCheckError::InvalidHash),
    );
}
#[test]
fn identifies_malformed_hash_separately_from_a_password_mismatch() {
    assert_eq!(
        verify_password("any password", "not-an-argon2-hash"),
        Err(PasswordCheckError::InvalidHash),
    );
    let hash = password_hash_for("correct password");
    assert_eq!(
        verify_password("wrong password", &hash),
        Err(PasswordCheckError::Mismatch),
    );
}
#[test]
fn accepts_a_valid_session_cookie_even_with_other_cookies() {
    let token = create_session_token(2_000, "test-session-secret").unwrap();
    let cookies = format!("theme=dark; kolibra_session={token}; other=value");
    assert!(session_cookie_is_valid(
        &cookies,
        "test-session-secret",
        1_000
    ));
}
#[test]
fn rejects_session_with_wrong_secret_tampering_or_expiration() {
    let token = create_session_token(2_000, "test-session-secret").unwrap();
    let cookies = format!("kolibra_session={token}");
    let tampered_cookies = format!("kolibra_session=2000.{}", "00".repeat(32));
    assert!(!session_cookie_is_valid(&cookies, "wrong-secret", 1_000));
    assert!(!session_cookie_is_valid(
        &tampered_cookies,
        "test-session-secret",
        1_000
    ));
    assert!(!session_cookie_is_valid(
        &cookies,
        "test-session-secret",
        2_000
    ));
}
#[test]
fn rejects_missing_or_malformed_session_cookie() {
    assert!(!session_cookie_is_valid(
        "theme=dark",
        "test-session-secret",
        1_000
    ));
    assert!(!session_cookie_is_valid(
        "kolibra_session=not-a-token",
        "test-session-secret",
        1_000,
    ),);
}
