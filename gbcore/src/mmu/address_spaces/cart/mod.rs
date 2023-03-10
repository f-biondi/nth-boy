use crate::mmu::address_spaces::adressable_memory::AdressableMemory;
use crate::mmu::address_spaces::cart::header::Header;
use crate::mmu::address_spaces::cart::mbc::Mbc;
use crate::mmu::address_spaces::cart::mbc::ReadResult;
use crate::mmu::address_spaces::cart::mbc::WriteResult;
use crate::mmu::address_spaces::Addressable;

use std::error::Error;
use std::fs;
use std::fs::File;
use std::os::unix::prelude::FileExt;
use std::str;

mod header;
mod mbc;

pub struct Cart {
    path: String,
    save_path: String,
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
            _ => Err(String::from(format!(
                "Unsopported mbc {:#02X}",
                header.cart_type
            ))),
        }
    }

    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let file: Vec<u8> = fs::read(path)?;
        let save_path: String = format!("{}.{}", path, "sav");
        let header: Header = Header::new(&file)?;
        let mut ram: Vec<u8> = vec![0; header.get_ram_size_bytes()];
        if header.has_battery() {
            if let Ok(save) = fs::read(&save_path) {
                for i in 0..save.len() {
                    if i < ram.len() {
                        ram[i] = save[i];
                    }
                }
            }
        }
        Ok(Self {
            path: String::from("path"),
            save_path: save_path,
            mbc: Cart::get_mbc(&header)?,
            rom: file,
            ram: ram,
            header: header,
        })
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        if self.header.has_battery() {
            fs::write(&self.save_path, &self.ram)?;
        }
        Ok(())
    }
}

impl Addressable for Cart {
    fn write(&mut self, location: u16, byte: u8) {
        match self.mbc.write(&self.header, location, byte) {
            WriteResult::Ram(location, byte_res) => {
                if self.ram.len() > 0 {
                    self.ram[location] = byte_res
                }
            }
            _ => {}
        }
    }
    fn read(&self, location: u16) -> u8 {
        match self.mbc.read(&self.header, location) {
            ReadResult::Rom(location) => self.rom[location],
            ReadResult::Ram(location) => self.ram[location],
            ReadResult::Mbc(value) => value,
            _ => 0x0,
        }
    }
}
