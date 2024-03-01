use std::sync::Arc;

use crate::notifier::NotifierAmqp;

use axum::{http::StatusCode, Json, Router};
use config::{Db, JwtTokenConfig};
use controller::{AuthController, UserController};
use serde_json::{json, Value};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod config;
pub mod controller;
pub mod exception;
pub mod model;
pub mod notifier;
pub mod repository;
pub mod service;
pub mod util;

#[tokio::main]
async fn main() {
    // load .env file
    dotenvy::from_filename(".env").ok();

    // initializing tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "rasrme_api=trace,tower_http=trace,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Config jwt keys...");
    JwtTokenConfig::config();

    info!("Config pg_pool...");
    let pg_pool = Arc::new(Db::config().await);
    
    info!("Create Rasrme AMQP...");
    let mut amqp = NotifierAmqp::new();
    amqp.init().await;
    let arc_amqp = Arc::new(amqp);

    let state = Arc::new(ArcturusState {
        pg_pool: pg_pool.clone(),
        amqp: arc_amqp.clone(),
    });

    // build our application with a route
    let app = Router::new()
        .merge(AuthController::routes(state).await)
        .merge(
            UserController::new()
                .routes(Arc::clone(&pg_pool), Arc::clone(&arc_amqp))
                .await,
        )
        .layer(TraceLayer::new_for_http())
        .fallback(api_fallback);

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn api_fallback() -> (StatusCode, Json<Value>) {
    let body = json!({
        "status": 404,
        "message": "Not Found",
    });
    (StatusCode::NOT_FOUND, Json(body))
}

pub struct ArcturusState {
    pub pg_pool: Arc<PgPool>,
    pub amqp: Arc<NotifierAmqp>,
}
