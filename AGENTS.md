# Project Knowledge Dump (M5Stack Atom Matrix)

## Hardware Summary (from docs/HARDWARE.md)
- Board: M5Stack Atom Matrix (ESP32-PICO-D4)
- CPU: Dual-core @ 240MHz (600 DMIPS)
- SRAM: 520KB
- Flash: 4MB
- Power: 5V @ 500mA
- Connectivity: 2.4GHz Wi-Fi
- Size: 24.0 x 24.0 x 13.8mm

## Peripherals / Pinout
- LED Matrix: 5x5 WS2812C 2020 (25 LEDs)
  - Data pin: GPIO27 (G27)
  - Color order: GRB (per M5Atom Arduino library FastLED usage)
- Button: GPIO39 (G39)
- IR TX: GPIO12 (G12)
- IMU (MPU6886): I2C
  - SCL: GPIO21 (G21)
  - SDA: GPIO25 (G25)
  - I2C address: 0x68

## Docs / References
- Hardware specs: docs/HARDWARE.md
- LED datasheet: docs/datasheets/WS2812C-2020_V1.2.pdf
- IMU datasheet: docs/datasheets/MPU-6886-000193+v1.1_GHIC_en.pdf
- Vendor Arduino reference: docs/M5Atom (notably src/utility/LED_DisPlay.*)

## Notes
- LED matrix is WS2812-compatible; use precise timing and a reset/latch low period.
- Board vendor Arduino library uses FastLED with WS2812 on DATA_PIN=27 and GRB ordering.
