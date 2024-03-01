use std::sync::Arc;

use bcrypt::{hash, DEFAULT_COST};
use hyper::StatusCode;
use sqlx::PgPool;

use crate::{
    exception::{
        ApiError, ERR_DB_TRANSACTION_ERROR, ERR_PASSWORD_HASH, USR_ERR_INSERTING, USR_ERR_NOT_FOUND,
    },
    model::{MailEvent, Pagination, PaginationResponse, User, UserRequest},
    notifier::NotifierAmqp,
    repository::UserRepository,
};

use super::TokenService;

#[derive(Debug)]
pub struct UserService {
    pg_pool: Arc<PgPool>,
    amqp: Arc<NotifierAmqp>,
}

impl UserService {
    pub async fn new(pg_pool: Arc<PgPool>, amqp: Arc<NotifierAmqp>) -> Self {
        UserService {
            pg_pool: pg_pool,
            amqp: amqp.clone(),
        }
    }

    pub async fn find_all(
        &self,
        pagination: Pagination,
    ) -> Result<PaginationResponse<User>, ApiError> {
        pagination.validate()?;

        let response = UserRepository::find_all(&self.pg_pool, pagination).await?;
        Ok(response)
    }

    pub async fn find_by_id(&self, id: i64) -> Result<User, ApiError> {
        let response = UserRepository::find_by_id(&self.pg_pool, id).await?;

        if response.is_none() {
            return Err(ApiError::new_with_status(
                StatusCode::NOT_FOUND,
                USR_ERR_NOT_FOUND,
            ));
        }

        Ok(response.unwrap())
    }

    pub async fn find_by_email(email: &str, pg_pool: &PgPool) -> Result<User, ApiError> {
        let response = UserRepository::find_by_email(pg_pool, email).await?;

        if response.is_none() {
            return Err(ApiError::new_with_status(
                StatusCode::NOT_FOUND,
                USR_ERR_NOT_FOUND,
            ));
        }

        Ok(response.unwrap())
    }

    pub async fn save(&self, mut entity: UserRequest) -> Result<User, ApiError> {
        entity.validate_on_save()?;

        let mut complete_name = entity.first_name.as_ref().unwrap().to_uppercase();
        if let Some(last_name) = entity.last_name.as_ref() {
            complete_name.push_str(" ");
            complete_name.push_str(&last_name.to_uppercase())
        }
        entity.password = Some(
            hash(entity.password.unwrap(), DEFAULT_COST)
                .map_err(|_e| ApiError::new(ERR_PASSWORD_HASH))?,
        );
        entity.complete_name = Some(complete_name);
        entity.active = Some(false);

        match self.pg_pool.begin().await {
            Ok(mut tx) => {
                let response = UserRepository::save(&mut tx, entity).await?;

                let tkn =
                    TokenService::create_user_email_validation_token(&mut tx, &response).await?;

                let event = MailEvent {
                    subject: String::from("Validate account."),
                    to: &response.email,
                    name: &response.first_name,
                    token: tkn.token.as_str(),
                };

                if let Err(error) = tx.commit().await {
                    tracing::error!(
                        "Error when commiting transaction with new user and token: {}",
                        error
                    );
                    return Err(ApiError::new(USR_ERR_INSERTING));
                }

                self.amqp.send_mail_event(event).await;

                Ok(response)
            }
            Err(error) => {
                tracing::error!("Error when generating a new transaction: {}", error);
                Err(ApiError::new(ERR_DB_TRANSACTION_ERROR))
            }
        }
    }

    pub async fn update(&self, id: i64, entity: UserRequest) -> Result<User, ApiError> {
        todo!()
    }

    pub async fn delete(&self, id: i64) -> Result<(), ApiError> {
        todo!()
    }

    pub async fn validate_email(pg_pool: &PgPool, id: i64) -> Result<(), ApiError> {
        UserRepository::validate_email(pg_pool, id).await?;

        Ok(())
    }
}
