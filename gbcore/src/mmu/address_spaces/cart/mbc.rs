use crate::mmu::address_spaces::cart::header::Header;

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

pub enum Mbc {
    NoMbc,
    Mbc1(bool, u8, u8, bool),
    Mbc2(bool, u8),
}

impl Mbc {
    pub fn read(&self, header: &Header, add: u16) -> ReadResult {
        match self {
            Mbc::NoMbc => ReadResult::Rom(add as usize),
            Mbc::Mbc1(_, _, _, _) => self.mbc1_read(header, add),
            Mbc::Mbc2(_, _) => self.mbc2_read(add),
            _ => ReadResult::NoOp,
        }
    }

    pub fn write(&mut self, header: &Header, add: u16, byte: u8) -> WriteResult {
        match self {
            Mbc::NoMbc => WriteResult::NoOp,
            Mbc::Mbc1(_, _, _, _) => self.mbc1_write(header, add, byte),
            Mbc::Mbc2(_, _) => self.mbc2_write(add, byte),
            _ => WriteResult::NoOp,
        }
    }

    fn mbc1_read(&self, header: &Header, add: u16) -> ReadResult {
        if let Mbc::Mbc1(ram_enabled, s1, s2, mode) = self {
            match add {
                0x0000..=0x3FFF => ReadResult::Rom(add as usize),
                0x4000..=0x7FFF => {
                    ReadResult::Rom(16384 * (*s1 as usize) + ((add - 0x4000) as usize))
                }
                0xA000..=0xBFFF => {
                    if *ram_enabled {
                        match *mode {
                            true => {
                                ReadResult::Ram(8192 * (*s2 as usize) + ((add - 0xA000) as usize))
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
                                8192 * (*s2 as usize) + ((add - 0xA000) as usize),
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
                    ReadResult::Rom(16384 * (*s1 as usize) + ((add - 0x4000) as usize))
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
}
