use crate::models::DiscordMessage;
use anyhow::{Result, Context};
use reqwest::Client;
use tracing::{info, error};

pub struct DiscordService {
    client: Client,
    webhook_url: String,
}

impl DiscordService {
    pub fn new(webhook_url: String) -> Self {
        Self {
            client: Client::new(),
            webhook_url,
        }
    }
}

#[async_trait::async_trait]
impl super::NotificationService for DiscordService {
    async fn send(&self, title: &str, message: &str) -> Result<()> {
        let payload = DiscordMessage {
            content: format!("**{}**\n{}", title, message),
            username: "GitHub Follower Bot".to_string(),
            avatar_url: Some(
                "https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png"
                    .to_string()
            ),
        };

        let response = self.client
            .post(&self.webhook_url)
            .json(&payload)
            .send()
            .await
            .context("Failed to send Discord message")?;

        if !response.status().is_success() {
            error!("Discord API error: {:?}", response.text().await?);
            return Err(anyhow::anyhow!("Discord API error"));
        }

        info!("Discord notification sent successfully");
        Ok(())
    }
}