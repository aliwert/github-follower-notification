use axum::{
    extract::{Json, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use hmac::{Hmac, Mac};
use notify_rust::Notification;
use sha2::Sha256;
use std::sync::Arc;
use tracing::{info, warn};

use super::HandlerError;
use crate::{models::FollowerEvent, services::NotificationManager};

pub async fn handle_webhook(
    State(manager): State<Arc<NotificationManager>>,
    headers: HeaderMap,
    Json(event): Json<FollowerEvent>,
) -> Result<impl IntoResponse, HandlerError> {
    // convert the event back to JSON string for signature verification
    let body_str = serde_json::to_string(&event)
        .map_err(|e| HandlerError::ValidationError(format!("Failed to serialize event: {}", e)))?;

    // verify GitHub signature
    let signature = headers
        .get("X-Hub-Signature-256")
        .and_then(|sig| sig.to_str().ok())
        .map(|sig| sig.trim_start_matches("sha256="))
        .ok_or_else(|| HandlerError::AuthenticationError("Missing signature header".into()))?;

    // verify webhook signature
    verify_signature(&body_str, signature, &manager.webhook_secret)
        .map_err(|e| HandlerError::AuthenticationError(format!("Invalid signature: {}", e)))?;

    // process the event
    match event.action.as_str() {
        "followed" => {
            info!("New follower: {}", event.sender.login);

            let title = "New GitHub Follower!";
            let message = format!(
                "User {} is now following you!\nProfile: {}",
                event.sender.login, event.sender.html_url
            );

            // send notifications
            manager
                .notify_all(&title, &message)
                .await
                .map_err(|e| HandlerError::NotificationError(e.to_string()))?;

            // show desktop notification
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
            Err(HandlerError::ValidationError(
                "Unsupported event action".into(),
            ))
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
