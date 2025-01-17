use crate::models::WhatsAppMessage;
use anyhow::{Result, Context};
use reqwest::Client;
use tracing::{info, error};

pub struct WhatsAppService {
    client: Client,
    api_key: String,
    phone_number: String,
}

impl WhatsAppService {
    pub fn new(api_key: String, phone_number: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            phone_number,
        }
    }
}

#[async_trait::async_trait]
impl super::NotificationService for WhatsAppService {
    async fn send(&self, title: &str, message: &str) -> Result<()> {
        let url = "https://graph.facebook.com/v13.0/YOUR_PHONE_NUMBER_ID/messages";
        
        let payload = WhatsAppMessage {
            messaging_product: "whatsapp".to_string(),
            to: self.phone_number.clone(),
            msg_type: "text".to_string(),
            text: crate::models::WhatsAppText {
                body: format!("{}\n{}", title, message),
            },
        };

        let response = self.client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await
            .context("Failed to send WhatsApp message")?;

        if !response.status().is_success() {
            error!("WhatsApp API error: {:?}", response.text().await?);
            return Err(anyhow::anyhow!("WhatsApp API error"));
        }

        info!("WhatsApp notification sent successfully");
        Ok(())
    }
}