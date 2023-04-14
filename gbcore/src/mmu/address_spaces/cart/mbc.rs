use crate::mmu::address_spaces::cart::header::Header;
use crate::mmu::address_spaces::cart::header::RAM_BANK_SIZE;
use crate::mmu::address_spaces::cart::header::ROM_BANK_SIZE;

pub enum ReadResult {
    Rom(usize),
    Ram(usize),
    Mbc(u8),
    Rtc(u8),
    NoOp,
}

pub enum WriteResult {
    Ram(usize, u8),
    Rtc(u8, u8),
    NoOp,
}

//TODO support for MBC1 >=1 1MB
pub enum Mbc {
    NoMbc,
    Mbc1(bool, u8, u8, bool),
    Mbc2(bool, u8),
    Mbc3(bool, u8, u8),
    Mbc5(bool, u16, u8),
}

impl Mbc {
    pub fn read(&self, header: &Header, add: u16) -> ReadResult {
        match self {
            Mbc::NoMbc => ReadResult::Rom(add as usize),
            Mbc::Mbc1(_, _, _, _) => self.mbc1_read(add),
            Mbc::Mbc2(_, _) => self.mbc2_read(add),
            Mbc::Mbc3(_, _, _) => self.mbc3_read(add),
            Mbc::Mbc5(_, _, _) => self.mbc5_read(add),
        }
    }

    pub fn write(&mut self, header: &Header, add: u16, byte: u8) -> WriteResult {
        match self {
            Mbc::NoMbc => WriteResult::NoOp,
            Mbc::Mbc1(_, _, _, _) => self.mbc1_write(header, add, byte),
            Mbc::Mbc2(_, _) => self.mbc2_write(add, byte),
            Mbc::Mbc3(_, _, _) => self.mbc3_write(add, byte),
            Mbc::Mbc5(_, _, _) => self.mbc5_write(header, add, byte),
        }
    }

    fn mbc1_read(&self, add: u16) -> ReadResult {
        if let Mbc::Mbc1(ram_enabled, s1, s2, mode) = self {
            match add {
                0x0000..=0x3FFF => ReadResult::Rom(add as usize),
                0x4000..=0x7FFF => {
                    ReadResult::Rom(ROM_BANK_SIZE * (*s1 as usize) + ((add - 0x4000) as usize))
                }
                0xA000..=0xBFFF => {
                    if *ram_enabled {
                        match *mode {
                            true => ReadResult::Ram(
                                RAM_BANK_SIZE * (*s2 as usize) + ((add - 0xA000) as usize),
                            ),
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
                }
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
        if let Mbc::Mbc3(ram_enabled, s1, s2) = self {
            match add {
                0x0000..=0x3FFF => ReadResult::Rom(add as usize),
                0x4000..=0x7FFF => {
                    ReadResult::Rom(ROM_BANK_SIZE * (*s1 as usize) + ((add - 0x4000) as usize))
                }
                0xA000..=0xBFFF => match (*s2, *ram_enabled) {
                    (0x0..=0x03, true) => {
                        ReadResult::Ram(RAM_BANK_SIZE * (*s2 as usize) + ((add - 0xA000) as usize))
                    }
                    (0x8..=0x0C, _) => ReadResult::Rtc(*s2),
                    _ => ReadResult::Mbc(0xFF),
                },
                _ => ReadResult::NoOp,
            }
        } else {
            ReadResult::NoOp
        }
    }

    fn mbc3_write(&mut self, add: u16, byte: u8) -> WriteResult {
        if let Mbc::Mbc3(ram_enabled, s1, s2) = self {
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
                }
                0x6000..=0x7FFF => WriteResult::Rtc(0x0D, byte),
                0xA000..=0xBFFF => match (*s2, *ram_enabled) {
                    (0x0..=0x03, true) => WriteResult::Ram(
                        RAM_BANK_SIZE * (*s2 as usize) + ((add - 0xA000) as usize),
                        byte,
                    ),
                    (0x8..=0x0C, true) => WriteResult::Rtc(*s2, byte),
                    _ => WriteResult::NoOp,
                },
                _ => WriteResult::NoOp,
            }
        } else {
            WriteResult::NoOp
        }
    }

    fn mbc5_read(&self, add: u16) -> ReadResult {
        if let Mbc::Mbc5(ram_enabled, s1, s2) = self {
            match add {
                0x0000..=0x3FFF => ReadResult::Rom(add as usize),
                0x4000..=0x7FFF => {
                    ReadResult::Rom(ROM_BANK_SIZE * (*s1 as usize) + ((add - 0x4000) as usize))
                },
                0xA000..=0xBFFF => if *ram_enabled {
                    ReadResult::Ram(RAM_BANK_SIZE * (*s2 as usize) + ((add - 0xA000) as usize))
                } else {
                    ReadResult::Mbc(0xFF)
                },
                _ => ReadResult::NoOp,
            }
        } else {
            ReadResult::NoOp
        }
    }

    fn mbc5_write(&mut self, header: &Header, add: u16, byte: u8) -> WriteResult {
        if let Mbc::Mbc5(ram_enabled, s1, s2) = self {
            match add {
                0x0000..=0x1FFF => {
                    *ram_enabled = (byte & 0x0F) == 0xA;
                    WriteResult::NoOp
                }
                0x2000..=0x2FFF => {
                    *s1 = (*s1 & 0x100) + (byte as u16);
                    WriteResult::NoOp
                }
                0x3000..=0x3FFF => {
                    *s1 &= 0x0FF;
                    *s1 += ((byte & 0x1) as u16) << 8;
                    WriteResult::NoOp
                }
                0x4000..=0x5FFF => {
                    let mask: u8 = if header.has_rumble() {
                        0x07
                    } else {
                        0x0F
                    };
                    *s2 = byte & mask;
                    WriteResult::NoOp
                }
                0xA000..=0xBFFF => if *ram_enabled {
                    WriteResult::Ram(RAM_BANK_SIZE * (*s2 as usize) + ((add - 0xA000) as usize), byte)
                } else {
                    WriteResult::NoOp
                },
                _ => WriteResult::NoOp,
            }
        } else {
            WriteResult::NoOp
        }
    }
}
