use async_trait::async_trait;
use reqwest::Client;

use serde_json::json;
use crate::common::{config::AppConfig, errors::AppError};
use super::super::domain::service::{EmailService, EmailRecipient};
use std::fs;

pub struct ResendEmailService {
    client: Client,
    config: AppConfig,
}

impl ResendEmailService {
    pub fn new(config: AppConfig) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    async fn send(&self, to: &str, subject: &str, html: String, text: String) -> Result<(), AppError> {
        let url = "https://api.resend.com/emails";
        let api_key = &self.config.resend_api_key;
        
        // Use configured sender
        let from = &self.config.email_from; 


        let body = json!({
            "from": from,
            "to": to,
            "subject": subject,
            "html": html,
            "text": text
        });

        let response = self.client.post(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to send email: {:?}", e);
                AppError::InternalError
            })?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            tracing::error!("Resend API error: {} - {}", status, text);
            Err(AppError::InternalError)
        }
    }

    fn read_template(&self, name: &str) -> Result<String, AppError> {
        // In production, templates might be compiled in or cached.
        // For this template, we read from disk.
        // Assuming running from project root.
        let path = format!("src/email_templates/{}", name);
        fs::read_to_string(&path).map_err(|e| {
            tracing::error!("Failed to read email template {}: {:?}", path, e);
            AppError::InternalError
        })
    }

    fn format_recipient(recipient: &EmailRecipient) -> String {
        match &recipient.name {
            Some(name) => format!("{} <{}>", name, recipient.email),
            None => recipient.email.clone(),
        }
    }
}

#[async_trait]
impl EmailService for ResendEmailService {
    async fn send_verification_email(&self, recipient: &EmailRecipient, token: &str) -> Result<(), AppError> {
        let link = format!("{}/auth/verify-email?token={}", self.config.app_url, token);
        let html_template = self.read_template("verification.html")?;
        let text_template = self.read_template("verification.txt")?;

        let html = html_template.replace("{{verification_link}}", &link);
        let text = text_template.replace("{{verification_link}}", &link);

        let to = Self::format_recipient(recipient);
        self.send(&to, "Verify your email", html, text).await
    }

    async fn send_password_reset_email(&self, recipient: &EmailRecipient, token: &str) -> Result<(), AppError> {
        let link = format!("{}/auth/reset-password?token={}", self.config.app_url, token);
        let html_template = self.read_template("password_reset.html")?;
        let text_template = self.read_template("password_reset.txt")?;

        let html = html_template.replace("{{reset_link}}", &link);
        let text = text_template.replace("{{reset_link}}", &link);

        let to = Self::format_recipient(recipient);
        self.send(&to, "Reset your password", html, text).await
    }
}
