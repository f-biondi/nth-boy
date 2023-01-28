use std::error::Error;
use crate::mmu::address_spaces::adressable_memory::AdressableMemory;
use crate::mmu::address_spaces::Addressable;

pub struct Sprite {
    pub y_position: u8,
    pub x_position: u8,
    pub tile_id: u8,
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
            panic!("The id {} is not in the range [0, 39]", sprite_id);
        }
        let sprite_start: u16 = 0xFE00 + (4 * sprite_id) as u16;
        Sprite {
            y_position: self.mem.read(sprite_start),
            x_position: self.mem.read(sprite_start+1),
            tile_id: self.mem.read(sprite_start+2),
            priority: (self.mem.read(sprite_start+3) & 0x80) != 0,
            y_flip: (self.mem.read(sprite_start+3) & 0x40) != 0,
            x_flip: (self.mem.read(sprite_start+3) & 0x20) != 0,
            palette: (self.mem.read(sprite_start+3) & 0x10) != 0,
        }
    }
}

impl Addressable for Oam {
    fn write(&mut self, location: u16, byte: u8) {
        self.mem.write(location, byte);
    }

    fn read(&self, location: u16) -> u8 {
        self.mem.read(location)
    }
}
