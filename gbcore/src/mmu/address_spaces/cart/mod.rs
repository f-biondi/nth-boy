use crate::mmu::address_spaces::cart::header::Header;
use crate::mmu::address_spaces::cart::mbc::Mbc;
use crate::mmu::address_spaces::cart::mbc::ReadResult;
use crate::mmu::address_spaces::cart::mbc::WriteResult;
use crate::mmu::address_spaces::cart::rtc::Rtc;
use crate::mmu::address_spaces::Addressable;

use std::error::Error;
use std::str;

mod header;
mod mbc;
mod rtc;

pub struct Cart {
    rom: Vec<u8>,
    ram: Option<Vec<u8>>,
    rtc: Option<Rtc>,
    header: Header,
    mbc: Mbc,
}

impl Cart {
    fn get_mbc(header: &Header) -> Result<Mbc, String> {
        match header.cart_type {
            0x0 => Ok(Mbc::NoMbc),
            0x1 | 0x2 | 0x3 => Ok(Mbc::Mbc1(false, 1, 0, false)),
            0x5 | 0x6 => Ok(Mbc::Mbc2(false, 1)),
            0x0F..=0x13 => Ok(Mbc::Mbc3(false, 1, 0)),
            0x19..=0x1E => Ok(Mbc::Mbc5(false, 0, 0)),
            _ => Err(String::from(format!(
                "Unsopported mbc {:#02X}",
                header.cart_type
            ))),
        }
    }

    pub fn new(
        rom: Vec<u8>,
        ram: Option<Vec<u8>>,
        rtc: Option<Vec<u8>>,
    ) -> Result<Self, Box<dyn Error>> {
        let header: Header = Header::new(&rom)?;
        Ok(Self {
            mbc: Cart::get_mbc(&header)?,
            rtc: if header.has_rtc() {
                if let Some(rtc_data) = rtc {
                    Some(Rtc::deserialize(&rtc_data))
                } else {
                    Some(Rtc::new())
                }
            } else {
                None
            },
            rom: rom,
            ram: if header.has_battery() {
                if let Some(ram_data) = ram {
                    Some(ram_data)
                } else {
                    Some(vec![0; header.get_ram_size_bytes()])
                }
            } else {
                None
            },
            header: header,
        })
    }

    pub fn update_rtc_now(&mut self, elapsed_secs: u64) {
        if let Some(rtc) = &mut self.rtc {
            rtc.update_now(elapsed_secs);
        }
    }

    pub fn dump_ram(&self) -> Option<Vec<u8>> {
        if let Some(ram) = &self.ram {
            Some(ram.clone())
        } else {
            None
        }
    }

    pub fn dump_rtc(&self) -> Option<Vec<u8>> {
        if let Some(rtc) = &self.rtc {
            Some(rtc.serialize())
        } else {
            None
        }
    }
}

impl Addressable for Cart {
    fn write(&mut self, location: u16, byte: u8) {
        match self.mbc.write(&self.header, location, byte) {
            WriteResult::Ram(location, byte_res) => {
                if let Some(ram) = &mut self.ram {
                    ram[location % self.header.get_ram_size_bytes()] = byte_res;
                }
            }
            WriteResult::Rtc(location, value) => {
                if let Some(rtc) = &mut self.rtc {
                    rtc.write(location, value);
                }
            }
            _ => {}
        }
    }
    fn read(&self, location: u16) -> u8 {
        match self.mbc.read(&self.header, location) {
            ReadResult::Rom(location) => self.rom[location % self.header.get_rom_size_bytes()],
            ReadResult::Ram(location) => {
                if let Some(ram) = &self.ram {
                    ram[location % self.header.get_ram_size_bytes()]
                } else {
                    0x0
                }
            }
            ReadResult::Rtc(location) => {
                if let Some(rtc) = &self.rtc {
                    rtc.read(location)
                } else {
                    0x0
                }
            }
            ReadResult::Mbc(value) => value,
            _ => 0x0,
        }
    }
}
