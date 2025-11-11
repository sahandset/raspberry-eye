# Raspberry Eye

Motion-triggered detection, imaging and notification for home security using Raspberry Pi Zero W 2.

## Features

- PIR motion sensor detection with configurable cooldown
- Automatic image capture using Raspberry Pi Camera
- Discord webhook notifications with images and timestamps
- Hybrid Rust/Python architecture for performance and compatibility
- Configurable YAML-based settings
- Structured logging to file and stdout
- Systemd service for automatic startup on boot
- Manual testing script for development

## Hardware Requirements

- **Raspberry Pi Zero W 2** (or any Raspberry Pi model)
- **PIR Motion Sensor** (HC-SR501 or similar)
- **Raspberry Pi Camera Module** (v2 or HQ camera recommended)
- Power supply
- MicroSD card (16GB+ recommended)

## Wiring Diagram

### PIR Motion Sensor

```
PIR Sensor          Raspberry Pi
----------          ------------
VCC       -------->  5V (Pin 2 or 4)
GND       -------->  GND (Pin 6, 9, 14, 20, 25, 30, 34, or 39)
OUT       -------->  GPIO 17 (Pin 11) - configurable in config.yaml
```

### Camera Module

Connect the camera ribbon cable to the camera port on the Raspberry Pi. Make sure to:
1. Lift the plastic clip on the camera port
2. Insert the ribbon cable with contacts facing toward the board
3. Press the clip back down

## Software Requirements

- Raspberry Pi OS (Bullseye or later)
- Rust (installed automatically by install.sh)
- Python 3.9+
- picamera2 library
- Internet connection for Discord notifications

## Installation

### 1. Clone the Repository

```bash
cd ~
git clone https://github.com/yourusername/raspberry-eye.git
cd raspberry-eye
```

### 2. Configure the Application

Copy and edit the configuration file:

```bash
# The config.yaml file already exists with defaults
# Edit it to add your Discord webhook URL
nano config.yaml
```

**Important**: You must set your Discord webhook URL in the `discord.webhook_url` field.

#### Getting a Discord Webhook URL

1. Open Discord and go to the channel where you want notifications
2. Click the gear icon (Edit Channel)
3. Go to Integrations > Webhooks
4. Click "New Webhook"
5. Copy the webhook URL
6. Paste it into `config.yaml`

### 3. Enable the Camera

```bash
sudo raspi-config
```

Navigate to: `Interface Options` > `Camera` > `Enable`

Reboot after enabling:

```bash
sudo reboot
```

### 4. Run the Installation Script

```bash
chmod +x install.sh
./install.sh
```

This will:
- Install system dependencies (picamera2, Rust)
- Build the project in release mode
- Set up the systemd service
- Configure permissions

### 5. Start the Service

```bash
# Start the service
sudo systemctl start raspberry-eye

# Enable auto-start on boot
sudo systemctl enable raspberry-eye

# Check status
sudo systemctl status raspberry-eye

# View live logs
sudo journalctl -u raspberry-eye -f
```

## Manual Testing

For testing without installing as a service:

```bash
chmod +x run.sh
./run.sh
```

Press Ctrl+C to stop.

## Configuration

Edit `config.yaml` to customize:

### Discord Settings
```yaml
discord:
  webhook_url: "https://discord.com/api/webhooks/YOUR_WEBHOOK"
```

### Sensor Settings
```yaml
sensor:
  gpio_pin: 17              # GPIO pin number (BCM numbering)
  cooldown_seconds: 30      # Time between detections
```

### Camera Settings
```yaml
camera:
  script_path: "./scripts/capture.py"
  output_dir: "./images"
  filename_format: "motion_%Y-%m-%d_%H-%M-%S.jpg"
  resolution:
    width: 1920
    height: 1080
```

### Logging Settings
```yaml
logging:
  level: "info"                    # trace, debug, info, warn, error
  file: "./raspberry-eye.log"      # Log file path
```

## Architecture

### Hybrid Rust + Python Design

- **Rust Core** (`src/`): Handles GPIO, motion detection logic, Discord HTTP client, orchestration, and logging
- **Python Script** (`scripts/capture.py`): Handles camera capture using picamera2 library

This design combines:
- Rust's performance, safety, and excellent GPIO support
- Python's mature picamera2 library for optimal camera support

### Project Structure

```
raspberry-eye/
├── src/
│   ├── main.rs           # Main orchestration loop
│   ├── config.rs         # Configuration parser
│   ├── motion.rs         # PIR sensor detection
│   ├── camera.rs         # Camera wrapper
│   └── discord.rs        # Discord webhook client
├── scripts/
│   └── capture.py        # Python camera capture
├── config.yaml           # Configuration file
├── Cargo.toml            # Rust dependencies
├── run.sh                # Manual run script
├── install.sh            # Installation script
└── raspberry-eye.service # Systemd service file
```

## Troubleshooting

### Camera Issues

If the camera fails to initialize:

```bash
# Test the camera directly
libcamera-hello

# Check if camera is detected
vcgencmd get_camera

# Should output: supported=1 detected=1
```

### GPIO Permission Issues

If you get GPIO permission errors:

```bash
# Add user to gpio group
sudo usermod -a -G gpio $USER

# Log out and back in for changes to take effect
```

### Discord Webhook Fails

- Verify the webhook URL is correct
- Check internet connectivity: `ping discord.com`
- View detailed logs: `sudo journalctl -u raspberry-eye -f`

### Motion Not Detected

- Check PIR sensor wiring
- Adjust PIR sensor sensitivity (potentiometers on sensor)
- Verify GPIO pin number in config.yaml matches wiring
- Check logs for errors

## Service Management

```bash
# Start service
sudo systemctl start raspberry-eye

# Stop service
sudo systemctl stop raspberry-eye

# Restart service
sudo systemctl restart raspberry-eye

# Enable auto-start on boot
sudo systemctl enable raspberry-eye

# Disable auto-start
sudo systemctl disable raspberry-eye

# View status
sudo systemctl status raspberry-eye

# View logs
sudo journalctl -u raspberry-eye -f

# View last 100 lines
sudo journalctl -u raspberry-eye -n 100
```

## Development

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

### Testing Components

```bash
# Test camera only
python3 scripts/capture.py test.jpg

# Run with debug logging
RUST_LOG=debug ./target/release/raspberry-eye
```

## License

MIT License - See LICENSE file for details

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Author

Sahand Setareh

## Security Note

Never commit your `config.yaml` file with the Discord webhook URL to version control. The `.gitignore` file excludes it by default.
