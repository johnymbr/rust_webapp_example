use std::sync::OnceLock;

use minijinja::{context, path_loader, Environment};
use tracing::error;

use crate::{
    exception::{ApiError, ERR_TEMPLATE_ERROR},
    model::MailEvent,
};

static ENGINE: OnceLock<Environment<'static>> = OnceLock::new();
const MAIL_VALIDATION_TMLP: &str = "mail_validation.html";

pub fn get_user_validation_email_tmpl(template_dir: &str, event: &MailEvent) -> Result<String, ApiError> {
    let engine = ENGINE.get_or_init(|| {
        let mut env = Environment::new();
        env.set_loader(path_loader(template_dir));
        env
    });

    match engine.get_template(MAIL_VALIDATION_TMLP) {
        Ok(tmpl) => {
            let ctx = context! {
                name => event.name,
                token => event.token,
            };

            match tmpl.render(ctx) {
                Ok(result) => Ok(result),
                Err(_) => Err(ApiError::new(ERR_TEMPLATE_ERROR)),
            }
        }
        Err(err) => {
            error!("{}", err);
            Err(ApiError::new(ERR_TEMPLATE_ERROR))
        }
    }
}
