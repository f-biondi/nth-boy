use crate::mmu::address_spaces::adressable_memory::AdressableMemory;
use crate::mmu::address_spaces::cart::header::Header;
use crate::mmu::address_spaces::Addressable;
use crate::mmu::address_spaces::cart::mbc::Mbc;
use crate::mmu::address_spaces::cart::mbc::MbcResult;

use std::error::Error;
use std::fs;
use std::fs::File;
use std::os::unix::prelude::FileExt;
use std::str;

mod header;
mod mbc;

pub struct Cart {
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
            _ => Err(String::from(format!("Unsopported mbc {:#02X}", header.cart_type))),
        }
    }

    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let file: Vec<u8> = fs::read(path)?;
        let header: Header = Header::new(&file)?;
        println!("Loading {} {}", header.title, header.cart_type);
        Ok(Self {
            mbc: Cart::get_mbc(&header)?,
            rom: file,
            ram: vec![0; header.get_ram_size_bytes()],
            header: header,
        })
    }
}

impl Addressable for Cart {
    fn write(&mut self, location: u16, byte: u8) {
        match self.mbc.write(location, byte) {
            MbcResult::Ram(location) => {
                if self.ram.len() > 0 {
                    self.ram[self.header.get_ram_address(location)] = byte
                }
            },
            _ => {}
        }
    }
    fn read(&self, location: u16) -> u8 {
        match self.mbc.read(location) {
            MbcResult::Rom(location) => self.rom[self.header.get_rom_address(location)],
            MbcResult::Ram(location) => self.ram[self.header.get_ram_address(location)],
            MbcResult::Mbc(value) => value,
            _ => 0x0,
        }
    }
}
