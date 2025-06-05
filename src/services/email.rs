use anyhow::Result;
use lettre::{
    message::Message, transport::smtp::authentication::Credentials, SmtpTransport, Transport,
};
use tracing::{error, info};

pub struct EmailService {
    mailer: SmtpTransport,
    from_email: String,
    to_email: String,
}

impl EmailService {
    pub fn new(
        smtp_server: String,
        smtp_username: String,
        smtp_password: String,
        from_email: String,
        to_email: String,
    ) -> Result<Self> {
        let creds = Credentials::new(smtp_username, smtp_password);

        let mailer = SmtpTransport::relay(&smtp_server)?
            .credentials(creds)
            .build();

        Ok(Self {
            mailer,
            from_email,
            to_email,
        })
    }
}

#[async_trait::async_trait]
impl super::NotificationService for EmailService {
    async fn send(&self, title: &str, message: &str) -> Result<()> {
        let email = Message::builder()
            .from(self.from_email.parse()?)
            .to(self.to_email.parse()?)
            .subject(title)
            .body(message.to_string())?;

        match self.mailer.send(&email) {
            Ok(_) => {
                info!("Email notification sent successfully");
                Ok(())
            }
            Err(e) => {
                error!("Failed to send email: {}", e);
                Err(e.into())
            }
        }
    }
}
