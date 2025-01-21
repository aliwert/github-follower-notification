use create::{
    models::FollowerEvent,
    services::NotificationManager,
};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use std::sync::Arc;
use notify_rust::Notification;
use tracing::{info, error, warn};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use hex;

pub async fn handle_webhook(
    State(manager): State<Arc<NotificationManager>>,
    headers: HeaderMap,
    body: String,
    Json(event): Json<FollowerEvent>,
) -> StatusCode {
    // Verify GitHub signature
    let signature = match headers.get("X-Hub-Signature-256") {
        Some(sig) => sig.to_str().unwrap_or("").trim_start_matches("sha256="),
        None => {
            error!("Missing signature header");
            return StatusCode::UNAUTHORIZED;
        }
    };

    // Verify webhook signature
    match verify_signature(&headers, body.as_bytes(), &manager.webhook_secret) {
        Ok(true) => (),
        Ok(false) => {
            error!("Invalid signature");
            return StatusCode::UNAUTHORIZED;
        }
        Err(e) => {
            error!("Signature verification error: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    }

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
            if let Err(e) = manager.notify_all(&title, &message).await {
                error!("Failed to send notifications: {}", e);
                return StatusCode::INTERNAL_SERVER_ERROR;
            }

            // Show desktop notification
            if let Err(e) = Notification::new()
                .summary(title)
                .body(&message)
                .icon("github")
                .show() {
                warn!("Failed to show desktop notification: {}", e);
            }

            StatusCode::OK
        }
        _ => {
            warn!("Unsupported event action: {}", event.action);
            StatusCode::BAD_REQUEST
        }
    }
}

fn verify_signature(headers: &HeaderMap, body: &[u8], secret: &str) -> anyhow::Result<bool> {
    let signature = match headers.get("X-Hub-Signature-256") {
        Some(sig) => sig.to_str()?.trim_start_matches("sha256="),
        None => return Ok(false),
    };

    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())?;
    mac.update(body);
    let result = mac.finalize().into_bytes();
    let expected = hex::decode(signature)?;

    Ok(result.as_slice() == expected.as_slice())
}