#!/bin/bash
# Installation script for Raspberry Eye systemd service

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Raspberry Eye - Installation Script${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

# Check if running as root
if [ "$EUID" -eq 0 ]; then
    echo -e "${RED}Error: Do not run this script as root${NC}"
    echo "Run it as the user who will run the service (usually 'pi')"
    exit 1
fi

# Get the current directory (project root)
PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
USER_NAME="$(whoami)"
SERVICE_FILE="raspberry-eye.service"

echo -e "${GREEN}Project directory: ${PROJECT_DIR}${NC}"
echo -e "${GREEN}User: ${USER_NAME}${NC}"
echo ""

# Check if on Raspberry Pi
if ! grep -q "Raspberry Pi" /proc/cpuinfo 2>/dev/null; then
    echo -e "${YELLOW}Warning: Not running on a Raspberry Pi${NC}"
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Install system dependencies
echo -e "${BLUE}Installing system dependencies...${NC}"
sudo apt update
sudo apt install -y python3-picamera2 python3-pip

# Install Rust if not already installed
if ! command -v cargo &> /dev/null; then
    echo -e "${BLUE}Installing Rust...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo -e "${GREEN}Rust already installed${NC}"
fi

# Check if config.yaml exists
if [ ! -f "$PROJECT_DIR/config.yaml" ]; then
    echo -e "${RED}Error: config.yaml not found${NC}"
    echo "Please create and configure config.yaml before installing the service"
    exit 1
fi

# Build the project
echo -e "${BLUE}Building project in release mode...${NC}"
cd "$PROJECT_DIR"
cargo build --release

# Make scripts executable
echo -e "${BLUE}Setting script permissions...${NC}"
chmod +x "$PROJECT_DIR/scripts/capture.py"
chmod +x "$PROJECT_DIR/run.sh"

# Create images directory
mkdir -p "$PROJECT_DIR/images"

# Update service file with actual paths
echo -e "${BLUE}Configuring systemd service...${NC}"
TEMP_SERVICE="/tmp/raspberry-eye-temp.service"
sed "s|/home/pi/raspberry-eye|$PROJECT_DIR|g" "$PROJECT_DIR/$SERVICE_FILE" > "$TEMP_SERVICE"
sed -i "s|User=pi|User=$USER_NAME|g" "$TEMP_SERVICE"
sed -i "s|Group=pi|Group=$USER_NAME|g" "$TEMP_SERVICE"

# Copy service file to systemd directory
echo -e "${BLUE}Installing systemd service...${NC}"
sudo cp "$TEMP_SERVICE" "/etc/systemd/system/$SERVICE_FILE"
rm "$TEMP_SERVICE"

# Reload systemd
sudo systemctl daemon-reload

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Installation complete!${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo "Available commands:"
echo ""
echo -e "  ${BLUE}Start service:${NC}"
echo "    sudo systemctl start raspberry-eye"
echo ""
echo -e "  ${BLUE}Enable service (auto-start on boot):${NC}"
echo "    sudo systemctl enable raspberry-eye"
echo ""
echo -e "  ${BLUE}Check status:${NC}"
echo "    sudo systemctl status raspberry-eye"
echo ""
echo -e "  ${BLUE}View logs:${NC}"
echo "    sudo journalctl -u raspberry-eye -f"
echo ""
echo -e "  ${BLUE}Stop service:${NC}"
echo "    sudo systemctl stop raspberry-eye"
echo ""
echo -e "  ${BLUE}Disable service:${NC}"
echo "    sudo systemctl disable raspberry-eye"
echo ""
echo -e "${YELLOW}Note: Make sure to configure your Discord webhook URL in config.yaml${NC}"
echo ""
