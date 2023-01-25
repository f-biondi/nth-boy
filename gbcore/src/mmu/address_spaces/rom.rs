use crate::mmu::address_spaces::adressable_memory::AdressableMemory;
use crate::mmu::address_spaces::Addressable;

use std::error::Error;
use std::fs;
use std::fs::File;
use std::os::unix::prelude::FileExt;
use std::str;

pub struct Rom {
    rom: AdressableMemory,
    pub swbank: AdressableMemory,
    pub eram: AdressableMemory,
}

impl Rom {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let mut file: Vec<u8> = fs::read(path)?;
        let mut rom: AdressableMemory = AdressableMemory::new(0x0000, 0x3FFF)?;
        let mut swbank: AdressableMemory = AdressableMemory::new(0x4000, 0x7FFF)?;

        for i in 0x000..=0x7FFF {
            if i < 0x4000 {
                rom.write(i, file[i as usize]);
            } else {
                swbank.write(i, file[i as usize]);
            }
        }

        Ok(Self {
            rom: rom,
            swbank: swbank,
            eram: AdressableMemory::new(0xA000, 0xBFFF)?,
        })
    }
}

impl Addressable for Rom {
    fn write(&mut self, location: u16, byte: u8) {
        panic!("write to rom not implemented {:#X}", location);
    }
    fn read(&self, location: u16) -> u8 {
        self.rom.read(location)
    }
}
