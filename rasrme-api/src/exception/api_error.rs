use core::fmt;

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use hyper::{Error, StatusCode};
use serde::{Deserialize, Serialize};

use super::{ApiErrorCode, ERR_HYPER_ERROR};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiError {
    #[serde(rename = "status")]
    pub status_code: u16,
    pub code: String,
    pub message: String,
    #[serde(rename = "fieldErrors")]
    pub field_errors: Option<Vec<ApiFieldError>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiFieldError {
    pub code: String,
    pub message: String,
    pub field: String,
    #[serde(rename = "minSize")]
    pub min_size: Option<u16>,
    #[serde(rename = "maxSize")]
    pub max_size: Option<u16>,
}

impl ApiError {
    pub fn new(api_error_cde: ApiErrorCode) -> ApiError {
        ApiError {
            status_code: 412,
            code: String::from(api_error_cde.0),
            message: String::from(api_error_cde.1),
            field_errors: None,
        }
    }

    pub fn new_with_status(status: StatusCode, api_error_cde: ApiErrorCode) -> ApiError {
        ApiError {
            status_code: status.as_u16(),
            code: String::from(api_error_cde.0),
            message: String::from(api_error_cde.1),
            field_errors: None,
        }
    }

    pub fn new_with_field_errors(
        api_error_cde: ApiErrorCode,
        field_errors: Vec<ApiFieldError>,
    ) -> ApiError {
        ApiError {
            status_code: 412,
            code: String::from(api_error_cde.0),
            message: String::from(api_error_cde.1),
            field_errors: Some(field_errors),
        }
    }
}

impl ApiFieldError {
    pub fn new(api_error_code: ApiErrorCode, field: String) -> ApiFieldError {
        ApiFieldError {
            code: String::from(api_error_code.0),
            message: String::from(api_error_code.1),
            field,
            min_size: None,
            max_size: None,
        }
    }

    pub fn new_with_min_size(
        api_error_code: ApiErrorCode,
        field: String,
        min_size: u16,
    ) -> ApiFieldError {
        let mut api_field_error = ApiFieldError::new(api_error_code, field);
        api_field_error.min_size = Some(min_size);
        api_field_error
    }

    pub fn new_with_max_size(
        api_error_code: ApiErrorCode,
        field: String,
        max_size: u16,
    ) -> ApiFieldError {
        let mut api_field_error = ApiFieldError::new(api_error_code, field);
        api_field_error.max_size = Some(max_size);
        api_field_error
    }

    pub fn new_with_min_and_max_size(
        api_error_code: ApiErrorCode,
        field: String,
        min_size: u16,
        max_size: u16,
    ) -> ApiFieldError {
        let mut api_field_error = ApiFieldError::new_with_min_size(api_error_code, field, min_size);
        api_field_error.max_size = Some(max_size);
        api_field_error
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(format!("{} - {}", self.code, self.message).as_str())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = match StatusCode::from_u16(self.status_code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status_code, Json(self)).into_response()
    }
}

impl From<Error> for ApiError {
    fn from(_e: Error) -> Self {
        ApiError::new_with_status(StatusCode::INTERNAL_SERVER_ERROR, ERR_HYPER_ERROR)
    }
}

impl fmt::Display for ApiFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            format!(
                "[{}] {} | {} | Min Size: {:?}, Max Size: {:?}",
                self.code, self.message, self.field, self.min_size, self.max_size
            )
            .as_str(),
        )
    }
}
