use address_spaces::generic_addressable::GenericAddressable;
use address_spaces::io::Io;
use address_spaces::rom::Rom;
use address_spaces::Addressable;
use std::error::Error;
use std::str;
use std::process;

pub mod address_spaces;

pub struct Mmu {
    rom: Rom,
    vram: GenericAddressable,
    wram: GenericAddressable,
    oam: GenericAddressable,
    pub io: Io,
    hram: GenericAddressable,
    ie: u8,
}

impl Mmu {
    pub fn from_file(path: &str) -> Result<Mmu, Box<dyn Error>> {
        Ok(Self {
            rom: Rom::from_file(path)?,
            vram: GenericAddressable::new(0x8000, 0x9FFF)?,
            wram: GenericAddressable::new(0xC000, 0xDFFF)?,
            oam: GenericAddressable::new(0xFE00, 0xFE9F)?,
            io: Io::new()?,
            hram: GenericAddressable::new(0xFF80, 0xFFFE)?,
            ie: 0,
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
            0xFFFF => self.ie = byte,
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
            0xFFFF => self.ie,
        }
    }
}
