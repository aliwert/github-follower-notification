use crate::models::TelegramMessage;
use anyhow::{Result, Context};
use reqwest::Client;
use tracing::{info, error};

pub struct TelegramService {
    client: Client,
    bot_token: String,
    chat_id: String,
}

impl TelegramService {
    pub fn new(bot_token: String, chat_id: String) -> Self {
        Self {
            client: Client::new(),
            bot_token,
            chat_id,
        }
    }
}

#[async_trait::async_trait]
impl super::NotificationService for TelegramService {
    async fn send(&self, title: &str, message: &str) -> Result<()> {
        let url = format!("https://api.telegram.org/bot{}/sendMessage", self.bot_token);
        
        let payload = TelegramMessage {
            chat_id: self.chat_id.clone(),
            text: format!("*{}*\n{}", title, message),
            parse_mode: "Markdown".to_string(),
        };

        let response = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .context("Failed to send Telegram message")?;

        if !response.status().is_success() {
            error!("Telegram API error: {:?}", response.text().await?);
            return Err(anyhow::anyhow!("Telegram API error"));
        }

        info!("Telegram notification sent successfully");
        Ok(())
    }
}