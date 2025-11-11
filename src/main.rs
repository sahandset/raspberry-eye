mod camera;
mod config;
mod discord;
mod motion;

use anyhow::{Context, Result};
use chrono::Local;
use log::{error, info, warn};
use std::env;

use camera::Camera;
use config::Config;
use discord::DiscordClient;
use motion::MotionDetector;

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::init();

    if let Err(e) = run().await {
        error!("Application error: {:#}", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    info!("Starting Raspberry Eye motion detection system");

    // Load configuration
    let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| "config.yaml".to_string());
    let config = Config::from_file(&config_path)
        .context("Failed to load configuration")?;

    info!("Configuration loaded from: {}", config_path);

    // Initialize logging level from config
    if let Ok(level) = env::var("RUST_LOG") {
        info!("Log level set to: {}", level);
    } else {
        info!("Log level set to: {} (from config)", config.logging.level);
    }

    // Initialize camera
    let camera = Camera::new(
        &config.camera.script_path,
        &config.camera.output_dir,
        config.camera.filename_format.clone(),
        config.camera.resolution.width,
        config.camera.resolution.height,
    );

    // Test camera on startup
    info!("Testing camera connection...");
    camera.test().context("Camera test failed")?;

    // Initialize Discord client
    let discord = DiscordClient::new(config.discord.webhook_url.clone());

    // Test Discord connection
    info!("Testing Discord webhook connection...");
    discord
        .test_connection()
        .await
        .context("Discord connection test failed")?;

    // Start motion detection loop
    info!("Starting motion detection...");
    let motion_rx = MotionDetector::start_async(
        config.sensor.gpio_pin,
        config.sensor.cooldown_seconds,
    )?;

    info!("System ready. Monitoring for motion...");

    // Main event loop
    loop {
        // Wait for motion detection event
        match motion_rx.recv() {
            Ok(_event) => {
                info!("Motion event received, processing...");

                // Capture image
                match camera.capture() {
                    Ok(image_path) => {
                        info!("Image captured: {:?}", image_path);

                        // Generate timestamp
                        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

                        // Send Discord notification
                        match discord.send_motion_alert(&image_path, &timestamp).await {
                            Ok(_) => {
                                info!("Notification sent successfully");
                            }
                            Err(e) => {
                                error!("Failed to send Discord notification: {:#}", e);
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to capture image: {:#}", e);
                    }
                }
            }
            Err(e) => {
                error!("Motion detection channel closed: {}", e);
                break;
            }
        }
    }

    warn!("Motion detection loop ended");
    Ok(())
}
