use crate::mmu::address_spaces::adressable_memory::AdressableMemory;
use crate::mmu::address_spaces::Addressable;
use std::error::Error;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Sprite {
    pub y_position: u8,
    pub x_position: u8,
    pub tile_no: u8,
    pub priority: bool,
    pub y_flip: bool,
    pub x_flip: bool,
    pub palette: bool,
}

pub struct Oam {
    mem: AdressableMemory,
}

impl Oam {
    pub fn new() -> Result<Oam, Box<dyn Error>> {
        Ok(Self {
            mem: AdressableMemory::new(0xFE00, 0xFE9F)?,
        })
    }

    pub fn get_sprite(&self, sprite_id: u8) -> Sprite {
        if sprite_id > 39 {
            panic!("The sprite id {} is not in the range [0, 39]", sprite_id);
        }
        let sprite_start: u16 = 0xFE00 + (4 * sprite_id) as u16;
        Sprite {
            y_position: self.read(sprite_start),
            x_position: self.read(sprite_start + 1),
            tile_no: self.read(sprite_start + 2),
            priority: (self.read(sprite_start + 3) & 0x80) != 0,
            y_flip: (self.read(sprite_start + 3) & 0x40) != 0,
            x_flip: (self.read(sprite_start + 3) & 0x20) != 0,
            palette: (self.read(sprite_start + 3) & 0x10) != 0,
        }
    }

    pub fn fake_read(&self, location: u16) -> u8 {
        self.mem.read(location)
    }
}

impl Addressable for Oam {
    fn write(&mut self, location: u16, byte: u8) {
        //println!("cringe {:#04X} {}", location, byte);
        self.mem.write(location, byte);
    }

    fn read(&self, location: u16) -> u8 {
        self.mem.read(location)
    }
}
