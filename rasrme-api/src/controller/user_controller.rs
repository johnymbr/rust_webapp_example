use std::sync::Arc;

use axum::{
    extract::{self, Path, Query, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use hyper::StatusCode;
use sqlx::PgPool;
use tracing::instrument;

use crate::{
    exception::ApiError,
    model::{JwtTokenClaims, Pagination, UserRequest},
    notifier::NotifierAmqp,
    service::UserService,
};

pub struct UserController;

impl Default for UserController {
    fn default() -> Self {
        Self::new()
    }
}

impl UserController {
    pub fn new() -> Self {
        UserController {}
    }

    pub async fn routes(&self, pg_pool: Arc<PgPool>, amqp: Arc<NotifierAmqp>) -> Router {
        tracing::info!("Config routes to /user");
        let user_service = UserService::new(Arc::clone(&pg_pool), Arc::clone(&amqp)).await;

        Router::new()
            .route(
                "/user",
                get(UserController::find_all).post(UserController::save),
            )
            .route(
                "/user/:id",
                get(UserController::find_by_id)
                    .put(UserController::update)
                    .delete(UserController::delete),
            )
            .route("/user/info", get(UserController::info))
            .with_state(Arc::new(user_service))
    }

    #[instrument]
    async fn find_all(
        Query(pagination): Query<Pagination>,
        State(user_service): State<Arc<UserService>>,
    ) -> Result<impl IntoResponse, ApiError> {
        let response = user_service.find_all(pagination).await?;
        Ok((StatusCode::OK, Json(response)))
    }

    #[instrument]
    async fn info(
        claims: JwtTokenClaims,
        State(user_service): State<Arc<UserService>>,
    ) -> Result<impl IntoResponse, ApiError> {
        let response = user_service.find_by_id(claims.sub).await?;
        Ok((StatusCode::OK, Json(response)))
    }

    #[instrument]
    async fn find_by_id(
        Path(id): Path<i64>,
        State(user_service): State<Arc<UserService>>,
    ) -> Result<impl IntoResponse, ApiError> {
        let response = user_service.find_by_id(id).await?;
        Ok((StatusCode::OK, Json(response)))
    }

    #[instrument]
    async fn save(
        State(user_service): State<Arc<UserService>>,
        extract::Json(entity): extract::Json<UserRequest>,
    ) -> Result<impl IntoResponse, ApiError> {
        let response = user_service.save(entity).await?;
        Ok((StatusCode::OK, Json(response)))
    }

    #[instrument]
    async fn update(
        Path(id): Path<i64>,
        State(user_service): State<Arc<UserService>>,
        extract::Json(entity): extract::Json<UserRequest>,
    ) -> Result<impl IntoResponse, ApiError> {
        let response = user_service.update(id, entity).await?;
        Ok((StatusCode::OK, Json(response)))
    }

    #[instrument]
    async fn delete(
        Path(id): Path<i64>,
        State(user_service): State<Arc<UserService>>,
    ) -> Result<impl IntoResponse, ApiError> {
        user_service.delete(id).await?;
        Ok(StatusCode::NO_CONTENT)
    }
}
