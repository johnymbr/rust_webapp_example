use fancy_regex::Regex;
use tracing::error;

// static EMAIL_RE: OnceLock<Regex> = OnceLock::new();

pub fn validate_email(email: &str) -> bool {
    if let Ok(email_regex) = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    ) {
        return email_regex.is_match(email).unwrap_or(false);
    }

    false
}

pub fn validate_password(password: &str) -> bool {
    match Regex::new(r"^((?=.*[a-z])(?=.*[@#$%])(?=.*\d).{8,16})") {
        Ok(password_regex) => {
            password_regex.is_match(password).unwrap_or(false)
        }
        Err(err) => {
            error!("{}", err);
            false
        }
    }
}