use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use tracing::{error, info, warn};
use notify_rust::Notification;
use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::{
    models::FollowerEvent,
    services::NotificationManager,
};
use super::HandlerError;

pub async fn handle_webhook(
    State(manager): State<Arc<NotificationManager>>,
    headers: HeaderMap,
    body: String,
    Json(event): Json<FollowerEvent>,
) -> Result<impl IntoResponse, HandlerError> {
    // Verify GitHub signature
    let signature = headers
        .get("X-Hub-Signature-256")
        .and_then(|sig| sig.to_str().ok())
        .map(|sig| sig.trim_start_matches("sha256="))
        .ok_or_else(|| HandlerError::AuthenticationError("Missing signature header".into()))?;

    // Verify webhook signature
    verify_signature(&body, signature, &manager.webhook_secret)
        .map_err(|e| HandlerError::AuthenticationError(format!("Invalid signature: {}", e)))?;

    // Process the event
    match event.action.as_str() {
        "followed" => {
            info!("New follower: {}", event.sender.login);
            
            let title = "New GitHub Follower!";
            let message = format!(
                "User {} is now following you!\nProfile: {}",
                event.sender.login,
                event.sender.html_url
            );

            // Send notifications
            manager.notify_all(&title, &message)
                .await
                .map_err(|e| HandlerError::NotificationError(e.to_string()))?;

            // Show desktop notification
            if let Err(e) = Notification::new()
                .summary(title)
                .body(&message)
                .icon("github")
                .show() 
            {
                warn!("Failed to show desktop notification: {}", e);
            }

            Ok(StatusCode::OK)
        }
        _ => {
            warn!("Unsupported event action: {}", event.action);
            Err(HandlerError::ValidationError("Unsupported event action".into()))
        }
    }
}

fn verify_signature(payload: &str, signature: &str, secret: &str) -> anyhow::Result<()> {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())?;
    mac.update(payload.as_bytes());
    
    let decoded_signature = hex::decode(signature)?;
    mac.verify_slice(&decoded_signature)
        .map_err(|_| anyhow::anyhow!("Invalid signature"))
}