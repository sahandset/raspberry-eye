#!/usr/bin/env python3
"""
Camera capture script for Raspberry Pi
Uses picamera2 library to capture images from the Raspberry Pi Camera
"""

import sys
import os
from pathlib import Path
import argparse
import time

try:
    from picamera2 import Picamera2
except ImportError:
    print("ERROR: picamera2 library not installed", file=sys.stderr)
    print("Install with: sudo apt install -y python3-picamera2", file=sys.stderr)
    sys.exit(1)


def capture_image(output_path: str, width: int = 1920, height: int = 1080) -> bool:
    """
    Capture an image from the Raspberry Pi camera

    Args:
        output_path: Path where the image will be saved
        width: Image width in pixels
        height: Image height in pixels

    Returns:
        True if successful, False otherwise
    """
    try:
        # Create output directory if it doesn't exist
        output_dir = Path(output_path).parent
        output_dir.mkdir(parents=True, exist_ok=True)

        # Initialize camera
        picam2 = Picamera2()

        # Configure camera
        config = picam2.create_still_configuration(
            main={"size": (width, height)}
        )
        picam2.configure(config)

        # Start camera
        picam2.start()

        # Allow camera to warm up
        time.sleep(2)

        # Capture image
        picam2.capture_file(output_path)

        # Stop camera
        picam2.stop()

        print(f"Image captured successfully: {output_path}")
        return True

    except Exception as e:
        print(f"ERROR: Failed to capture image: {e}", file=sys.stderr)
        return False


def main():
    parser = argparse.ArgumentParser(
        description="Capture image from Raspberry Pi camera"
    )
    parser.add_argument(
        "output",
        type=str,
        help="Output file path for the captured image"
    )
    parser.add_argument(
        "--width",
        type=int,
        default=1920,
        help="Image width in pixels (default: 1920)"
    )
    parser.add_argument(
        "--height",
        type=int,
        default=1080,
        help="Image height in pixels (default: 1080)"
    )

    args = parser.parse_args()

    success = capture_image(args.output, args.width, args.height)
    sys.exit(0 if success else 1)


if __name__ == "__main__":
    main()
