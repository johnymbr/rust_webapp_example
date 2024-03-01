use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MailEvent<'a> {
    pub subject: String,
    pub to: &'a str,
    pub name: &'a str,
    pub token: &'a str,
}
