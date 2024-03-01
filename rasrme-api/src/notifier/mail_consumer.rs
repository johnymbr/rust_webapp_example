use amqprs::{
    channel::{BasicAckArguments, BasicConsumeArguments, Channel},
    consumer::AsyncConsumer,
    BasicProperties, Deliver,
};
use axum::async_trait;
use lettre::{
    message::{Mailbox, MultiPart, SinglePart},
    transport::smtp::{authentication::Credentials, client::Tls},
    Message, SmtpTransport, Transport,
};
use tracing::{error, info};

use crate::{config::get_user_validation_email_tmpl, model::MailEvent};

use super::{ArcturusAmqpConnChannel, NotifierAmqp, QUEUE_EVENT_MAIL};

struct MailConfig {
    host: String,
    port: u16,
    enable_auth: bool,
    enable_starttls: bool,
    username: String,
    password: Option<String>,
    template_dir: String,
}

impl MailConfig {
    fn new(
        host: String,
        port: u16,
        enable_auth: bool,
        enable_starttls: bool,
        username: String,
        password: Option<String>,
        template_dir: String,
    ) -> MailConfig {
        MailConfig {
            host,
            port,
            enable_auth,
            enable_starttls,
            username,
            password,
            template_dir,
        }
    }
}

pub struct UserMailValidationConsumer {
    mail_config: MailConfig,
}

impl UserMailValidationConsumer {
    pub fn new() -> Self {
        let host =
            std::env::var("MAIL_SMTP_HOST").expect("SMTP Host is required to config mail sender.");
        let port = std::env::var("MAIL_SMTP_PORT")
            .expect("SMTP Port is required to config mail sender.")
            .parse::<u16>()
            .expect("SMTP Port is required to config mail sender.");
        let enable_starttls = std::env::var("MAIL_SMTP_STARTTLS")
            .unwrap_or(String::from("false"))
            .parse::<bool>()
            .expect("SMTP Starttls is required to config mail sender.");
        let enable_auth = std::env::var("MAIL_SMTP_AUTH")
            .unwrap_or(String::from("false"))
            .parse::<bool>()
            .expect("SMTP Auth is required to config mail sender.");
        let template_dir = std::env::var("MAIL_TEMPLATES_DIR").expect("Mail templates dir is required to config mail sender.");

        let username = std::env::var("MAIL_SMTP_USERNAME")
            .expect("SMTP Username is required to config mail sender.");
        let mut password = None;
        if enable_auth {
            password = Some(
                std::env::var("RABBITMQ_PASSWORD")
                    .expect("RabbitMQ Password is required to start Rasrme API."),
            );
        }

        UserMailValidationConsumer {
            mail_config: MailConfig::new(
                host,
                port,
                enable_auth,
                enable_starttls,
                username,
                password,
                template_dir,
            ),
        }
    }

    pub async fn config(amqp: &NotifierAmqp) -> ArcturusAmqpConnChannel {
        let connection = amqp.connection().await;
        let channel = connection.open_channel(None).await.unwrap();

        let args = BasicConsumeArguments::new(QUEUE_EVENT_MAIL, "consumer.event.mail");
        channel
            .basic_consume(UserMailValidationConsumer::new(), args)
            .await
            .unwrap();

        ArcturusAmqpConnChannel {
            0: connection,
            1: channel,
        }
    }

    fn send(&self, subject: String, to: &str, content: String) {
        let from_address = self.mail_config.username.parse().unwrap();
        let from_mail = Mailbox::new(None, from_address);

        let to_address = to.parse().unwrap();
        let to_mail = Mailbox::new(None, to_address);

        let email = Message::builder()
            .from(from_mail)
            .to(to_mail)
            .subject(subject)
            .multipart(MultiPart::alternative().singlepart(SinglePart::html(content)))
            .unwrap();

        let mailer;
        if self.mail_config.enable_starttls {
            if self.mail_config.enable_auth {
                let credentials = Credentials::new(
                    self.mail_config.username.clone(),
                    self.mail_config.password.as_ref().unwrap().clone(),
                );

                mailer = SmtpTransport::starttls_relay(&self.mail_config.host)
                    .unwrap()
                    .credentials(credentials)
                    .port(self.mail_config.port)
                    .build();
            } else {
                mailer = SmtpTransport::starttls_relay(&self.mail_config.host)
                    .unwrap()
                    .port(self.mail_config.port)
                    .build();
            }
        } else {
            if self.mail_config.enable_auth {
                let credentials = Credentials::new(
                    self.mail_config.username.clone(),
                    self.mail_config.password.as_ref().unwrap().clone(),
                );

                mailer = SmtpTransport::relay(&self.mail_config.host)
                    .unwrap()
                    .port(self.mail_config.port)
                    .credentials(credentials)
                    .build();
            } else {
                mailer = SmtpTransport::relay(&self.mail_config.host)
                    .unwrap()
                    .port(self.mail_config.port)
                    .tls(Tls::None)
                    .build();
            }
        }

        match mailer.send(&email) {
            Ok(_) => info!("Email sent successfully!"),
            Err(e) => error!("Could not send email: {:?}", e),
        }
    }
}

#[async_trait]
impl AsyncConsumer for UserMailValidationConsumer {
    async fn consume(
        &mut self, // use `&mut self` to make trait object to be `Sync`
        channel: &Channel,
        deliver: Deliver,
        _basic_properties: BasicProperties,
        content: Vec<u8>,
    ) {
        info!(
            "consume delivery {} on channel {}, content size: {}",
            deliver,
            channel,
            content.len()
        );

        if let Ok(event) = serde_json::from_str::<MailEvent>(
            String::from_utf8_lossy(&content).to_string().as_str(),
        ) {
            if let Ok(tmpl) = get_user_validation_email_tmpl(&self.mail_config.template_dir, &event) {
                // info!(tmpl);

                self.send(event.subject, event.to, tmpl);

                // ack explicitly if manual ack
                info!("ack to delivery {} on channel {}", deliver, channel);
                let args = BasicAckArguments::new(deliver.delivery_tag(), false);
                channel.basic_ack(args).await.unwrap();
            }
        } else {
            error!("Error when deserialize a MailEvent.");
        }
    }
}
