use anyhow::{Context, Result};
use chrono::Local;
use log::{debug, info};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Camera controller that wraps the Python capture script
pub struct Camera {
    script_path: PathBuf,
    output_dir: PathBuf,
    filename_format: String,
    width: u32,
    height: u32,
}

impl Camera {
    /// Create a new camera controller
    pub fn new(
        script_path: impl AsRef<Path>,
        output_dir: impl AsRef<Path>,
        filename_format: String,
        width: u32,
        height: u32,
    ) -> Self {
        Self {
            script_path: script_path.as_ref().to_path_buf(),
            output_dir: output_dir.as_ref().to_path_buf(),
            filename_format,
            width,
            height,
        }
    }

    /// Capture an image and return the path to the saved file
    pub fn capture(&self) -> Result<PathBuf> {
        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&self.output_dir)
            .context("Failed to create output directory")?;

        // Generate filename with timestamp
        let timestamp = Local::now();
        let filename = timestamp.format(&self.filename_format).to_string();
        let output_path = self.output_dir.join(filename);

        info!("Capturing image to: {:?}", output_path);

        // Execute Python capture script
        let output = Command::new("python3")
            .arg(&self.script_path)
            .arg(output_path.to_str().unwrap())
            .arg("--width")
            .arg(self.width.to_string())
            .arg("--height")
            .arg(self.height.to_string())
            .output()
            .context("Failed to execute camera capture script")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Camera capture failed: {}", stderr);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        debug!("Camera output: {}", stdout);

        // Verify the file was created
        if !output_path.exists() {
            anyhow::bail!("Image file was not created at {:?}", output_path);
        }

        info!("Image captured successfully");
        Ok(output_path)
    }

    /// Test camera functionality
    pub fn test(&self) -> Result<()> {
        info!("Testing camera...");

        let test_path = self.output_dir.join("test.jpg");

        // Create output directory
        std::fs::create_dir_all(&self.output_dir)
            .context("Failed to create output directory")?;

        // Execute test capture
        let output = Command::new("python3")
            .arg(&self.script_path)
            .arg(test_path.to_str().unwrap())
            .arg("--width")
            .arg("640")
            .arg("--height")
            .arg("480")
            .output()
            .context("Failed to execute camera test")?;

        if output.status.success() {
            info!("Camera test successful");
            // Clean up test image
            let _ = std::fs::remove_file(test_path);
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Camera test failed: {}", stderr)
        }
    }
}
