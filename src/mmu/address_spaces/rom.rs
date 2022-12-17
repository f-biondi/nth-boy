use std::{fs::File, io::Result};
use std::os::unix::prelude::FileExt;
use std::str;
use crate::mmu::address_spaces::Addressable;


pub struct Rom {
    name: String,
    rom: Vec<u8>,
}

impl Rom {
    pub fn from_file(path: &str) -> Result<Self> {
        let mut file: File =  File::open(path)?; 
        let name_buffer : Vec<u8> = Self::read_buffer(&mut file, 0x134, 15)?;
        Ok(
            Self{
                name: str::from_utf8(&name_buffer).unwrap().to_string(),
                rom: Self::read_buffer(&mut file, 0x0, 32768)?
            }
        )
    }

    pub fn boot_rom() -> Self {
        let mut file: File =  File::open("roms/DMG_ROM.gb").unwrap(); 
        Self{
            name: String::from("DMG boot rom"),
            rom: Self::read_buffer(&mut file, 0, 0x3FFF).unwrap()
        }
    }

    pub fn print_info(&self) {
        println!("NAME: {}", self.name);
        println!("SIZE: {}", self.rom.len());
    }

    fn read_buffer(input: &mut File, start: u64, buf_size: usize) -> Result<Vec<u8>> {
        let mut buffer : Vec<u8> = vec![0; buf_size];
        input.read_at(&mut buffer, start)?;
        Ok(buffer)
    }
}

impl Addressable for Rom {
    fn write(&mut self, location: u16, byte: u8) {
        panic!("write to rom not implemented");
    }
    fn read(&self, location: u16) -> u8 {
        self.rom[location as usize]
    }
}
