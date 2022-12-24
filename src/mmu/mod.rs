use address_spaces::rom::Rom;
use address_spaces::Addressable;
use std::io::Result;
use std::str;

pub mod address_spaces;

pub struct Mmu {
    rom: Rom,
    ram: Vec<u8>,
    test: String,
}

impl Mmu {
    pub fn boot_rom() -> Self {
        Self {
            rom: Rom::boot_rom(),
            ram: vec![0; 0xffff - 0x7fff],
            test: String::from(""),
        }
    }

    pub fn from_file(path: &str) -> Result<Self> {
        let mut ram: Vec<u8> = vec![0; 0xffff - 0x7fff];
        ram[0xFF44 - 0x8000] = 0x90;
        Ok(Self {
            rom: Rom::from_file(path)?,
            ram: ram,
            test: String::from(""),
        })
    }

    pub fn test(&self) {
        println!("Test: {}", self.test);
    }
}

impl Addressable for Mmu {
    fn write(&mut self, location: u16, byte: u8) {
        match location {
            0x0..=0x7FFF => self.rom.write(location, byte),
            0xff02 => {
                if byte == 0x81 {
                    let cb: [u8; 1] = [self.ram[0xff01 - 0x8000]];
                    let c: &str = str::from_utf8(&cb).unwrap();
                    self.test += c;
                    self.ram[0xff02 - 0x8000] = 0x0;
                }
            }
            _ => self.ram[(location - 0x8000) as usize] = byte,
        }
    }
    fn read(&self, location: u16) -> u8 {
        match location {
            0x0..=0x7FFF => self.rom.read(location),
            _ => self.ram[(location - 0x8000) as usize],
        }
    }
}
