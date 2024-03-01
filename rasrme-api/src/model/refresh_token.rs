use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug)]
pub struct InsertRefreshToken {
    pub uuid: String,
    pub created_at: DateTime<Utc>,
    pub user_id: i64,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct RefreshToken {
    pub id: i64,
    pub uuid: String,
    pub revoked: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub user_id: i64,
}
