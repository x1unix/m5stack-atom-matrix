# M5Stack Atom Matrix Probe

Playground project for ESP32-Pico-based [M5Stack Atom Matrix][devkit-url] devkit.

[devkit-url]: https://shop.m5stack.com/products/atom-matrix-esp32-development-kit

## Prerequisites

Install following packages using `cargo install`:

- `espflash`
- `esp-config --features=tui`
- `espup`
  - Run `espup install` to install the xtensa target.
  - Source `~/export-esp.sh` to your shell.

### NeoVim

Link the rust-analyzer path to make it work for `esp` target:

```bash
ln -sf ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rust-analyzer ~/.rustup/toolchains/esp/bin/rust-analyzer
```

## Resources/Docs

See ./docs/HARDWARE.md for specs and datasheets.

- https://docs.m5stack.com/en/core/ATOM%20Matrix
- https://docs.espressif.com/projects/rust/book/application-development/index.html
- https://embassy.dev/
