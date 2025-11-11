use anyhow::{Context, Result};
use log::{debug, info, warn};
use rppal::gpio::{Gpio, InputPin, Level};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

/// Motion detector using a PIR sensor
pub struct MotionDetector {
    pin: InputPin,
    cooldown: Duration,
    last_detection: Option<Instant>,
}

impl MotionDetector {
    /// Create a new motion detector
    pub fn new(gpio_pin: u8, cooldown_seconds: u64) -> Result<Self> {
        info!("Initializing motion detector on GPIO pin {}", gpio_pin);

        let gpio = Gpio::new().context("Failed to initialize GPIO")?;
        let pin = gpio
            .get(gpio_pin)
            .context(format!("Failed to get GPIO pin {}", gpio_pin))?
            .into_input();

        Ok(Self {
            pin,
            cooldown: Duration::from_secs(cooldown_seconds),
            last_detection: None,
        })
    }

    /// Check if motion is detected (blocking call)
    pub fn is_motion_detected(&self) -> bool {
        self.pin.read() == Level::High
    }

    /// Check if we're still in cooldown period
    pub fn is_in_cooldown(&self) -> bool {
        if let Some(last) = self.last_detection {
            last.elapsed() < self.cooldown
        } else {
            false
        }
    }

    /// Get remaining cooldown time in seconds
    pub fn remaining_cooldown(&self) -> u64 {
        if let Some(last) = self.last_detection {
            let elapsed = last.elapsed();
            if elapsed < self.cooldown {
                (self.cooldown - elapsed).as_secs()
            } else {
                0
            }
        } else {
            0
        }
    }

    /// Record a motion detection event
    pub fn record_detection(&mut self) {
        self.last_detection = Some(Instant::now());
    }

    /// Start motion detection loop (blocking)
    /// Returns a receiver that will receive motion detection events
    pub fn start_async(
        gpio_pin: u8,
        cooldown_seconds: u64,
    ) -> Result<Receiver<MotionEvent>> {
        let (tx, rx) = channel();

        thread::spawn(move || {
            if let Err(e) = Self::run_detection_loop(gpio_pin, cooldown_seconds, tx) {
                log::error!("Motion detection loop failed: {}", e);
            }
        });

        Ok(rx)
    }

    /// Run the motion detection loop
    fn run_detection_loop(
        gpio_pin: u8,
        cooldown_seconds: u64,
        tx: Sender<MotionEvent>,
    ) -> Result<()> {
        let mut detector = Self::new(gpio_pin, cooldown_seconds)?;

        info!("Motion detection started. Waiting for motion...");

        loop {
            if detector.is_motion_detected() {
                if detector.is_in_cooldown() {
                    let remaining = detector.remaining_cooldown();
                    debug!(
                        "Motion detected but in cooldown ({}s remaining)",
                        remaining
                    );
                } else {
                    info!("Motion detected!");
                    detector.record_detection();

                    if tx.send(MotionEvent::Detected).is_err() {
                        warn!("Failed to send motion event - receiver dropped");
                        break;
                    }
                }
            }

            // Poll every 100ms to avoid excessive CPU usage
            thread::sleep(Duration::from_millis(100));
        }

        Ok(())
    }
}

/// Motion detection event
#[derive(Debug, Clone, Copy)]
pub enum MotionEvent {
    Detected,
}
