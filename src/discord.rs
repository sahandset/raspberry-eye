use anyhow::{Context, Result};
use log::{debug, info};
use reqwest::multipart;
use std::path::Path;
use tokio::fs;

/// Discord webhook client
pub struct DiscordClient {
    webhook_url: String,
    client: reqwest::Client,
}

impl DiscordClient {
    /// Create a new Discord client
    pub fn new(webhook_url: String) -> Self {
        Self {
            webhook_url,
            client: reqwest::Client::new(),
        }
    }

    /// Send a motion detection notification with image
    pub async fn send_motion_alert<P: AsRef<Path>>(
        &self,
        image_path: P,
        timestamp: &str,
    ) -> Result<()> {
        let image_path = image_path.as_ref();

        info!("Sending Discord notification for image: {:?}", image_path);

        // Read the image file
        let image_data = fs::read(image_path)
            .await
            .context("Failed to read image file")?;

        let filename = image_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("motion.jpg");

        // Create message content
        let message = format!(
            "**Motion Detected!**\n\nTimestamp: `{}`\nLocation: Raspberry Pi",
            timestamp
        );

        // Build multipart form
        let image_part = multipart::Part::bytes(image_data)
            .file_name(filename.to_string())
            .mime_str("image/jpeg")?;

        let form = multipart::Form::new()
            .text("content", message)
            .part("file", image_part);

        // Send the webhook request
        let response = self
            .client
            .post(&self.webhook_url)
            .multipart(form)
            .send()
            .await
            .context("Failed to send Discord webhook")?;

        if response.status().is_success() {
            info!("Discord notification sent successfully");
            Ok(())
        } else {
            let status = response.status();
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to read response body".to_string());

            anyhow::bail!(
                "Discord webhook failed with status {}: {}",
                status,
                body
            )
        }
    }

    /// Test the webhook connection
    pub async fn test_connection(&self) -> Result<()> {
        debug!("Testing Discord webhook connection");

        let form = multipart::Form::new()
            .text("content", "Raspberry Eye: Connection test successful!");

        let response = self
            .client
            .post(&self.webhook_url)
            .multipart(form)
            .send()
            .await
            .context("Failed to send test webhook")?;

        if response.status().is_success() {
            info!("Discord connection test successful");
            Ok(())
        } else {
            anyhow::bail!(
                "Discord connection test failed with status: {}",
                response.status()
            )
        }
    }
}
