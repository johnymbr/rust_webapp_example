use chrono::Utc;
use sqlx::PgPool;

use crate::{
    exception::{
        ApiError, USR_ERR_DELETE, USR_ERR_FIND_ALL_PAGINATED, USR_ERR_FIND_BY_EMAIL,
        USR_ERR_FIND_BY_ID, USR_ERR_INSERTING, USR_ERR_UPDATING, USR_ERR_VALIDATE_EMAIL,
    },
    model::{Pagination, PaginationResponse, User, UserRequest},
};

#[derive(Debug)]
pub struct UserRepository {}

impl UserRepository {
    pub async fn find_all(
        pg_pool: &PgPool,
        pagination: Pagination,
    ) -> Result<PaginationResponse<User>, ApiError> {
        let total = sqlx::query_scalar("select count(*) as count from tb_user")
            .fetch_one(pg_pool)
            .await
            .map_err(|e| {
                tracing::error!("Error when finding users: {}", e);
                ApiError::new(USR_ERR_FIND_ALL_PAGINATED)
            })?;

        let mut response = PaginationResponse {
            page: pagination.page.unwrap(),
            page_size: pagination.page_size.unwrap(),
            total,
            elements: Vec::<User>::new(),
        };

        if total > 0 {
            let users =
                sqlx::query_as::<_, User>("select * from tb_user limit ? offset ?")
                    .bind(pagination.page_size.unwrap())
                    .bind(pagination.offset())
                    .fetch_all(pg_pool)
                    .await
                    .map_err(|e| {
                        tracing::error!("Error when finding users: {}", e);
                        ApiError::new(USR_ERR_FIND_ALL_PAGINATED)
                    })?;

            response.elements = users;
        }

        Ok(response)
    }

    pub async fn find_by_id(pg_pool: &PgPool, id: i64) -> Result<Option<User>, ApiError> {
        let user = sqlx::query_as("select * from tb_user where id = $1")
            .bind(id)
            .fetch_optional(pg_pool)
            .await
            .map_err(|e| {
                tracing::error!("Error when finding an user by id: {}", e);
                ApiError::new(USR_ERR_FIND_BY_ID)
            })?;

        Ok(user)
    }

    pub async fn find_by_email(pg_pool: &PgPool, email: &str) -> Result<Option<User>, ApiError> {
        let user = sqlx::query_as("select * from tb_user where email = $1")
            .bind(email)
            .fetch_optional(pg_pool)
            .await
            .map_err(|e| {
                tracing::error!("Error when finding an user by email: {}", e);
                ApiError::new(USR_ERR_FIND_BY_EMAIL)
            })?;

        Ok(user)
    }

    pub async fn save(
        transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        entity: UserRequest,
    ) -> Result<User, ApiError> {
        let user = sqlx::query_as::<_, User>("insert into tb_user(first_name, last_name, complete_name, email, password, active, created_at, updated_at) values ($1, $2, $3, $4, $5, $6, $7, $8) returning *;")
            .bind(entity.first_name.unwrap())
            .bind(entity.last_name)
            .bind(entity.complete_name)
            .bind(entity.email.unwrap())
            .bind(entity.password.unwrap())
            .bind(entity.active.unwrap())
            .bind(Utc::now())
            .bind(Utc::now())
            .fetch_one(&mut **transaction)
            .await
            .map_err(|e| {
                tracing::info!("Error when inserting an user: {}", e);
                ApiError::new(USR_ERR_INSERTING)
            })?;

        Ok(user)
    }

    pub async fn update(
        transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        entity: User,
    ) -> Result<User, ApiError> {
        let user = sqlx::query_as::<_,User>("update tb_user set first_name = $1, last_name = $2, complete_name = $3, updated_at = $4 where id = $5 returning *;")
            .bind(entity.first_name)
            .bind(entity.last_name)
            .bind(entity.complete_name)
            .bind(Utc::now())
            .bind(entity.id)
            .fetch_one(&mut **transaction)
            .await
            .map_err(|e| {
                tracing::info!("Error when updating an user: {}", e);
                ApiError::new(USR_ERR_UPDATING)
            })?;

        Ok(user)
    }

    pub async fn delete(
        transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        id: i64,
    ) -> Result<(), ApiError> {
        sqlx::query("update tb_user set active = false, deleted = true, delete_at = $1 where id = $2")
            .bind(Utc::now())
            .bind(id)
            .execute(&mut **transaction)
            .await
            .map_err(|e| {
                tracing::info!("Error when deleting an user: {}", e);
                ApiError::new(USR_ERR_DELETE)
            })?;

        Ok(())
    }

    pub async fn validate_email(pg_pool: &PgPool, id: i64) -> Result<(), ApiError> {
        sqlx::query("update tb_user set active = true, updated_at = $1 where id = $2")
            .bind(Utc::now())
            .bind(id)
            .execute(&*pg_pool)
            .await
            .map_err(|e| {
                tracing::info!("Error when validate an user email: {}", e);
                ApiError::new(USR_ERR_VALIDATE_EMAIL)
            })?;

        Ok(())
    }
}
