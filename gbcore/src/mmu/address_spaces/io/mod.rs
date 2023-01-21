use crate::mmu::address_spaces::generic_addressable::GenericAddressable;
use crate::mmu::address_spaces::Addressable;
use timers::Timers;
use std::error::Error;

mod timers;

pub struct Io {
    i1: GenericAddressable,
    pub timers: Timers,
    i2: GenericAddressable,
    if_flag: u8,
    pub test: String,
}

impl Io {
    pub fn new() -> Result<Io, Box<dyn Error>> {
        let mut i1: GenericAddressable = GenericAddressable::new(0xFF00, 0xFF03)?;
        let mut timers: Timers = Timers::new();
        let mut i2: GenericAddressable = GenericAddressable::new(0xFF08, 0xFF7F)?;
        i2.write(0xFF44, 0x90);
        Ok(Self {
            i1: i1,
            timers: timers,
            i2: i2,
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
            0xFF00..=0xFF03 => {
                if location == 0xFF02 && byte == 0x81 {
                    let cb: [u8; 1] = [self.i1.read(0xff01)];
                    let c: &str = std::str::from_utf8(&cb).unwrap();
                    self.test = String::from(c);
                    self.i1.write(0xff02, 0x0);
                } else {
                    self.i1.write(location, byte);
                }
            }
            0xFF04..=0xFF07 => self.timers.write(location, byte),
            0xFF08..=0xFF0E => self.i2.write(location, byte),
            0xFF0F => self.if_flag = byte, 
            0xFF10..=0xFF7F => self.i2.write(location, byte),
            _ => panic!("IO unsupported write to {:#04X}", location),
        }
    }

    fn read(&self, location: u16) -> u8 {
        match location {
            0xFF00..=0xFF03 => self.i1.read(location),
            0xFF04..=0xFF07 => self.timers.read(location),
            0xFF08..=0xFF0E => self.i2.read(location),
            0xFF0F => self.if_flag, 
            0xFF10..=0xFF7F => self.i2.read(location),
            _ => panic!("IO unsupported write to {:#04X}", location),
        }
    }
}
