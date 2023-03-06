use crate::mmu::address_spaces::adressable_memory::AdressableMemory;
use crate::mmu::address_spaces::Addressable;
use joypad::Joypad;
use lcd::Lcd;
use std::error::Error;
use timers::Timers;

pub mod joypad;
mod lcd;
mod timers;

pub struct Io {
    pub joypad: Joypad,
    i1: AdressableMemory,
    pub timers: Timers,
    i2: AdressableMemory,
    pub lcd: Lcd,
    i3: AdressableMemory,
    pub if_flag: u8,
    pub test: String,
}

impl Io {
    pub fn new() -> Result<Io, Box<dyn Error>> {
        Ok(Self {
            joypad: Joypad::new(),
            i1: AdressableMemory::new(0xFF01, 0xFF02)?,
            timers: Timers::new(),
            i2: AdressableMemory::new(0xFF10, 0xFF3F)?,
            lcd: Lcd::new(),
            i3: AdressableMemory::new(0xFF4C, 0xFF7F)?,
            if_flag: 0xE1,
            test: String::from(""),
        })
    }

    pub fn get_test(&mut self) -> String {
        let res: String = self.test.clone();
        self.test = String::from("");
        res
    }

    pub fn request_vblank_interrupt(&mut self) {
        self.if_flag |= 0x1;
    }

    pub fn request_lcd_stat_interrupt(&mut self) {
        self.if_flag |= 0x2;
    }

    pub fn request_timer_interrupt(&mut self) {
        self.if_flag |= 0x4;
    }

    pub fn request_serial_interrupt(&mut self) {
        self.if_flag |= 0x8;
    }

    pub fn request_joypad_interrupt(&mut self) {
        self.if_flag |= 0x10;
    }

    pub fn get_vblank_interrupt(&mut self) -> bool {
        (self.if_flag & 0x1) != 0
    }

    pub fn get_lcd_stat_interrupt(&mut self) -> bool {
        (self.if_flag & 0x2) != 0
    }

    pub fn get_timer_interrupt(&mut self) -> bool {
        (self.if_flag & 0x4) != 0
    }

    pub fn get_serial_interrupt(&mut self) -> bool {
        (self.if_flag & 0x8) != 0
    }

    pub fn get_joypad_interrupt(&mut self) -> bool {
        (self.if_flag & 0x10) != 0
    }
}

impl Addressable for Io {
    fn write(&mut self, location: u16, byte: u8) {
        match location {
            0xFF00 => self.joypad.write(location, byte),
            0xFF01..=0xFF02 => {
                if location == 0xFF02 && byte == 0x81 {
                    let cb: [u8; 1] = [self.i1.read(0xff01)];
                    let c: &str = std::str::from_utf8(&cb).unwrap();
                    self.test = String::from(c);
                    self.i1.write(0xff02, 0x0);
                } else {
                    self.i1.write(location, byte);
                }
            }
            0xFF03 => {}
            0xFF04..=0xFF07 => self.timers.write(location, byte),
            0xFF08..=0xFF0E => {}
            0xFF0F => self.if_flag = byte,
            0xFF10..=0xFF3F => self.i2.write(location, byte),
            0xFF40..=0xFF4B => self.lcd.write(location, byte),
            0xFF4C..=0xFF7F => self.i3.write(location, byte),
            _ => panic!("IO unsupported write to {:#04X}", location),
        }
    }

    fn read(&self, location: u16) -> u8 {
        match location {
            0xFF00 => self.joypad.read(location),
            0xFF01..=0xFF02 => self.i1.read(location),
            0xFF03 => 0x00,
            0xFF04..=0xFF07 => self.timers.read(location),
            0xFF08..=0xFF0E => 0x00,
            0xFF0F => self.if_flag,
            0xFF10..=0xFF3F => self.i2.read(location),
            0xFF40..=0xFF4B => self.lcd.read(location),
            0xFF4C..=0xFF7F => self.i3.read(location),
            _ => panic!("IO unsupported write to {:#04X}", location),
        }
    }
}
