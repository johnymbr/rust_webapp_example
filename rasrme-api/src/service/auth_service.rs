use std::sync::Arc;

use axum_extra::extract::CookieJar;

use crate::{
    exception::{
        ApiError, AUTH_EMAIL_AND_PASSWORD_REQUIRED, AUTH_EMAIL_REQUIRED, AUTH_GRANT_TYPE_INVALID,
        AUTH_GRANT_TYPE_REQUIRED, AUTH_INACTIVE_USER, AUTH_INVALID_LOGIN,
        AUTH_REFRESH_TOKEN_REQUIRED, AUTH_REFRESH_TOKEN_REVOKED, AUTH_TOKEN_REQUIRED,
    },
    model::{AccessTokenRequest, EmailValidationRequest, LoginRequest, LoginResponse, TokenType},
    service::{JwtService, RefreshTokenService},
    ArcturusState,
};

use super::{TokenService, UserService};

pub struct AuthService;

impl AuthService {
    pub async fn email_validation(
        state: Arc<ArcturusState>,
        request: EmailValidationRequest,
    ) -> Result<(), ApiError> {
        // check if email and token was sent.
        if request.email.is_none() {
            return Err(ApiError::new(AUTH_EMAIL_REQUIRED));
        }

        if request.token.is_none() {
            return Err(ApiError::new(AUTH_TOKEN_REQUIRED));
        }

        // check if user email exists...
        let user =
            UserService::find_by_email(request.email.unwrap().as_str(), &state.pg_pool).await?;

        // check if token belongs to user and if token is valid.
        let _tkn = TokenService::check_email_validation_token(
            &state.pg_pool,
            user.id,
            request.token.unwrap().as_str(),
            &TokenType::UserEmailActivation,
        )
        .await?;

        // validate token and validate user account.
        UserService::validate_email(&state.pg_pool, user.id).await?;

        Ok(())
    }

    pub async fn login(
        state: Arc<ArcturusState>,
        entity: LoginRequest,
    ) -> Result<LoginResponse, ApiError> {
        if entity.email.is_none() || entity.password.is_none() {
            return Err(ApiError::new(AUTH_EMAIL_AND_PASSWORD_REQUIRED));
        }

        let user = UserService::find_by_email(entity.email.unwrap().as_str(), &state.pg_pool)
            .await
            .or_else(|_| Err(ApiError::new(AUTH_INVALID_LOGIN)))?;

        if !user.active {
            return Err(ApiError::new(AUTH_INACTIVE_USER));
        }

        if let Err(_) = bcrypt::verify(entity.password.unwrap(), &user.password) {
            return Err(ApiError::new(AUTH_INVALID_LOGIN));
        }

        let access_token = JwtService::generate_access_token(user.id)?;

        // create a refresh token register and then generate a jwt refresh token
        let refresh_token = RefreshTokenService::create(&state.pg_pool, user.id).await?;
        let refresh_token = JwtService::generate_refresh_token(user.id, refresh_token.uuid)?;

        Ok(LoginResponse {
            user: user,
            access_token: access_token,
            refresh_token: refresh_token,
        })
    }

    pub async fn refresh_access_token(
        state: Arc<ArcturusState>,
        cookie_jar: CookieJar,
        request: AccessTokenRequest,
    ) -> Result<String, ApiError> {
        let refresh_token = match cookie_jar
            .get("JWT_REFRESH_TOKEN")
            .map(|cookie| cookie.value().to_string())
        {
            Some(tkn) => Ok(tkn),
            None => request
                .refreh_token
                .ok_or_else(|| ApiError::new(AUTH_REFRESH_TOKEN_REQUIRED)),
        }?;

        if request.grant_type.is_none() {
            return Err(ApiError::new(AUTH_GRANT_TYPE_REQUIRED));
        }

        if request.grant_type.as_ref().unwrap() != "refresh_token" {
            return Err(ApiError::new(AUTH_GRANT_TYPE_INVALID));
        }

        let jwt_claims = JwtService::verify_refresh_token(&refresh_token)?;

        // check if
        let refresh_token_db = RefreshTokenService::find_by_uuid(
            &state.pg_pool,
            jwt_claims.sub,
            jwt_claims.token_uuid,
        )
        .await?;

        if refresh_token_db.revoked.is_some_and(|revoked| revoked) {
            return Err(ApiError::new(AUTH_REFRESH_TOKEN_REVOKED));
        }

        let access_token = JwtService::generate_access_token(jwt_claims.sub)?;
        Ok(access_token)
    }
}
