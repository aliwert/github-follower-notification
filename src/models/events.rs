use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct FollowerEvent {
    pub action: String,
    pub sender: Sender,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Sender {
    pub login: String,
    pub avatar_url: String,
    pub html_url: String,
}

#[derive(Serialize)]
pub struct WhatsAppMessage {
    pub messaging_product: String,
    pub to: String,
    #[serde(rename = "type")]
    pub msg_type: String,
    pub text: WhatsAppText,
}

#[derive(Serialize)]
pub struct WhatsAppText {
    pub body: String,
}

#[derive(Serialize)]
pub struct TelegramMessage {
    pub chat_id: String,
    pub text: String,
    pub parse_mode: String,
}

#[derive(Serialize)]
pub struct DiscordMessage {
    pub content: String,
    pub username: String,
    pub avatar_url: Option<String>,
}

#[derive(Serialize)]
pub struct SlackMessage {
    pub text: String,
}
