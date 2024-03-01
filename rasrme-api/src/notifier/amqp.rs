use std::fmt;

use amqprs::{
    channel::{
        BasicPublishArguments, Channel, ExchangeDeclareArguments, ExchangeType, QueueBindArguments,
        QueueDeclareArguments,
    },
    connection::{Connection, OpenConnectionArguments},
    BasicProperties,
};

use tracing::{error, info};

use crate::{model::MailEvent, notifier::UserMailValidationConsumer};

pub const EXCHANGE_NAME: &str = "rasrme.events";
pub const QUEUE_EVENT_MAIL: &str = "rasrme.event.user.mail.validation";
pub const ROUTING_KEY_EVENT_MAIL: &str = "event.user.mail.validation";

pub struct ArcturusAmqpConnChannel(pub Connection, pub Channel);

impl fmt::Debug for ArcturusAmqpConnChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ArcturusAmqpConnChannel")
            //.field("connection", &self.connection)
            .field("channel", &self.1.channel_id())
            .finish()
    }
}

#[derive(Debug)]
pub struct NotifierAmqp {
    pub amqp_host: String,
    pub amqp_port: u16,
    pub amqp_username: String,
    pub amqp_password: String,
    pub conn_channel_list: Vec<ArcturusAmqpConnChannel>,
}

impl NotifierAmqp {
    pub fn new() -> Self {
        let host = std::env::var("RABBITMQ_HOST")
            .expect("RabbitMQ Host is required to start Rasrme API.");
        let port = std::env::var("RABBITMQ_PORT")
            .expect("RabbitMQ Port is required to start Rasrme API.")
            .parse::<u16>()
            .expect("RabbitMQ Port needs to be a number.");
        let username = std::env::var("RABBITMQ_USERNAME")
            .expect("RabbitMQ Username is required to start Rasrme API.");
        let password = std::env::var("RABBITMQ_PASSWORD")
            .expect("RabbitMQ Password is required to start Rasrme API.");

        NotifierAmqp {
            amqp_host: host,
            amqp_port: port,
            amqp_username: username,
            amqp_password: password,
            conn_channel_list: Vec::new(),
        }
    }

    pub async fn init(&mut self) {
        // declare queues...
        let connection = self.connection().await;
        let channel = connection.open_channel(None).await.unwrap();

        // queue = rasrme.user.email
        info!(
            "Starting declaring queue {}, exchange {} and bind them.",
            QUEUE_EVENT_MAIL, EXCHANGE_NAME
        );

        // queue = rasrme.user.email
        let (queue_name, _, _) = channel
            .queue_declare(QueueDeclareArguments::durable_client_named(
                QUEUE_EVENT_MAIL,
            ))
            .await
            .unwrap()
            .unwrap();

        channel
            .exchange_declare(ExchangeDeclareArguments::new(
                EXCHANGE_NAME,
                ExchangeType::Direct.to_string().as_str(),
            ))
            .await
            .unwrap();

        channel
            .queue_bind(QueueBindArguments::new(
                &queue_name,
                EXCHANGE_NAME,
                ROUTING_KEY_EVENT_MAIL,
            ))
            .await
            .unwrap();

        info!("Finished declaring amqp resources.");

        info!("Starting config mail consumer.");
        let mail_conn_channel = UserMailValidationConsumer::config(&self).await;
        self.conn_channel_list.push(mail_conn_channel);
        info!("Finished config mail consumer.");
    }

    pub async fn connection(&self) -> Connection {
        let connection = Connection::open(&OpenConnectionArguments::new(
            self.amqp_host.to_owned().as_str(),
            self.amqp_port,
            self.amqp_username.to_owned().as_str(),
            self.amqp_password.to_owned().as_str(),
        ))
        .await
        .unwrap();

        connection
    }

    pub async fn send_mail_event(&self, event: MailEvent<'_>) {
        let connection = self.connection().await;
        let channel = connection.open_channel(None).await.unwrap();

        match serde_json::to_string(&event) {
            Ok(content) => {
                let args = BasicPublishArguments::new(EXCHANGE_NAME, ROUTING_KEY_EVENT_MAIL);
                if let Err(err) = channel
                    .basic_publish(
                        BasicProperties::default(),
                        content.as_bytes().to_vec(),
                        args,
                    )
                    .await
                {
                    error!("Error when publish an evento into queue: {}", err);
                }
            }
            Err(err) => {
                error!("Error when transform event into json: {}", err);
            }
        }
    }
}
