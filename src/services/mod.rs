use anyhow::Result;

pub mod whatsapp;
pub mod telegram;
pub mod discord;
pub mod slack;
pub mod email;

#[async_trait::async_trait]
pub trait NotificationService: Send + Sync {
    async fn send(&self, title: &str, message: &str) -> Result<()>;
}

pub use whatsapp::WhatsAppService;
pub use telegram::TelegramService;
pub use discord::DiscordService;
pub use slack::SlackService;
pub use email::EmailService;