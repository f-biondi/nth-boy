use crate::mmu::address_spaces::cart::header::Header;

#[derive(Debug)]
pub enum MbcResult {
    Rom(usize),
    Ram(usize),
    Mbc(u8),
    NoOp,
}

pub enum Mbc {
    NoMbc,
    Mbc1(bool, u8, u8, bool)
}

impl Mbc {
    pub fn read(&self, add: u16) -> MbcResult {
        match self {
            Mbc::NoMbc => MbcResult::Rom(add as usize),
            Mbc::Mbc1(ram_enabled, s1, s2, mode) => match add { 
                0x0000..=0x3FFF => if !mode {
                    MbcResult::Rom(add as usize)
                } else {
                    MbcResult::Rom((16384 * (*s2 as usize)) + (add as usize))
                },
                0x4000..=0x7FFF => {
                    let slot: usize = ((s2 << 5) | s1) as usize;
                    MbcResult::Rom((16384 * slot) + ((add - 0x4000) as usize))
                },
                0xA000..=0xBFFF => if !mode && *ram_enabled {
                    MbcResult::Ram((add - 0xA000) as usize)
                } else if *ram_enabled {
                    MbcResult::Ram((8192 * (*s2 as usize)) + ((add - 0xA000) as usize))
                } else {
                    MbcResult::Mbc(0xFF)
                },
                _ => MbcResult::NoOp,
            },
            _ => MbcResult::NoOp
        }
    }

    pub fn write(&mut self, add: u16, byte: u8) -> MbcResult {
        match self {
            Mbc::NoMbc => MbcResult::NoOp,
            Mbc::Mbc1(ram_enabled, s1, s2, mode) => match add { 
                0x0000..=0x1FFF => {
                    *ram_enabled = (byte & 0x0F) == 0xA;
                    MbcResult::NoOp
                }
                0x2000..=0x3FFF => {
                    *s1 = if byte != 0 {
                        byte & 0b00011111
                    } else {
                        1
                    };
                    MbcResult::NoOp
                },
                0x4000..=0x5FFF => {
                    *s2 = byte & 0x3;
                    MbcResult::NoOp
                },
                0x6000..=0x7FFF => {
                    *mode = (byte & 0x1) != 0;
                    MbcResult::NoOp
                },
                0xA000..=0xBFFF => if !*mode && *ram_enabled {
                    MbcResult::Ram((add - 0xA000) as usize)
                } else if *ram_enabled {
                    MbcResult::Ram((8192 * (*s2 as usize)) + ((add - 0xA000) as usize))
                } else {
                    MbcResult::NoOp
                },
                _ => MbcResult::NoOp,
            }
            _ => MbcResult::NoOp
        }
    }
}
