use esp_hal::gpio::Level;
use esp_hal::rmt::{Channel, Error, PulseCode, Tx};
use esp_hal::Blocking;

pub const LED_COUNT: usize = 25;
pub const RMT_CLK_DIVIDER: u8 = 2;
pub const GREEN_GRB: [u8; 3] = [0xFF, 0x00, 0x00];
pub const OFF_GRB: [u8; 3] = [0x00, 0x00, 0x00];

const LED_BITS_PER_PIXEL: usize = 24;
const LED_TOTAL_BITS: usize = LED_COUNT * LED_BITS_PER_PIXEL;
const LED_DATA_LEN: usize = LED_TOTAL_BITS + 1;
const LED_BYTES_PER_PIXEL: usize = 3;
const LED_PIXEL_BYTES: usize = LED_COUNT * LED_BYTES_PER_PIXEL;

// WS2812 timing (approx.) for 40MHz RMT clock (80MHz / divider 2).
const T0H_TICKS: u16 = 14;
const T0L_TICKS: u16 = 36;
const T1H_TICKS: u16 = 28;
const T1L_TICKS: u16 = 22;
const RESET_TICKS: u16 = 2_000; // 50us @ 40MHz

pub struct LedMatrix<'ch> {
    channel: Option<Channel<'ch, Blocking, Tx>>,
    data: [PulseCode; LED_DATA_LEN],
    pixels: [u8; LED_PIXEL_BYTES],
    pulse_0: PulseCode,
    pulse_1: PulseCode,
    frame: usize,
}

impl<'ch> LedMatrix<'ch> {
    pub fn new(channel: Channel<'ch, Blocking, Tx>) -> Self {
        let pulse_0 = PulseCode::new(Level::High, T0H_TICKS, Level::Low, T0L_TICKS);
        let pulse_1 = PulseCode::new(Level::High, T1H_TICKS, Level::Low, T1L_TICKS);

        Self {
            channel: Some(channel),
            data: [PulseCode::default(); LED_DATA_LEN],
            pixels: [0u8; LED_PIXEL_BYTES],
            pulse_0,
            pulse_1,
            frame: 0,
        }
    }

    pub fn set_all(&mut self, color_grb: [u8; 3]) {
        for i in 0..LED_COUNT {
            let base = i * LED_BYTES_PER_PIXEL;
            self.pixels[base..base + 3].copy_from_slice(&color_grb);
        }
    }

    pub fn set_pixel(&mut self, index: usize, color_grb: [u8; 3]) {
        if index >= LED_COUNT {
            return;
        }
        let base = index * LED_BYTES_PER_PIXEL;
        self.pixels[base..base + 3].copy_from_slice(&color_grb);
    }

    pub fn show(&mut self) -> Result<(), Error> {
        self.encode_ws2812();
        let channel = self.channel.take().expect("LED channel missing");
        let tx = channel
            .transmit(&self.data)
            .expect("RMT transmit precondition failed");
        match tx.wait() {
            Ok(channel) => {
                self.channel = Some(channel);
                Ok(())
            }
            Err((err, channel)) => {
                self.channel = Some(channel);
                Err(err)
            }
        }
    }

    pub fn step_green_chase(&mut self) -> Result<(), Error> {
        self.set_all(OFF_GRB);
        self.set_pixel(self.frame, GREEN_GRB);
        let result = self.show();
        self.frame = (self.frame + 1) % LED_COUNT;
        result
    }

    fn encode_ws2812(&mut self) {
        let mut idx = 0;
        for &byte in self.pixels.iter() {
            for bit in (0..8).rev() {
                let is_one = (byte >> bit) & 1 == 1;
                self.data[idx] = if is_one { self.pulse_1 } else { self.pulse_0 };
                idx += 1;
            }
        }
        self.data[idx] = PulseCode::new(Level::Low, RESET_TICKS, Level::Low, 0);
    }
}
