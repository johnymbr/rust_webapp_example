use chrono::{DateTime, Duration, Utc};
use hyper::StatusCode;
use jsonwebtoken::{decode, encode, Algorithm, Header, Validation};

use crate::{
    config::JWT_CONFIG,
    exception::{
        ApiError, JWT_ERR_EXPIRED_ACCESS_TOKEN, JWT_ERR_EXPIRED_REFRESH_TOKEN,
        JWT_ERR_GENERATING_ACCESS_TOKEN, JWT_ERR_GENERATING_REFRESH_TOKEN,
        JWT_ERR_INVALID_EXPIRATION_TIMESTAMP, JWT_ERR_VALIDATION_ACCESS_TOKEN,
        JWT_ERR_VALIDATION_REFRESH_TOKEN,
    },
    model::JwtTokenClaims,
    service::RefreshTokenService,
};

pub struct JwtService;

impl JwtService {
    pub fn generate_access_token(user_id: i64) -> Result<String, ApiError> {
        let now = Utc::now();

        let claims = JwtTokenClaims {
            sub: user_id,
            token_uuid: None,
            exp: (now + Duration::minutes(JWT_CONFIG.get().unwrap().access_token_ttl)).timestamp(),
            iat: now.timestamp(),
            nbf: now.timestamp(),
        };

        let header = Header::default();
        let token = encode(
            &header,
            &claims,
            &JWT_CONFIG.get().unwrap().access_token_encoding_key,
        )
        .map_err(|err| {
            tracing::error!("Error when generate access token: {}", err);
            ApiError::new(JWT_ERR_GENERATING_ACCESS_TOKEN)
        })?;

        Ok(token)
    }

    pub fn generate_refresh_token(user_id: i64, uuid: String) -> Result<String, ApiError> {
        let now = Utc::now();

        let claims = JwtTokenClaims {
            sub: user_id,
            token_uuid: Some(uuid),
            exp: (now + Duration::days(JWT_CONFIG.get().unwrap().refresh_token_ttl)).timestamp(),
            iat: now.timestamp(),
            nbf: now.timestamp(),
        };

        let header = Header::default();
        let token = encode(
            &header,
            &claims,
            &JWT_CONFIG.get().unwrap().refresh_token_encoding_key,
        )
        .map_err(|err| {
            tracing::error!("Error when generate refresh token: {}", err);
            ApiError::new(JWT_ERR_GENERATING_REFRESH_TOKEN)
        })?;

        Ok(token)
    }

    pub fn verify_refresh_token(refresh_token: &str) -> Result<JwtTokenClaims, ApiError> {
        let validation = Validation::default();
        let decoded = decode::<JwtTokenClaims>(
            refresh_token,
            &JWT_CONFIG.get().unwrap().refresh_token_decoding_key,
            &validation,
        )
        .map_err(|err| {
            tracing::error!("Error when validating a refresh token: {}", err);
            ApiError::new(JWT_ERR_VALIDATION_REFRESH_TOKEN)
        })?;

        let claims = decoded.claims;
        let now = Utc::now();
        match DateTime::from_timestamp(claims.exp, 0) {
            Some(expiration) => {
                if expiration.lt(&now) {
                    return Err(ApiError::new(JWT_ERR_EXPIRED_REFRESH_TOKEN));
                }
            }
            None => {
                return Err(ApiError::new(JWT_ERR_INVALID_EXPIRATION_TIMESTAMP));
            }
        }

        Ok(claims)
    }

    pub fn verify_access_token(access_token: &str) -> Result<JwtTokenClaims, ApiError> {
        let validation = Validation::default();
        let decoded = decode::<JwtTokenClaims>(
            access_token,
            &JWT_CONFIG.get().unwrap().access_token_decoding_key,
            &validation,
        )
        .map_err(|err| {
            tracing::error!("Error when validating an access token: {}", err);
            ApiError::new_with_status(StatusCode::UNAUTHORIZED, JWT_ERR_VALIDATION_ACCESS_TOKEN)
        })?;

        let claims = decoded.claims;
        let now = Utc::now();
        match DateTime::from_timestamp(claims.exp, 0) {
            Some(expiration) => {
                if expiration.lt(&now) {
                    return Err(ApiError::new_with_status(
                        StatusCode::UNAUTHORIZED,
                        JWT_ERR_EXPIRED_ACCESS_TOKEN,
                    ));
                }
            }
            None => {
                return Err(ApiError::new_with_status(
                    StatusCode::UNAUTHORIZED,
                    JWT_ERR_INVALID_EXPIRATION_TIMESTAMP,
                ));
            }
        }

        Ok(claims)
    }
}
