# Hardware Setup Guide

## Components Needed

1. **Raspberry Pi Zero W 2** (or any Raspberry Pi model)
2. **PIR Motion Sensor** (HC-SR501 recommended)
3. **Raspberry Pi Camera Module** (v2 or HQ camera)
4. **Power Supply** (5V 2.5A minimum)
5. **MicroSD Card** (16GB+ recommended)
6. **Jumper Wires** (3 wires for PIR sensor)
7. **Case** (optional, but recommended for protection)

## PIR Motion Sensor Wiring

The HC-SR501 PIR sensor has 3 pins:

```
┌─────────────────┐
│   HC-SR501      │
│  PIR Sensor     │
│                 │
│  [POT1]  [POT2] │  POT1: Sensitivity adjustment
│                 │  POT2: Time delay adjustment
│                 │
│  VCC GND OUT    │
└──┬───┬────┬─────┘
   │   │    │
   │   │    └─────────> GPIO 17 (Pin 11) - Signal
   │   └──────────────> GND (Pin 6) - Ground
   └──────────────────> 5V (Pin 2) - Power
```

### Raspberry Pi GPIO Pinout (relevant pins)

```
     3V3  (1) (2)  5V     ← Connect PIR VCC here
   GPIO2  (3) (4)  5V
   GPIO3  (5) (6)  GND    ← Connect PIR GND here
   GPIO4  (7) (8)  GPIO14
     GND  (9) (10) GPIO15
  GPIO17 (11) (12) GPIO18  ← Connect PIR OUT here (GPIO 17)
  GPIO27 (13) (14) GND
  GPIO22 (15) (16) GPIO23
     3V3 (17) (18) GPIO24
  GPIO10 (19) (20) GND
   GPIO9 (21) (22) GPIO25
  GPIO11 (23) (24) GPIO8
     GND (25) (26) GPIO7
```

### Wiring Steps

1. **Power (VCC)**: Connect PIR VCC pin to Pin 2 (5V) on Raspberry Pi
2. **Ground (GND)**: Connect PIR GND pin to Pin 6 (GND) on Raspberry Pi
3. **Signal (OUT)**: Connect PIR OUT pin to Pin 11 (GPIO 17) on Raspberry Pi

### PIR Sensor Configuration

The HC-SR501 has two potentiometers and one jumper:

**Potentiometer 1 (Sensitivity)**:
- Clockwise: Increase detection range (up to 7 meters)
- Counter-clockwise: Decrease detection range

**Potentiometer 2 (Time Delay)**:
- Clockwise: Longer high output time (up to 300 seconds)
- Counter-clockwise: Shorter high output time (minimum 3 seconds)

**Jumper Settings**:
- H (High): Retriggering mode - extends output while motion continues
- L (Low): Single trigger mode - waits for timeout before retriggering

Recommended settings:
- Sensitivity: Medium (12 o'clock position)
- Time Delay: Minimum (fully counter-clockwise)
- Jumper: L (Low) position

## Camera Module Setup

### Physical Connection

1. Locate the camera connector on your Raspberry Pi (labeled "CAMERA")
2. Gently pull up the plastic clip on the connector
3. Insert the camera ribbon cable with the **blue side facing the Ethernet/USB ports** (silver contacts toward HDMI)
4. Press the plastic clip back down to secure the cable

### Enabling the Camera

```bash
sudo raspi-config
```

Navigate to:
- Interface Options → Camera → Enable

Reboot after enabling:
```bash
sudo reboot
```

### Testing the Camera

After rebooting, test the camera:

```bash
# Test with libcamera (modern way)
libcamera-hello

# Or take a test photo
libcamera-jpeg -o test.jpg

# Check camera detection
vcgencmd get_camera
# Should output: supported=1 detected=1
```

## Power Considerations

- **Raspberry Pi Zero W 2**: Requires 5V 2.5A minimum
- **PIR Sensor**: Consumes ~65mA (powered by Pi's 5V rail)
- **Camera Module**: Consumes ~250mA during operation

Total power requirement: ~500mA + Raspberry Pi base consumption

Recommendation: Use a quality 5V 3A power supply.

## Case and Mounting

Consider these factors when choosing/building a case:

1. **Camera placement**: Ensure camera lens is not obstructed
2. **PIR sensor placement**: PIR dome should be exposed and unobstructed
3. **Ventilation**: Small holes for heat dissipation
4. **Cable management**: Strain relief for camera ribbon cable
5. **Weatherproofing**: If used outdoors, ensure proper IP rating

## Troubleshooting Hardware

### PIR Sensor Not Working

1. Check wiring connections
2. Adjust sensitivity potentiometer
3. Wait for sensor warm-up period (~1 minute after power on)
4. Test with multimeter: OUT pin should show 3.3V when motion detected
5. Check GPIO permissions: `groups $USER` should include 'gpio'

### Camera Not Detected

1. Check ribbon cable connection (both ends)
2. Ensure cable is inserted correctly (contacts facing right direction)
3. Enable camera in raspi-config
4. Try different camera port if Pi has multiple (compute module)
5. Test with: `vcgencmd get_camera`

### Power Issues

If experiencing random reboots or camera failures:

1. Check power supply voltage under load
2. Use shorter/thicker USB cable
3. Upgrade to higher amperage power supply
4. Check for voltage drop: `vcgencmd get_throttled`
   - Should return: `throttled=0x0`
   - Non-zero means power issues

## Safety Notes

1. **Always power off** before connecting/disconnecting components
2. **Handle camera module carefully** - avoid touching the lens
3. **Avoid static discharge** - touch grounded metal before handling components
4. **Check polarity** - ensure 5V and GND are not swapped
5. **Don't short pins** - be careful with loose wires

## Testing the Complete Setup

Once everything is connected:

1. Power on the Raspberry Pi
2. Wait for boot (~30 seconds)
3. SSH into the Pi or connect monitor/keyboard
4. Run the test script:
   ```bash
   cd ~/raspberry-eye
   ./run.sh
   ```
5. Wave your hand in front of the PIR sensor
6. Check Discord for notification with image

## Reference Images

For detailed visual guides, refer to:
- [Raspberry Pi GPIO Pinout](https://pinout.xyz)
- [HC-SR501 Datasheet](https://www.epitran.it/ebayDrive/datasheet/44.pdf)
- [Raspberry Pi Camera Setup](https://www.raspberrypi.com/documentation/accessories/camera.html)
