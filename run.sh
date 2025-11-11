#!/bin/bash
# Manual run script for Raspberry Eye

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Raspberry Eye - Motion Detection System${NC}"
echo "========================================"
echo ""

# Check if running on Raspberry Pi
if ! grep -q "Raspberry Pi" /proc/cpuinfo 2>/dev/null; then
    echo -e "${YELLOW}Warning: Not running on a Raspberry Pi${NC}"
    echo "Some features may not work correctly"
    echo ""
fi

# Check if config file exists
if [ ! -f "config.yaml" ]; then
    echo -e "${RED}Error: config.yaml not found${NC}"
    echo "Please copy config.yaml.example to config.yaml and configure it"
    exit 1
fi

# Check if Python script exists
if [ ! -f "scripts/capture.py" ]; then
    echo -e "${RED}Error: scripts/capture.py not found${NC}"
    exit 1
fi

# Make Python script executable
chmod +x scripts/capture.py

# Check for picamera2 installation
echo "Checking dependencies..."
if ! python3 -c "import picamera2" 2>/dev/null; then
    echo -e "${RED}Error: picamera2 not installed${NC}"
    echo "Install with: sudo apt install -y python3-picamera2"
    exit 1
fi

echo -e "${GREEN}Dependencies OK${NC}"
echo ""

# Build the project if binary doesn't exist or if source changed
if [ ! -f "target/release/raspberry-eye" ] || [ "src/" -nt "target/release/raspberry-eye" ]; then
    echo "Building project..."
    cargo build --release
    echo ""
fi

# Set log level
export RUST_LOG="${RUST_LOG:-info}"

# Create images directory if it doesn't exist
mkdir -p images

echo "Starting Raspberry Eye..."
echo "Press Ctrl+C to stop"
echo ""

# Run the application
./target/release/raspberry-eye
