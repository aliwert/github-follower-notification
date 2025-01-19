use std::env;
use anyhow::Result;


pub struct Config {
    pub port: String,
    pub webhook_secret: Option<String>,
    pub whatsapp_config: Option<WhatsAppConfig>,
    pub telegram_config: Option<TelegramConfig>,
    pub discord_config: Option<DiscordConfig>,
    pub slack_config: Option<SlackConfig>,
    pub email_config: Option<EmailConfig>,
}

pub struct EmailConfig {
    pub smtp_server: String,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_email: String,
    pub to_email: String,
}
pub struct TelegramConfig {
    pub bot_token: String,
    pub chat_id: String,
}

pub struct DiscordConfig {
    pub webhook_url: String,
    pub bot_token: String,
}

pub struct SlackConfig {
    pub webhook_url: String,
    pub channel: String,
    pub bot_token: String,
}

pub struct WhatsAppConfig {
    pub api_key: String,
    pub phone_number: String,
}

impl Config {
    pub fn new() -> Result<Self> {
        Ok(Config {
            port: env::var("PORT").unwrap_or_else(|_| "8080".to_string()),
            webhook_secret: env::var("GITHUB_WEBHOOK_SECRET").ok(),
            whatsapp_config: Self::load_whatsapp_config(),
            telegram_config: Self::load_telegram_config(),
            discord_config: Self::load_discord_config(),
            slack_config: Self::load_slack_config(),
            email_config: Self::load_email_config(),
        })
    }
    fn load_telegram_config() -> Option<TelegramConfig> {
        match (env::var("TELEGRAM_BOT_TOKEN"), env::var("TELEGRAM_CHAT_ID")) {
            (Ok(bot_token), Ok(chat_id)) => Some(TelegramConfig {
                bot_token,
                chat_id,
            }),
            _ => None,
        }
    }

    fn load_whatsapp_config() -> Option<WhatsAppConfig> {
        match (env::var("WHATSAPP_API_KEY"), env::var("WHATSAPP_PHONE_NUMBER")) {
            (Ok(api_key), Ok(phone_number)) => Some(WhatsAppConfig {
                api_key,
                phone_number,
            }),
            _ => None,
        }
    }
    fn load_discord_config() -> Option<DiscordConfig> {
        match (env::var("DISCORD_WEBHOOK_URL"), env::var("DISCORD_BOT_TOKEN")) {
            (Ok(webhook_url), Ok(bot_token)) => Some(DiscordConfig {
                webhook_url,
                bot_token,
            }),
            _ => None,
        }
    }
    fn load_slack_config() -> Option<SlackConfig> {
        match (
            env::var("SLACK_WEBHOOK_URL"),
            env::var("SLACK_CHANNEL"),
            env::var("SLACK_BOT_TOKEN"),
        ) {
            (Ok(webhook_url), Ok(channel), Ok(bot_token)) => Some(SlackConfig {
                webhook_url,
                channel,
                bot_token,
            }),
            _ => None,
        }
    }
    fn load_email_config() -> Option<EmailConfig> {
        match (
            env::var("SMTP_SERVER"),
            env::var("SMTP_USERNAME"),
            env::var("SMTP_PASSWORD"),
            env::var("FROM_EMAIL"),
            env::var("TO_EMAIL"),
        ) {
            (Ok(smtp_server), Ok(smtp_username), Ok(smtp_password), Ok(from_email), Ok(to_email)) => {
                Some(EmailConfig {
                    smtp_server,
                    smtp_username,
                    smtp_password,
                    from_email,
                    to_email,
                })
            }
            _ => None,
        }
    }
}