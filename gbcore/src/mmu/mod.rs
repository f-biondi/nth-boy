use address_spaces::adressable_memory::AdressableMemory;
use address_spaces::io::Io;
use address_spaces::rom::Rom;
use address_spaces::Addressable;
use std::error::Error;
use std::str;

pub mod address_spaces;

pub struct Mmu {
    rom: Rom,
    vram: AdressableMemory,
    wram: AdressableMemory,
    oam: AdressableMemory,
    pub io: Io,
    hram: AdressableMemory,
    pub ie_flag: u8,
}

impl Mmu {
    pub fn from_file(path: &str) -> Result<Mmu, Box<dyn Error>> {
        Ok(Self {
            rom: Rom::from_file(path)?,
            vram: AdressableMemory::new(0x8000, 0x9FFF)?,
            wram: AdressableMemory::new(0xC000, 0xDFFF)?,
            oam: AdressableMemory::new(0xFE00, 0xFE9F)?,
            io: Io::new()?,
            hram: AdressableMemory::new(0xFF80, 0xFFFE)?,
            ie_flag: 0,
        })
    }

    pub fn test(&mut self) {
        let res: String = self.io.get_test();
        print!("{}", res);
    }
}

impl Addressable for Mmu {
    fn write(&mut self, location: u16, byte: u8) {
        match location {
            0x0000..=0x3FFF => self.rom.write(location, byte),
            0x4000..=0x7FFF => self.rom.swbank.write(location, byte),
            0x8000..=0x9FFF => self.vram.write(location, byte),
            0xA000..=0xBFFF => self.rom.eram.write(location, byte),
            0xC000..=0xDFFF => self.wram.write(location, byte),
            0xE000..=0xFDFF => self.wram.write(location - 0xE000 + 0xC000, byte),
            0xFE00..=0xFE9F => self.oam.write(location, byte),
            0xFEA0..=0xFEFF => {}
            0xFF00..=0xFF7F => self.io.write(location, byte),
            0xFF80..=0xFFFE => self.hram.write(location, byte),
            0xFFFF => self.ie_flag = byte,
        }
    }

    fn read(&self, location: u16) -> u8 {
        match location {
            0x0000..=0x3FFF => self.rom.read(location),
            0x4000..=0x7FFF => self.rom.swbank.read(location),
            0x8000..=0x9FFF => self.vram.read(location),
            0xA000..=0xBFFF => self.rom.eram.read(location),
            0xC000..=0xDFFF => self.wram.read(location),
            0xE000..=0xFDFF => self.wram.read(location - 0xE000 + 0xC000),
            0xFE00..=0xFE9F => self.oam.read(location),
            0xFEA0..=0xFEFF => 0,
            0xFF00..=0xFF7F => self.io.read(location),
            0xFF80..=0xFFFE => self.hram.read(location),
            0xFFFF => self.ie_flag,
        }
    }
}
