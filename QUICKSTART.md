# Quick Start Guide

Get your Raspberry Eye motion detection system up and running in minutes!

## Prerequisites Checklist

- [ ] Raspberry Pi Zero W 2 (or any Raspberry Pi)
- [ ] PIR Motion Sensor (HC-SR501 or similar)
- [ ] Raspberry Pi Camera Module
- [ ] MicroSD card with Raspberry Pi OS installed
- [ ] Power supply (5V 2.5A minimum)
- [ ] Internet connection (WiFi or Ethernet)
- [ ] Discord account with webhook URL

## Step-by-Step Installation

### 1. Hardware Setup (10 minutes)

Wire the PIR sensor to your Raspberry Pi:

```
PIR VCC  → Pin 2 (5V)
PIR GND  → Pin 6 (GND)
PIR OUT  → Pin 11 (GPIO 17)
```

Connect the camera ribbon cable to the camera port.

See [HARDWARE.md](HARDWARE.md) for detailed wiring diagrams.

### 2. Prepare Raspberry Pi (5 minutes)

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Enable camera interface
sudo raspi-config
# Navigate: Interface Options → Camera → Enable
# Exit and reboot

sudo reboot
```

### 3. Get Discord Webhook URL (2 minutes)

1. Open Discord, go to your channel
2. Click channel settings (gear icon)
3. Go to Integrations → Webhooks
4. Click "New Webhook"
5. Copy the webhook URL
6. Save it for the next step

### 4. Clone and Configure (5 minutes)

```bash
# Clone the repository
cd ~
git clone https://github.com/yourusername/raspberry-eye.git
cd raspberry-eye

# Edit configuration
nano config.yaml
```

Update the `discord.webhook_url` field with your webhook URL from step 3.

Press `Ctrl+X`, then `Y`, then `Enter` to save.

### 5. Install and Run (15 minutes)

```bash
# Make install script executable and run it
chmod +x install.sh
./install.sh

# This will:
# - Install Rust and Python dependencies
# - Build the project
# - Set up systemd service
```

### 6. Start the Service

```bash
# Start the service
sudo systemctl start raspberry-eye

# Enable auto-start on boot
sudo systemctl enable raspberry-eye

# Check if it's running
sudo systemctl status raspberry-eye
```

### 7. Test It!

Wave your hand in front of the PIR sensor. Within a few seconds, you should receive a Discord notification with an image!

## Verify Installation

Check the logs to ensure everything is working:

```bash
# View live logs
sudo journalctl -u raspberry-eye -f
```

You should see messages like:
```
INFO raspberry_eye: Configuration loaded from: config.yaml
INFO raspberry_eye: Testing camera connection...
INFO raspberry_eye: Camera test successful
INFO raspberry_eye: Testing Discord webhook connection...
INFO raspberry_eye: Discord connection test successful
INFO raspberry_eye: System ready. Monitoring for motion...
```

## Quick Commands

```bash
# View status
sudo systemctl status raspberry-eye

# Stop service
sudo systemctl stop raspberry-eye

# Start service
sudo systemctl start raspberry-eye

# View logs
sudo journalctl -u raspberry-eye -f

# Manual testing
./run.sh
```

## Troubleshooting Common Issues

### "Camera test failed"

```bash
# Check camera detection
vcgencmd get_camera
# Should show: supported=1 detected=1

# Test camera directly
libcamera-hello
```

**Fix**: Reconnect camera cable, ensure it's enabled in raspi-config

### "Discord connection test failed"

- Check internet connection: `ping discord.com`
- Verify webhook URL in config.yaml is correct
- Ensure no trailing spaces in webhook URL

### "Motion detected but in cooldown"

This is normal! The cooldown prevents spam. Wait 30 seconds between tests, or reduce `cooldown_seconds` in config.yaml.

### GPIO Permission Error

```bash
# Add user to gpio group
sudo usermod -a -G gpio $USER

# Log out and back in
logout
```

### Build Errors

Ensure you have enough space:
```bash
df -h
```

Raspberry Pi OS needs at least 2GB free for building.

## Configuration Tips

### Reduce False Alarms

In `config.yaml`, increase cooldown:
```yaml
sensor:
  cooldown_seconds: 60  # Wait 1 minute between detections
```

### Adjust Image Quality

```yaml
camera:
  resolution:
    width: 1280   # Lower resolution = smaller files
    height: 720
```

### Change GPIO Pin

If GPIO 17 is already used:
```yaml
sensor:
  gpio_pin: 22  # Use any available GPIO pin
```

Don't forget to update your wiring!

## Next Steps

- Adjust PIR sensor sensitivity using the potentiometers
- Set up a dedicated Discord channel for alerts
- Consider adding a case to protect the hardware
- Set up port forwarding if you want remote access
- Check logs regularly to ensure system health

## Need Help?

- Read the full [README.md](README.md)
- Check [HARDWARE.md](HARDWARE.md) for wiring details
- Review logs: `sudo journalctl -u raspberry-eye -n 100`
- Test individual components with the manual run script: `./run.sh`

## Performance Notes

- **Detection latency**: ~2 seconds from motion to notification
- **Image capture time**: ~2-3 seconds
- **Cooldown default**: 30 seconds
- **Power consumption**: ~500mA average, ~750mA during capture

Enjoy your new motion detection system!
