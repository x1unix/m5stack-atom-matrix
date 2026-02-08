#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use bt_hci::controller::ExternalController;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::Level;
use esp_hal::rmt::{Rmt, TxChannelConfig, TxChannelCreator};
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;
use esp_radio::ble::controller::BleConnector;
use log::{info, warn};
use trouble_host::prelude::*;

use atom_test::led::{LedMatrix, RMT_CLK_DIVIDER};

extern crate alloc;

const CONNECTIONS_MAX: usize = 1;
const L2CAP_CHANNELS_MAX: usize = 1;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // generator version: 1.2.0

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 98768);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    info!("Embassy initialized!");

    // Animate the 5x5 WS2812 matrix (GPIO27).
    let rmt = Rmt::new(peripherals.RMT, Rate::from_mhz(80)).unwrap();
    let tx_config = TxChannelConfig::default()
        .with_clk_divider(RMT_CLK_DIVIDER)
        .with_idle_output_level(Level::Low)
        .with_idle_output(true)
        .with_carrier_modulation(false);
    let channel = rmt
        .channel0
        .configure_tx(peripherals.GPIO27, tx_config)
        .unwrap();
    let mut matrix = LedMatrix::new(channel);

    let radio_init = esp_radio::init().expect("Failed to initialize Wi-Fi/BLE controller");
    // find more examples https://github.com/embassy-rs/trouble/tree/main/examples/esp32
    let transport = BleConnector::new(&radio_init, peripherals.BT, Default::default()).unwrap();
    let ble_controller = ExternalController::<_, 1>::new(transport);
    let mut resources: HostResources<DefaultPacketPool, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX> =
        HostResources::new();
    let _stack = trouble_host::new(ble_controller, &mut resources);

    // TODO: Spawn some tasks
    let _ = spawner;

    loop {
        if let Err(err) = matrix.step_green_chase() {
            warn!("LED update failed: {:?}", err);
        }
        Timer::after(Duration::from_millis(80)).await;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0/examples
}
