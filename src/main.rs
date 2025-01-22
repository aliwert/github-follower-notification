use anyhow::Result;
use axum::{
    routing::post,
    Router,
};
use dotenv::dotenv;
use github_notification_service::{
    Config,
    NotificationManager,
    handlers::webhook::handle_webhook,
};
use std::{sync::Arc, net::SocketAddr};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::new()?;
    let manager = Arc::new(NotificationManager::new(&config)?);

    //TODO: Implement the webhook handler
    let app = Router::new()
        .route("/webhook", post(handle_webhook))
        .with_state(manager)
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port.parse()?));
    info!("Server starting on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}