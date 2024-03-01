use chrono::{Duration, Utc};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use sqlx::PgPool;

use crate::{
    exception::{ApiError, TKN_ERR_ALREADY_EXPIRED, TKN_ERR_ALREADY_VALIDATED, TKN_ERR_NOT_FOUND},
    model::{InsertToken, Token, TokenType, User},
    repository::TokenRepository,
};

pub struct TokenService;

impl TokenService {
    pub async fn create_user_email_validation_token(
        transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        user: &User,
    ) -> Result<Token, ApiError> {
        let token_rng = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect();

        let now = Utc::now();

        let token = InsertToken {
            user_id: user.id,
            token: token_rng,
            token_type: TokenType::UserEmailActivation,
            created_at: now,
            expire_at: now + Duration::days(1),
        };

        let token = TokenRepository::save(transaction, token).await?;
        Ok(token)
    }

    pub async fn find_by_user_id_and_token_and_token_type(
        pg_pool: &PgPool,
        user_id: i64,
        token: &str,
        token_type: &TokenType,
    ) -> Result<Token, ApiError> {
        let token = TokenRepository::find_by_user_id_and_token_and_token_type(
            pg_pool, user_id, token, token_type,
        )
        .await?;

        if token.is_none() {
            return Err(ApiError::new(TKN_ERR_NOT_FOUND));
        }

        Ok(token.unwrap())
    }

    pub async fn check_email_validation_token(
        pg_pool: &PgPool,
        user_id: i64,
        token: &str,
        token_type: &TokenType,
    ) -> Result<Token, ApiError> {
        let mut token = TokenService::find_by_user_id_and_token_and_token_type(
            pg_pool, user_id, token, token_type,
        )
        .await?;

        if token.validated {
            return Err(ApiError::new(TKN_ERR_ALREADY_VALIDATED));
        }

        let now = Utc::now();
        if token.expire_at.is_some() && now.gt(&token.expire_at.unwrap()) {
            return Err(ApiError::new(TKN_ERR_ALREADY_EXPIRED));
        }

        token.validated = true;
        token.validated_at = Some(now);

        TokenRepository::validate(pg_pool, &token).await?;

        Ok(token)
    }
}
