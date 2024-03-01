use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::CookieJar;
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::{
    exception::{ApiError, JWT_ACCESS_TOKEN_REQUIRED},
    service::JwtService,
};

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR")]
#[sqlx(rename_all = "PascalCase")]
pub enum TokenType {
    Empty,
    UserEmailActivation,
}

#[derive(Debug)]
pub struct InsertToken {
    pub token: String,
    pub token_type: TokenType,
    pub user_id: i64,
    pub created_at: DateTime<Utc>,
    pub expire_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub id: i64,
    pub user_id: i64,
    pub token: String,
    pub token_type: TokenType,
    pub validated: bool,
    pub created_at: DateTime<Utc>,
    pub validated_at: Option<DateTime<Utc>>,
    pub expire_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JwtTokenClaims {
    pub sub: i64,
    pub token_uuid: Option<String>,
    pub exp: i64,
    pub iat: i64,
    pub nbf: i64,
}

#[async_trait]
impl<S> FromRequestParts<S> for JwtTokenClaims
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookie_jar = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|err| {
                tracing::error!(
                    "Error when generate JwtTokenClaims of from_request_parts: {}",
                    err
                );
                ApiError::new_with_status(StatusCode::UNAUTHORIZED, JWT_ACCESS_TOKEN_REQUIRED)
            })?;

        let access_token = match cookie_jar
            .get("JWT")
            .map(|cookie| cookie.value().to_string())
        {
            Some(tkn) => Ok(tkn.to_owned()),
            None => Err(ApiError::new_with_status(
                StatusCode::UNAUTHORIZED,
                JWT_ACCESS_TOKEN_REQUIRED,
            )),
        }?;

        let jwt_claims = JwtService::verify_access_token(&access_token)?;
        Ok(jwt_claims)
    }
}
