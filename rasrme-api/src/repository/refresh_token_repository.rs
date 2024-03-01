use sqlx::PgPool;

use crate::{
    exception::{ApiError, RTKN_ERR_FINDING_REFRESH_TOKEN, RTKN_ERR_INSERTING},
    model::{InsertRefreshToken, RefreshToken},
};

#[derive(Debug)]
pub struct RefreshTokenRepository;

impl RefreshTokenRepository {
    pub async fn save(
        pg_pool: &PgPool,
        refresh_token: InsertRefreshToken,
    ) -> Result<RefreshToken, ApiError> {
        let refresh_token = sqlx::query_as::<_, RefreshToken>("insert into tb_refresh_token(uuid, created_at, user_id) values ($1, $2, $3) returning *;")
            .bind(refresh_token.uuid.to_owned())
            .bind(refresh_token.created_at)
            .bind(refresh_token.user_id)
            .fetch_one(&*pg_pool)
            .await
            .map_err(|e| {
                tracing::error!("Error when inserting a refresh token: {}", e);
                ApiError::new(RTKN_ERR_INSERTING)
            })?;

        Ok(refresh_token)
    }

    pub async fn find_by_user_id_and_uuid(
        pg_pool: &PgPool,
        user_id: i64,
        uuid: &str,
    ) -> Result<RefreshToken, ApiError> {
        let refresh_token = sqlx::query_as::<_, RefreshToken>(
            "select * from tb_refresh_token trt where trt.user_id = $1 and trt.uuid = $2",
        )
        .bind(user_id)
        .bind(uuid)
        .fetch_one(&*pg_pool)
        .await
        .map_err(|e| {
            tracing::error!(
                "Error when finding a refresh token by user_id and uuid: {}",
                e
            );
            ApiError::new(RTKN_ERR_FINDING_REFRESH_TOKEN)
        })?;

        Ok(refresh_token)
    }
}
