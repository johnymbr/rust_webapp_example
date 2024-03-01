use std::sync::OnceLock;

use jsonwebtoken::{DecodingKey, EncodingKey};

pub struct TokenConfig {
    pub token_expire_days: i64,
}

static TOKEN_CONFIG: OnceLock<TokenConfig> = OnceLock::new();

pub fn get_token_config() -> &'static TokenConfig {
    TOKEN_CONFIG.get_or_init(|| {
        let expire_days = std::env::var("TOKEN_EXPIRE_DAYS")
            .unwrap_or(String::from("1"))
            .parse()
            .unwrap();

        TokenConfig {
            token_expire_days: expire_days,
        }
    })
}

pub struct JwtTokenConfig {
    pub access_token_encoding_key: EncodingKey,
    pub access_token_decoding_key: DecodingKey,
    pub refresh_token_encoding_key: EncodingKey,
    pub refresh_token_decoding_key: DecodingKey,
    pub access_token_ttl: i64,
    pub refresh_token_ttl: i64,
}

pub static JWT_CONFIG: OnceLock<JwtTokenConfig> = OnceLock::new();

impl JwtTokenConfig {
    pub fn config() {
        let access_token_secret_key = std::env::var("ACCESS_TOKEN_SECRET_KEY")
            .expect("Access token secret key is required to start Rasrme API.");

        let refresh_token_secret_key = std::env::var("REFRESH_TOKEN_SECRET_KEY")
            .expect("Refresh token secret key is required to start Rasrme API.");

        let access_token_ttl = std::env::var("ACCESS_TOKEN_TTL")
            .expect("Access token ttl is required to start Rasrme API.")
            .parse::<i64>()
            .expect("Access token ttl needs to be a number.");

        let refresh_token_ttl = std::env::var("REFRESH_TOKEN_TTL")
            .expect("Refresh token ttl is required to start Rasrme API.")
            .parse::<i64>()
            .expect("Refresh token ttl needs to be a number.");

        JWT_CONFIG.get_or_init(|| JwtTokenConfig {
            access_token_encoding_key: EncodingKey::from_secret(access_token_secret_key.as_bytes()),
            access_token_decoding_key: DecodingKey::from_secret(access_token_secret_key.as_bytes()),
            refresh_token_encoding_key: EncodingKey::from_secret(
                refresh_token_secret_key.as_bytes(),
            ),
            refresh_token_decoding_key: DecodingKey::from_secret(
                refresh_token_secret_key.as_bytes(),
            ),
            access_token_ttl,
            refresh_token_ttl,
        });
    }
}
