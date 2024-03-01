use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    exception::{ApiError, RTKN_TOKEN_UUID_REQUIRED},
    model::{InsertRefreshToken, RefreshToken},
    repository::RefreshTokenRepository,
};

#[derive(Debug)]
pub struct RefreshTokenService;

impl RefreshTokenService {
    pub async fn create(pg_pool: &PgPool, user_id: i64) -> Result<RefreshToken, ApiError> {
        let uuid = Uuid::new_v4().to_string();

        let refresh_token = InsertRefreshToken {
            uuid,
            created_at: Utc::now(),
            user_id,
        };

        let refresh_token = RefreshTokenRepository::save(&pg_pool, refresh_token).await?;
        Ok(refresh_token)
    }

    pub async fn find_by_uuid(
        pg_pool: &PgPool,
        user_id: i64,
        token_uuid: Option<String>,
    ) -> Result<RefreshToken, ApiError> {
        if token_uuid.is_none() {
            return Err(ApiError::new(RTKN_TOKEN_UUID_REQUIRED));
        }

        let refresh_token = RefreshTokenRepository::find_by_user_id_and_uuid(
            &pg_pool,
            user_id,
            &token_uuid.unwrap(),
        )
        .await?;
        Ok(refresh_token)
    }
}
