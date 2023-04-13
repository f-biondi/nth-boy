mod utils;

use gbcore::mmu::address_spaces::io::joypad::JoypadState;
use gbcore::ppu::LcdBuffer;
use gbcore::Device;
use wasm_bindgen::prelude::*;
use web_time::{SystemTime, UNIX_EPOCH};

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

const WIDTH: usize = 160;
const HEIGHT: usize = 144;
const EMPTY_BUFFER: &'static [u32] = &[0xffffff; WIDTH * HEIGHT];

#[wasm_bindgen]
struct Emulator {
    device: Device,
    lcd_buffer: LcdBuffer,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    a: bool,
    b: bool,
    start: bool,
    select: bool,
}

#[wasm_bindgen]
impl Emulator {
    pub fn new(rom: &[u8], ram: &[u8], rtc: &[u8]) -> Emulator {
        utils::set_panic_hook();
        Emulator {
            device: Device::new(
                rom.to_vec(),
                if ram.len() > 0 {
                    Some(ram.to_vec())
                } else {
                    None
                },
                if rtc.len() > 0 {
                    Some(rtc.to_vec())
                } else {
                    None
                },
            )
            .unwrap(),
            lcd_buffer: LcdBuffer {
                buffer: vec![0; WIDTH * HEIGHT],
                cleared: false,
            },
            up: false,
            down: false,
            left: false,
            right: false,
            a: false,
            b: false,
            start: false,
            select: false,
        }
    }

    pub fn next_frame(&mut self) {
        self.device.update_rtc_now(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
        );
        self.lcd_buffer.cleared = false;
        self.device.frame(
            &mut self.lcd_buffer,
            JoypadState {
                up: self.up,
                down: self.down,
                left: self.left,
                right: self.right,
                a: self.a,
                b: self.b,
                start: self.start,
                select: self.select,
            },
        );
    }

    pub fn buffer(&self) -> *const u32 {
        if !self.lcd_buffer.cleared {
            self.lcd_buffer.buffer.as_ptr()
        } else {
            EMPTY_BUFFER.as_ptr()
        }
    }

    pub fn dump_ram(&self) -> Vec<u8> {
        if let Some(ram) = self.device.dump_ram() {
            ram
        } else {
            vec![]
        }
    }

    pub fn dump_rtc(&self) -> Vec<u8> {
        if let Some(rtc) = self.device.dump_rtc() {
            rtc
        } else {
            vec![]
        }
    }

    pub fn set_up(&mut self) {
        self.up = true;
    }

    pub fn unset_up(&mut self) {
        self.up = false;
    }

    pub fn set_down(&mut self) {
        self.down = true;
    }

    pub fn unset_down(&mut self) {
        self.down = false;
    }

    pub fn set_left(&mut self) {
        self.left = true;
    }

    pub fn unset_left(&mut self) {
        self.left = false;
    }

    pub fn set_right(&mut self) {
        self.right = true;
    }

    pub fn unset_right(&mut self) {
        self.right = false;
    }

    pub fn set_a(&mut self) {
        self.a = true;
    }

    pub fn unset_a(&mut self) {
        self.a = false;
    }

    pub fn set_b(&mut self) {
        self.b = true;
    }

    pub fn unset_b(&mut self) {
        self.b = false;
    }

    pub fn set_start(&mut self) {
        self.start = true;
    }

    pub fn unset_start(&mut self) {
        self.start = false;
    }

    pub fn set_select(&mut self) {
        self.select = true;
    }

    pub fn unset_select(&mut self) {
        self.select = false;
    }
}
