use crate::mmu::address_spaces::cart::header::Header;
use std::time::{Duration, SystemTime};

const ROM_BANK_SIZE: usize = 16384;
const RAM_BANK_SIZE: usize = 8192;

pub struct Rtc {
    s: u8,
    m: u8, 
    h: u8,
    dl: u16,

    latched_s: u8,
    latched_m: u8, 
    latched_h: u8,
    latched_dl: u8,
    latched_dh: u8,

    last_update: SystemTime,
    timer_halt: bool,
    day_carry: bool,

    latch_state: bool,
}

impl Rtc {

    pub fn new() -> Rtc {
        Rtc {
            s: 0,
            m: 0, 
            h: 0,
            dl: 0,
            latched_s: 0,
            latched_m: 0, 
            latched_h: 0,
            latched_dl: 0,
            latched_dh: 0,
            last_update: SystemTime::now(),
            timer_halt: false,
            day_carry: false,
            latch_state: false,
        }
    }

    pub fn update_timer(&mut self) {
        if !self.timer_halt {
            let now: SystemTime = SystemTime::now();

            let mut elapsed: Duration = if now >= self.last_update {
                now.duration_since(self.last_update).expect("Time went backwards")
            } else {
                self.last_update.duration_since(now).expect("Time went backwards")
            };
            let days: u64 = (elapsed.as_secs() / 3600) / 24;
            self.day_carry = days > 0x1FF;
            self.dl = (days & 0x1FF) as u16;
            elapsed -= Duration::from_secs(days * 3600 * 24);
            let hours: u64 = (elapsed.as_secs() / 3600);
            self.h = hours as u8;
            elapsed -= Duration::from_secs(hours * 3600);
            let minutes: u64 = (elapsed.as_secs() / 60);
            self.m = minutes as u8;
            elapsed -= Duration::from_secs(minutes * 60);
            self.s = elapsed.as_secs() as u8;
        }
    }

    pub fn update_latch_state(&mut self, value: u8) {
        if !self.latch_state && value == 0 {
            self.latch_state = true;
        } else if self.latch_state && value == 1 {
            self.latch_state = false;
            self.latch_registers();
        }
    }

    pub fn latch_registers(&mut self) {
        self.update_timer();
        self.latched_s = 0b11000000 | self.s;
        self.latched_m = 0b11000000 | self.m; 
        self.latched_h = 0b11100000 | self.h;
        self.latched_dl = (self.dl & 0xFF) as u8;
        self.latched_dh = ((self.dl & 0x100) >> 8) as u8;
        self.latched_dh |= if self.timer_halt {
            1 << 6
        } else {
            0
        };
        self.latched_dh |= if self.day_carry {
            1 << 7
        } else {
            0
        };
    }

    pub fn read(&self, add: u8) -> u8 {
        match add {
            0x08 => self.latched_s,
            0x09 => self.latched_m,
            0x0A => self.latched_h,
            0x0B => self.latched_dl,
            0x0C => self.latched_dh,
            _ => 0,
        }
    }

    pub fn write(&mut self, add: u8, value: u8) {
        match add {
            0x08 => self.write_s(value),
            0x09 => self.write_m(value),
            0x0A => self.write_h(value),
            0x0B => self.write_dl(value),
            0x0C => self.write_dh(value),
            _ => {},
        }
    }

    fn write_s(&mut self, value: u8) {
        self.update_timer();
        println!("second {}", value);
        let new: u8 = value & 0b00111111;
        if new < self.s {
            self.last_update -= Duration::from_secs((self.s - new) as u64);
        } else {
            self.last_update += Duration::from_secs((new - self.s) as u64);
        }
        self.s = new;
        self.latched_s = 0b11000000 | self.s;
    }

    fn write_m(&mut self, value: u8) {
        self.update_timer();
        let new: u8 = value & 0b00111111;
        if new < self.m {
            self.last_update -= Duration::from_secs(((self.m - new) as u64) * 60);
        } else {
            self.last_update += Duration::from_secs(((new - self.m) as u64) * 60);
        }
        self.m = new;
        self.latched_m = 0b11000000 | self.m; 
    }

    fn write_h(&mut self, value: u8) {
        self.update_timer();
        let new: u8 = value & 0b00011111;
        if new < self.h {
            self.last_update -= Duration::from_secs(((self.h - new) as u64) * 3600);
        } else {
            self.last_update += Duration::from_secs(((new - self.h) as u64) * 3600);
        }
        self.h = new;
        self.latched_h = 0b11100000 | self.h;
    }

    fn write_dl(&mut self, value: u8) {
        self.update_timer();
        let old: u16 = self.dl;
        let new: u16 = (self.dl & 0x100) | (value as u16);

        if new < old {
            self.last_update -= Duration::from_secs(((old - new) as u64) * 3600 * 24);
        } else {
            self.last_update += Duration::from_secs(((new - old) as u64) * 3600 * 24);
        }
        self.dl |= (value as u16);
        self.latched_dl = value;
    }

    fn write_dh(&mut self, value: u8) {
        self.update_timer();
        self.latched_dh = value & 0b11000001;
        self.timer_halt = (self.latched_dh & 0x40) != 0;
        self.day_carry = (self.latched_dh & 0x60) != 0;

        let old: u16 = self.dl;
        let new: u16 = (((self.latched_dh & 0x01) as u16) << 8) | (value as u16);

        if new < old {
            self.last_update -= Duration::from_secs(((old - new) as u64) * 3600 * 24);
        } else {
            self.last_update += Duration::from_secs(((new - old) as u64) * 3600 * 24);
        }
    }

}

pub enum ReadResult {
    Rom(usize),
    Ram(usize),
    Mbc(u8),
    NoOp,
}

pub enum WriteResult {
    Ram(usize, u8),
    NoOp,
}

//TODO support for MBC1 >=1 1MB
pub enum Mbc {
    NoMbc,
    Mbc1(bool, u8, u8, bool),
    Mbc2(bool, u8),
    Mbc3(bool, u8, u8, Rtc),
}

impl Mbc {
    pub fn read(&self, header: &Header, add: u16) -> ReadResult {
        match self {
            Mbc::NoMbc => ReadResult::Rom(add as usize),
            Mbc::Mbc1(_, _, _, _) => self.mbc1_read(header, add),
            Mbc::Mbc2(_, _) => self.mbc2_read(add),
            Mbc::Mbc3(_, _, _, _) => self.mbc3_read(add),
            _ => ReadResult::NoOp,
        }
    }

    pub fn write(&mut self, header: &Header, add: u16, byte: u8) -> WriteResult {
        match self {
            Mbc::NoMbc => WriteResult::NoOp,
            Mbc::Mbc1(_, _, _, _) => self.mbc1_write(header, add, byte),
            Mbc::Mbc2(_, _) => self.mbc2_write(add, byte),
            Mbc::Mbc3(_, _, _, _) => self.mbc3_write(add, byte),
            _ => WriteResult::NoOp,
        }
    }

    fn mbc1_read(&self, header: &Header, add: u16) -> ReadResult {
        if let Mbc::Mbc1(ram_enabled, s1, s2, mode) = self {
            match add {
                0x0000..=0x3FFF => ReadResult::Rom(add as usize),
                0x4000..=0x7FFF => {
                    ReadResult::Rom(ROM_BANK_SIZE * (*s1 as usize) + ((add - 0x4000) as usize))
                }
                0xA000..=0xBFFF => {
                    if *ram_enabled {
                        match *mode {
                            true => {
                                ReadResult::Ram(RAM_BANK_SIZE * (*s2 as usize) + ((add - 0xA000) as usize))
                            }
                            false => ReadResult::Ram((add - 0xA000) as usize),
                        }
                    } else {
                        ReadResult::Mbc(0xFF)
                    }
                }
                _ => ReadResult::NoOp,
            }
        } else {
            ReadResult::NoOp
        }
    }

    fn mbc1_write(&mut self, header: &Header, add: u16, byte: u8) -> WriteResult {
        if let Mbc::Mbc1(ram_enabled, s1, s2, mode) = self {
            match add {
                0x0000..=0x1FFF => {
                    *ram_enabled = (byte & 0x0F) == 0xA;
                    WriteResult::NoOp
                }
                0x2000..=0x3FFF => {
                    *s1 = u8::max(byte & 0x1f, 1);
                    WriteResult::NoOp
                }
                0x4000..=0x5FFF => {
                    *s2 = byte & 0x3;
                    WriteResult::NoOp
                }
                0x6000..=0x7FFF => {
                    *mode = (byte & 0x1) != 0;
                    WriteResult::NoOp
                }
                0xA000..=0xBFFF => {
                    if *ram_enabled {
                        match (header.get_ram_banks(), *mode) {
                            (0 | 1, _) => WriteResult::Ram(
                                ((add - 0xA000) as usize) % header.get_ram_size_bytes(),
                                byte,
                                ),
                                (_, true) => WriteResult::Ram(
                                    RAM_BANK_SIZE * (*s2 as usize) + ((add - 0xA000) as usize),
                                    byte,
                                    ),
                                    (_, false) => WriteResult::Ram((add - 0xA000) as usize, byte),
                        }
                    } else {
                        WriteResult::NoOp
                    }
                }
                _ => WriteResult::NoOp,
            }
        } else {
            WriteResult::NoOp
        }
    }

    fn mbc2_read(&self, add: u16) -> ReadResult {
        if let Mbc::Mbc2(ram_enabled, s1) = self {
            match add {
                0x0000..=0x3FFF => ReadResult::Rom(add as usize),
                0x4000..=0x7FFF => {
                    ReadResult::Rom(ROM_BANK_SIZE * (*s1 as usize) + ((add - 0x4000) as usize))
                }
                0xA000..=0xBFFF => {
                    if *ram_enabled {
                        ReadResult::Ram((add & 0x1FF) as usize)
                    } else {
                        ReadResult::Mbc(0xFF)
                    }
                }
                _ => ReadResult::NoOp,
            }
        } else {
            ReadResult::NoOp
        }
    }

    fn mbc2_write(&mut self, add: u16, byte: u8) -> WriteResult {
        if let Mbc::Mbc2(ram_enabled, s1) = self {
            match add {
                0x0000..=0x3FFF => {
                    if (add & 0x0100) != 0 {
                        *s1 = u8::max(byte, 1);
                    } else {
                        *ram_enabled = (byte & 0x0F) == 0xA;
                    }
                    WriteResult::NoOp
                },
                0xA000..=0xBFFF => {
                    if *ram_enabled {
                        WriteResult::Ram((add & 0x1FF) as usize, 0xF0 | (byte & 0x0F))
                    } else {
                        WriteResult::NoOp
                    }
                }
                _ => WriteResult::NoOp,
            }
        } else {
            WriteResult::NoOp
        }
    }

    fn mbc3_read(&self, add: u16) -> ReadResult {
        if let Mbc::Mbc3(ram_enabled, s1, s2, rtc) = self {
            match add {
                0x0000..=0x3FFF => ReadResult::Rom(add as usize),
                0x4000..=0x7FFF => {
                    ReadResult::Rom(ROM_BANK_SIZE * (*s1 as usize) + ((add - 0x4000) as usize))
                },
                0xA000..=0xBFFF => {
                    match (*s2, *ram_enabled) {
                        (0x0..=0x03, true) => ReadResult::Ram(RAM_BANK_SIZE * (*s2 as usize) + ((add - 0xA000) as usize)),
                        (0x8..=0x0C, _) => ReadResult::Mbc(rtc.read(*s2)),
                        _ => ReadResult::Mbc(0xFF),
                    }
                },
                _ => ReadResult::NoOp,
            }
        } else {
            ReadResult::NoOp
        }
    }

    fn mbc3_write(&mut self, add: u16, byte: u8) -> WriteResult {
        if let Mbc::Mbc3(ram_enabled, s1, s2, rtc) = self {
            match add {
                0x0000..=0x1FFF => {
                    *ram_enabled = (byte & 0x0F) == 0xA;
                    WriteResult::NoOp
                }
                0x2000..=0x3FFF => {
                    *s1 = byte & 0b01111111;
                    if *s1 == 0 {
                        *s1 = 1;
                    }
                    WriteResult::NoOp
                }
                0x4000..=0x5FFF => {
                    *s2 = byte % 0x0C;
                    WriteResult::NoOp
                },
                0x6000..=0x7FFF => {
                    rtc.update_latch_state(byte);
                    WriteResult::NoOp
                },
                0xA000..=0xBFFF => match (*s2, *ram_enabled) {
                    (0x0..=0x03, true) => WriteResult::Ram(RAM_BANK_SIZE * (*s2 as usize) + ((add - 0xA000) as usize), byte),
                    (0x8..=0x0C, true) => {
                        rtc.write(*s2, byte);
                        WriteResult::NoOp
                    },
                    _ => WriteResult::NoOp
                },
                _ => WriteResult::NoOp,
            }
        } else {
            WriteResult::NoOp
        }
    }
}
