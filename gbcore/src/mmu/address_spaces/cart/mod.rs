use crate::mmu::address_spaces::cart::header::Header;
use crate::mmu::address_spaces::cart::mbc::Mbc;
use crate::mmu::address_spaces::cart::mbc::ReadResult;
use crate::mmu::address_spaces::cart::mbc::WriteResult;
use crate::mmu::address_spaces::cart::rtc::Rtc;
use crate::mmu::address_spaces::Addressable;

use std::error::Error;
use std::fs;
use std::str;

mod header;
mod mbc;
mod rtc;

pub struct Cart {
    path: String,
    rtc: Option<Rtc>,
    rom: Vec<u8>,
    ram: Vec<u8>,
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
            _ => Err(String::from(format!(
                "Unsopported mbc {:#02X}",
                header.cart_type
            ))),
        }
    }

    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let file: Vec<u8> = fs::read(path)?;
        let header: Header = Header::new(&file)?;
        let mut ram: Vec<u8> = vec![0; header.get_ram_size_bytes()];
        if header.has_battery() {
            if let Ok(save) = fs::read(format!("{}.{}", path, "sav")) {
                for i in 0..save.len() {
                    if i < ram.len() {
                        ram[i] = save[i];
                    }
                }
            }
        }

        let rtc: Option<Rtc> = if header.has_rtc() {
            if let Ok(rtc_data) = fs::read(format!("{}.{}", path, "rtc")) {
                Some(Rtc::deserialize(&rtc_data))
            } else {
                Some(Rtc::new())
            }
        } else {
            None
        };

        Ok(Self {
            path: String::from(path),
            mbc: Cart::get_mbc(&header)?,
            rtc: rtc,
            rom: file,
            ram: ram,
            header: header,
        })
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        if self.header.has_battery() {
            fs::write(format!("{}.{}", self.path, "sav"), &self.ram)?;
        }
        if let Some(rtc) = &self.rtc {
            fs::write(format!("{}.{}", self.path, "rtc"), &rtc.serialize())?;
        }
        Ok(())
    }
}

impl Addressable for Cart {
    fn write(&mut self, location: u16, byte: u8) {
        match self.mbc.write(&self.header, location, byte) {
            WriteResult::Ram(location, byte_res) => {
                if self.ram.len() > 0 {
                    self.ram[location] = byte_res;
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
            ReadResult::Rom(location) => self.rom[location],
            ReadResult::Ram(location) => self.ram[location],
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
