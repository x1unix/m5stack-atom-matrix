# TODOs

## LED Matrix

- [x] Find out how to get internal LED matrix working. Matrix is `WS2812C-2020`.
- [ ] Use [esp-hal-smartled][esp-hal-smartled] instead of custom @./src/led.rs library.

See @./docs/HARDWARE.md

[esp-hal-smartled]: https://github.com/esp-rs/esp-hal-community/blob/main/esp-hal-smartled/src/lib.rs

Example:

```rust
#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{rmt::Rmt, time::Rate, Config};
use esp_hal_smartled::{smartLedBuffer, SmartLedsAdapter};
use smart_leds::{brightness, colors::RED, SmartLedsWrite as _};

#[esp_hal::main]
fn main() -> ! {
    let p = esp_hal::init(Config::default());
    let mut led = {
        let frequency = Rate::from_mhz(80);
        let rmt = Rmt::new(p.RMT, frequency).expect("Failed to initialize RMT0");
        SmartLedsAdapter::new(rmt.channel0, p.GPIO2, smartLedBuffer!(1))
    };
    let level = 10;
    led.write(brightness([RED].into_iter(), level)).unwrap();
    loop {} // loop forever
}

```

## Gyro

Find out how to get internal gyro MPU6886 working

See @./docs/HARDWARE.md
