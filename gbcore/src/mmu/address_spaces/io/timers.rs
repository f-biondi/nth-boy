use crate::mmu::address_spaces::Addressable;

pub struct Timers {
    sysclk: u16,
    tima: u8,
    tma: u8,
    tac: u8,
}

impl Timers {
    pub fn new() -> Self {
        Self {
            sysclk: 0x00AB,
            tima: 0,
            tma: 0,
            tac: 0xF8,
        }
    }

    pub fn get_sysclk(&mut self) -> u16 {
        self.sysclk
    }

    pub fn inc_sysclk(&mut self) {
        self.sysclk = self.sysclk.wrapping_add(1);
    }

    pub fn inc_tima(&mut self) -> bool {
        if self.tima == 0xff {
            self.tima = self.tma;
            true
        } else {
            self.tima = self.tima.wrapping_add(1);
            false
        }
    }

    pub fn get_tima_clock(&self) -> u16 {
        match self.tac & 0x3 {
            0b00 => 1024,
            0b01 => 16,
            0b10 => 64,
            0b11 => 256,
            _ => panic!("Unexpected tac value {:#b}", self.tac & 0x3),
        }
    }

    pub fn get_tima_enabled(&self) -> bool {
        (self.tac & 0x4) != 0
    }
}

impl Addressable for Timers {
    fn write(&mut self, location: u16, byte: u8) {
        match location {
            0xFF04 => self.sysclk = 0,
            0xFF05 => self.tima = byte,
            0xFF06 => self.tma = byte,
            0xFF07 => self.tac = byte,
            _ => panic!("TIMERS Unsupported write to {:#04X}", location),
        }
    }
    fn read(&self, location: u16) -> u8 {
        match location {
            0xFF04 => ((self.sysclk & 0xff00) >> 8) as u8,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac,
            _ => panic!("TIMERS Unsupported read from {:#04X}", location),
        }
    }
}
