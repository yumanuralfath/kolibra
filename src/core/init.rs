#[cfg(feature = "server")]
pub fn load_env() {
    let password_hash_was_set = std::env::var_os("APP_PASSWORD_HASH").is_some();
    let env_path = dotenvy::dotenv().ok();

    if !password_hash_was_set {
        if let Some(password_hash) = env_path
            .and_then(|path| std::fs::read_to_string(path).ok())
            .as_deref()
            .and_then(raw_password_hash)
        {
            std::env::set_var("APP_PASSWORD_HASH", password_hash);
        }
    }
}

#[cfg(feature = "server")]
fn raw_password_hash(env_contents: &str) -> Option<&str> {
    env_contents.lines().find_map(|line| {
        let value = line.strip_prefix("APP_PASSWORD_HASH=")?.trim();
        let value = value
            .strip_prefix('\'')
            .and_then(|value| value.strip_suffix('\''))
            .or_else(|| {
                value
                    .strip_prefix('"')
                    .and_then(|value| value.strip_suffix('"'))
            })
            .unwrap_or(value);
        (!value.is_empty()).then_some(value)
    })
}

#[cfg(all(test, feature = "server"))]
mod tests {
    use super::raw_password_hash;

    #[test]
    fn raw_hash_loader_preserves_argon2_dollar_signs() {
        let expected = "$argon2id$v=19$m=19456,t=2,p=1$c2FsdA$ZGlnZXN0";
        let contents = format!("SESSION_SECRET=example\nAPP_PASSWORD_HASH={expected}\n");

        assert_eq!(raw_password_hash(&contents), Some(expected));
    }

    #[test]
    fn raw_hash_loader_accepts_quoted_hashes() {
        let expected = "$argon2id$v=19$m=19456,t=2,p=1$c2FsdA$ZGlnZXN0";
        let contents = format!("APP_PASSWORD_HASH='{expected}'\n");

        assert_eq!(raw_password_hash(&contents), Some(expected));
    }
}
