use crate::models::SlackMessage;
use anyhow::{Result, Context};
use reqwest::Client;
use tracing::{info, error};

pub struct SlackService {
    client: Client,
    webhook_url: String,
}

impl SlackService {
    pub fn new(webhook_url: String) -> Self {
        Self {
            client: Client::new(),
            webhook_url,
        }
    }
}

#[async_trait::async_trait]
impl super::NotificationService for SlackService {
    async fn send(&self, title: &str, message: &str) -> Result<()> {
        let payload = SlackMessage {
            text: format!("*{}*\n{}", title, message),
        };

        let response = self.client
            .post(&self.webhook_url)
            .json(&payload)
            .send()
            .await
            .context("Failed to send Slack message")?;

        if !response.status().is_success() {
            error!("Slack API error: {:?}", response.text().await?);
            return Err(anyhow::anyhow!("Slack API error"));
        }

        info!("Slack notification sent successfully");
        Ok(())
    }
}