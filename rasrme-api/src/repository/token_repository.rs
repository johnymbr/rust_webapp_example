use sqlx::PgPool;

use crate::{
    exception::{
        ApiError, TKN_ERR_FIND_BY_ID, TKN_ERR_FIND_BY_TOKEN, TKN_ERR_INSERTING, TKN_ERR_VALIDATING,
    },
    model::{InsertToken, Token, TokenType},
};

#[derive(Debug)]
pub struct TokenRepository;

impl TokenRepository {
    pub async fn find_by_id(pg_pool: &PgPool, id: i64) -> Result<Option<Token>, ApiError> {
        let token = sqlx::query_as("select * from tb_token where id = ?")
            .bind(id)
            .fetch_optional(&*pg_pool)
            .await
            .map_err(|e| {
                tracing::error!("Error when finding a token by id: {}", e);
                ApiError::new(TKN_ERR_FIND_BY_ID)
            })?;

        Ok(token)
    }

    pub async fn find_by_token_and_user_id(
        pg_pool: &PgPool,
        token: &String,
        user_id: i64,
    ) -> Result<Option<Token>, ApiError> {
        let token =
            sqlx::query_as("select * from tb_token where token = ? and user_id = ?")
                .bind(token)
                .bind(user_id)
                .fetch_optional(&*pg_pool)
                .await
                .map_err(|e| {
                    tracing::error!(
                        "Error when finding a token by user_id and token_type: {}",
                        e
                    );
                    ApiError::new(TKN_ERR_FIND_BY_TOKEN)
                })?;

        Ok(token)
    }

    pub async fn find_by_user_id_and_token_and_token_type(
        pg_pool: &PgPool,
        user_id: i64,
        token: &str,
        token_type: &TokenType,
    ) -> Result<Option<Token>, ApiError> {
        let token = sqlx::query_as(
            "select * from tb_token where user_id = $1 and token = $2 and token_type = $3",
        )
        .bind(user_id)
        .bind(token)
        .bind(token_type)
        .fetch_optional(&*pg_pool)
        .await
        .map_err(|e| {
            tracing::error!(
                "Error when finding a token by user_id and token and token_type: {}",
                e
            );
            ApiError::new(TKN_ERR_FIND_BY_TOKEN)
        })?;

        Ok(token)
    }

    pub async fn save(
        transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        entity: InsertToken,
    ) -> Result<Token, ApiError> {
        let token = sqlx::query_as::<_, Token>("insert into tb_token(token, token_type, validated, created_at, expire_at, user_id) values ($1, $2, $3, $4, $5, $6) returning *;")
            .bind(entity.token.to_owned())
            .bind(entity.token_type)
            .bind(false)
            .bind(entity.created_at)
            .bind(entity.expire_at)
            .bind(entity.user_id)
            .fetch_one(&mut **transaction)
            .await
            .map_err(|e| {
                tracing::info!("Error when inserting a token: {}", e);
                ApiError::new(TKN_ERR_INSERTING)
            })?;

        Ok(token)
    }

    pub async fn validate(pg_pool: &PgPool, token: &Token) -> Result<(), ApiError> {
        sqlx::query("update tb_token set validated = true, validated_at = $1 where id = $2")
            .bind(token.validated_at.unwrap())
            .bind(token.id)
            .execute(&*pg_pool)
            .await
            .map_err(|e| {
                tracing::info!("Error when validating a token: {}", e);
                ApiError::new(TKN_ERR_VALIDATING)
            })?;

        Ok(())
    }
}
