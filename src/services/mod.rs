use anyhow::Result;
use discord::DiscordService;
use email::EmailService;
use slack::SlackService;
use telegram::TelegramService;
use whatsapp::WhatsAppService;

pub mod discord;
pub mod email;
pub mod slack;
pub mod telegram;
pub mod whatsapp;

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
        // init services based on available config
        let whatsapp = if let Some(whatsapp_config) = &config.whatsapp_config {
            Some(WhatsAppService::new(
                whatsapp_config.api_key.clone(),
                whatsapp_config.phone_number.clone(),
            ))
        } else {
            None
        };

        let telegram = if let Some(telegram_config) = &config.telegram_config {
            Some(TelegramService::new(
                telegram_config.bot_token.clone(),
                telegram_config.chat_id.clone(),
            ))
        } else {
            None
        };

        let discord = if let Some(discord_config) = &config.discord_config {
            Some(DiscordService::new(discord_config.webhook_url.clone()))
        } else {
            None
        };

        let slack = if let Some(slack_config) = &config.slack_config {
            Some(SlackService::new(slack_config.webhook_url.clone()))
        } else {
            None
        };

        let email = if let Some(email_config) = &config.email_config {
            Some(EmailService::new(
                email_config.smtp_server.clone(),
                email_config.smtp_username.clone(),
                email_config.smtp_password.clone(),
                email_config.from_email.clone(),
                email_config.to_email.clone(),
            )?)
        } else {
            None
        };

        Ok(Self {
            webhook_secret: config.webhook_secret.clone().unwrap_or_default(),
            whatsapp,
            telegram,
            discord,
            slack,
            email,
        })
    }

    pub async fn notify_all(&self, title: &str, message: &str) -> Result<()> {
        let mut errors = Vec::new();

        // send notifications to all configured services
        if let Some(whatsapp) = &self.whatsapp {
            if let Err(e) = whatsapp.send(title, message).await {
                errors.push(format!("WhatsApp: {}", e));
            }
        }

        if let Some(telegram) = &self.telegram {
            if let Err(e) = telegram.send(title, message).await {
                errors.push(format!("Telegram: {}", e));
            }
        }

        if let Some(discord) = &self.discord {
            if let Err(e) = discord.send(title, message).await {
                errors.push(format!("Discord: {}", e));
            }
        }

        if let Some(slack) = &self.slack {
            if let Err(e) = slack.send(title, message).await {
                errors.push(format!("Slack: {}", e));
            }
        }

        if let Some(email) = &self.email {
            if let Err(e) = email.send(title, message).await {
                errors.push(format!("Email: {}", e));
            }
        }

        // if there were errors but at least one service succeeded, log warnings
        // if all services failed, return an error
        if !errors.is_empty() {
            let error_msg = format!("Some notifications failed: {}", errors.join(", "));
            tracing::warn!("{}", error_msg);

            // only return error if ALL configured services failed
            let total_services = [
                self.whatsapp.is_some(),
                self.telegram.is_some(),
                self.discord.is_some(),
                self.slack.is_some(),
                self.email.is_some(),
            ]
            .iter()
            .filter(|&&is_configured| is_configured)
            .count();

            if errors.len() == total_services && total_services > 0 {
                return Err(anyhow::anyhow!(error_msg));
            }
        }

        Ok(())
    }
}
