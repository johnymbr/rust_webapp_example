use serde::{Deserialize, Serialize};

use super::User;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug)]
pub struct LoginResponse {
    pub user: User,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessTokenRequest {
    pub refreh_token: Option<String>,
    pub grant_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ForgotPasswordRequest {
    pub email: Option<String>,
    pub token: Option<String>,
    pub password: Option<String>,
    pub confirm_password: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailValidationRequest {
    pub email: Option<String>,
    pub token: Option<String>,
}
