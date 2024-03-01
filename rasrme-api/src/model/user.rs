use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::{
    exception::{
        ApiError, ApiFieldError, ERR_CONFIRM_PASSWORD_DIFF_PASSWORD, ERR_INVALID_EMAIL,
        ERR_INVALID_PASSWORD, ERR_INVALID_REQUEST, ERR_MIN_SIZE, ERR_REQUIRED_FIELD,
    },
    util,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub complete_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub confirm_password: Option<String>,
    pub fcm_token: Option<String>,
    pub active: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: Option<String>,
    pub complete_name: Option<String>,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub active: bool,
    pub deleted: Option<bool>,
    #[serde(skip_serializing)]
    pub fcm_token: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl UserRequest {
    pub fn validate_on_save(&mut self) -> Result<(), ApiError> {
        let mut field_errors = Vec::<ApiFieldError>::new();

        if let Err(error) = self.validate_first_name(true) {
            field_errors.push(error);
        }

        if let Err(error) = self.validate_last_name() {
            field_errors.push(error);
        }

        if let Err(error) = self.validate_email() {
            field_errors.push(error);
        }

        if let Err(error) = self.validate_password() {
            field_errors.push(error);
        }

        if !field_errors.is_empty() {
            return Err(ApiError::new_with_field_errors(
                ERR_INVALID_REQUEST,
                field_errors,
            ));
        }

        Ok(())
    }

    fn validate_first_name(&mut self, required: bool) -> Result<(), ApiFieldError> {
        match &self.first_name {
            Some(first_name) => {
                if first_name.len() < 3 {
                    Err(ApiFieldError::new_with_min_size(
                        ERR_MIN_SIZE,
                        "user.firstName".to_owned(),
                        3,
                    ))
                } else {
                    self.first_name = Some(first_name.to_uppercase());
                    Ok(())
                }
            }
            None => {
                if required {
                    Err(ApiFieldError::new(
                        ERR_REQUIRED_FIELD,
                        "user.firstName".to_owned(),
                    ))
                } else {
                    Ok(())
                }
            }
        }
    }

    fn validate_last_name(&mut self) -> Result<(), ApiFieldError> {
        if let Some(last_name) = &self.last_name {
            if last_name.len() < 2 {
                return Err(ApiFieldError::new_with_min_size(
                    ERR_MIN_SIZE,
                    "user.lastName".to_owned(),
                    2,
                ));
            } else {
                self.last_name = Some(last_name.to_uppercase());
            }
        }

        Ok(())
    }

    fn validate_email(&self) -> Result<(), ApiFieldError> {
        match &self.email {
            Some(email) => {
                if !util::validate_email(email.as_str()) {
                    Err(ApiFieldError::new(
                        ERR_INVALID_EMAIL,
                        "user.email".to_owned(),
                    ))
                } else {
                    Ok(())
                }
            }
            None => Err(ApiFieldError::new(
                ERR_REQUIRED_FIELD,
                "user.email".to_owned(),
            )),
        }
    }

    fn validate_password(&self) -> Result<(), ApiFieldError> {
        match &self.password {
            Some(password) => {
                if !util::validate_password(password.as_str()) {
                    Err(ApiFieldError::new(
                        ERR_INVALID_PASSWORD,
                        "user.password".to_owned(),
                    ))
                } else {
                    match &self.confirm_password {
                        Some(confirm_password) => {
                            if confirm_password != password {
                                Err(ApiFieldError::new(
                                    ERR_CONFIRM_PASSWORD_DIFF_PASSWORD,
                                    "user.confirmPassword".to_owned(),
                                ))
                            } else {
                                Ok(())
                            }
                        }
                        None => Err(ApiFieldError::new(
                            ERR_REQUIRED_FIELD,
                            "user.confirmPassword".to_owned(),
                        )),
                    }
                }
            }
            None => Err(ApiFieldError::new(
                ERR_REQUIRED_FIELD,
                "user.password".to_owned(),
            )),
        }
    }
}
