use address_spaces::rom::Rom;
use address_spaces::Addressable;
use std::io::Result;

pub mod address_spaces;

pub struct Mmu {
    rom: Rom,
}

impl Mmu {
    pub fn boot_rom() -> Self {
        Self {
            rom: Rom::boot_rom(),
        }
    }

    pub fn from_file(path: &str) -> Result<Self> {
        Ok(Self {
            rom: Rom::from_file(path)?,
        })
    }
}

impl Addressable for Mmu {
    fn write(&mut self, location: u16, byte: u8) {
        match location {
            0x0000..=0x7FFF => self.rom.write(location, byte),
            _ => panic!("MMU write to unknown location {}", location),
        }
    }
    fn read(&self, location: u16) -> u8 {
        match location {
            0x0000..=0x7FFF => self.rom.read(location),
            _ => panic!("MMU read to unknown location {}", location),
        }
    }
}
