use anyhow::Result;
use discord::DiscordService;
use email::EmailService;
use slack::SlackService;
use telegram::TelegramService;
use whatsapp::WhatsAppService;

pub mod whatsapp;
pub mod telegram;
pub mod discord;
pub mod slack;
pub mod email;

#[async_trait::async_trait]
pub trait NotificationService: Send + Sync {
    async fn send(&self, title: &str, message: &str) -> Result<()>;
}

pub struct NotificationManager {
    pub webhook_secret: String,
    whatsapp: Option<WhatsAppService>,
    telegram: Option<TelegramService>,
    discord: Option<DiscordService>,
    slack: Option<SlackService>,
    email: Option<EmailService>,
}

impl NotificationManager {
    pub fn new(config: &crate::config::Config) -> Result<Self> {
        Ok(Self {
            webhook_secret: config.webhook_secret.clone().unwrap_or_default(),
            whatsapp: None, // Initialize services based on config
            telegram: None,
            discord: None,
            slack: None,
            email: None,
        })
    }

    pub async fn notify_all(&self, title: &str, message: &str) -> Result<()> {
        Ok(())
    }
}