use std::sync::Arc;

use axum::{
    body::Body,
    extract::{self, State},
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use hyper::{header, HeaderMap, StatusCode};
use serde_json::json;
use time::Duration;

use crate::{
    exception::ApiError,
    model::{AccessTokenRequest, EmailValidationRequest, ForgotPasswordRequest, LoginRequest},
    service::AuthService,
    ArcturusState,
};

pub struct AuthController;

impl AuthController {
    pub async fn routes(state: Arc<ArcturusState>) -> Router {
        tracing::info!("Config routes to /auth");

        Router::new()
            .route("/auth/login", post(AuthController::login))
            .route("/auth/access-token", post(AuthController::access_token))
            .route(
                "/auth/forget-password",
                post(AuthController::forgot_password),
            )
            .route("/auth/reset-password", post(AuthController::reset_password))
            .route(
                "/auth/validate-email",
                post(AuthController::email_validation),
            )
            .with_state(state)
    }

    // function that logins a user and return jwt tokens for access_token and refresh_token.
    pub async fn login(
        State(state): State<Arc<ArcturusState>>,
        extract::Json(entity): extract::Json<LoginRequest>,
    ) -> Result<impl IntoResponse, ApiError> {
        // https://github.com/wpcodevo/rust-axum-jwt-rs256/blob/master/src/handler.rs -> login_user_handler
        // https://github.com/tokio-rs/axum/blob/main/examples/jwt/src/main.rs
        let response = AuthService::login(state, entity).await?;

        // create cookies...
        let access_cookie = Cookie::build(("JWT", response.access_token))
            .path("/")
            .max_age(Duration::minutes(-1))
            .same_site(SameSite::Lax)
            .http_only(true);

        let refresh_cookie = Cookie::build(("JWT_REFRESH_TOKEN", response.refresh_token))
            .path("/auth/access-token")
            .max_age(Duration::minutes(-1))
            .same_site(SameSite::Lax)
            .http_only(true);

        // set cookies...
        let mut headers = HeaderMap::new();
        headers.append(
            header::SET_COOKIE,
            access_cookie.to_string().parse().unwrap(),
        );
        headers.append(
            header::SET_COOKIE,
            refresh_cookie.to_string().parse().unwrap(),
        );

        // add to response
        let mut response = Response::new(json!(response.user).to_string());
        response.headers_mut().extend(headers);

        Ok(response)
    }

    // function that generates a new access_token based on refresh_token inside cookies.
    pub async fn access_token(
        cookie_jar: CookieJar,
        State(state): State<Arc<ArcturusState>>,
        extract::Json(request): extract::Json<AccessTokenRequest>,
    ) -> Result<impl IntoResponse, ApiError> {
        let access_token = AuthService::refresh_access_token(state, cookie_jar, request).await?;

        // create cookies...
        let access_cookie = Cookie::build(("JWT", access_token))
            .path("/")
            .max_age(Duration::minutes(-1))
            .same_site(SameSite::Lax)
            .http_only(true);

        // set cookies...
        let mut headers = HeaderMap::new();
        headers.append(
            header::SET_COOKIE,
            access_cookie.to_string().parse().unwrap(),
        );

        // add to response
        let mut response = Response::builder()
            .status(StatusCode::NO_CONTENT)
            .body(Body::empty())
            .unwrap();
        response.headers_mut().extend(headers);

        Ok(response)
    }

    // function that sends a mail to the user to reset his password.
    pub async fn forgot_password(
        State(state): State<Arc<ArcturusState>>,
        extract::Json(entity): extract::Json<ForgotPasswordRequest>,
    ) -> Result<impl IntoResponse, ApiError> {
        let body = json!({
            "status": 400,
            "message": "Not yet implemented",
        });
        Ok((StatusCode::NOT_FOUND, Json(body)))
    }

    // function that receives an email, token and a new password to reset user's password.
    pub async fn reset_password(
        State(state): State<Arc<ArcturusState>>,
        extract::Json(entity): extract::Json<ForgotPasswordRequest>,
    ) -> Result<impl IntoResponse, ApiError> {
        let body = json!({
            "status": 400,
            "message": "Not yet implemented",
        });
        Ok((StatusCode::NOT_FOUND, Json(body)))
    }

    // function that confirms user's mail.
    pub async fn email_validation(
        State(state): State<Arc<ArcturusState>>,
        extract::Json(entity): extract::Json<EmailValidationRequest>,
    ) -> Result<impl IntoResponse, ApiError> {
        AuthService::email_validation(state.clone(), entity).await?;

        Ok(StatusCode::NO_CONTENT)
    }
}
