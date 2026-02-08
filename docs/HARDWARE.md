# Hardware specs

The M5Stack Atom Matrix is ESP32-based devkit (xtensa) with different peripherals described below.

## Board Specs

| Specification  | Parameter                                    |
| -------------- | -------------------------------------------- |
| SoC            | ESP32-PICO-D4@Dual-core processor, 240MHz    |
| DMIPS          | 600                                          |
| SRAM           | 520KB                                        |
| Flash          | 4MB                                          |
| Wi-Fi          | 2.4 GHz Wi-Fi                                |
| Input Voltage  | 5 V @ 500 mA                                 |
| Host Interface | USB Type-C × 1, GROVE (I2C + I/O + UART) × 1 |
| PINs           | G19, G21, G22, G23, G25, G33                 |
| RGB LED        | WS2812C 2020 × 25                            |
| MEMS           | MPU6886 (I2C Address: 0x68)                  |
| IR             | Infrared transmission                        |
| Button         | Programmable button × 1                      |
| Antenna        | 2.4 G 3D antenna                             |
| Working Temp.  | 0 ~ 60 °C                                    |
| Enclosure      | Plastic (PC)                                 |
| Product Size   | 24.0 x 24.0 x 13.8mm                         |
| Product Weight | 7.3 g                                        |
| Package Size   | 85.0 x 66.0 x 15.0mm                         |
| Gross Weight   | 12.6 g                                       |

## Pinout

### RGB & BUTTON & IR & MPU6886

| Peripheral | Pin | Kind   |
| ---------- | --- | ------ |
| RGB LED    | G27 | RGB    |
| Button     | G39 | Button |
| Infrared   | G12 | IR_TX  |
| MPU6886    | G21 | SCL    |
| MPU6886    | G25 | SDA    |

### Grove

TODO

## LED Matrix

LED Matrix is 5x5 `WS2812C 2020`

Datasheet is available at @./datasheets/WS2812C-2020_V1.2.pdf

M5Stack provides a demo library to work with the display: https://github.com/m5stack/M5Atom/tree/master

## Gyro

Gyro is `MPU6886`. Datasheet is available at @./datasheets/MPU-6886-000193+v1.1_GHIC_en.pdf

Gyro I2C address `0x68`
