use std::error::Error;

//TODO add MBC5
const CART_TYPE_BATTERY: &'static [u8] = &[0x03, 0x06, 0x0F, 0x10, 0x13];

pub struct Header {
    pub title: String,
    pub cart_type: u8,
    pub rom_size: u8,
    ram_size: u8,
}

impl Header {
    pub fn new(rom: &Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let mut title: String = String::from("");
        let mut title_index: u16 = 0x134;

        while title_index <= 0x143 {
            match std::str::from_utf8(&[rom[title_index as usize]]) {
                Ok(title_char) => title += title_char,
                Err(_) => break,
            }
            title_index += 1;
        }

        Ok(Header {
            title: title,
            cart_type: rom[0x147],
            rom_size: rom[0x148],
            ram_size: rom[0x149],
        })
    }

    pub fn get_rom_banks(&self) -> u16 {
        match self.rom_size {
            0 => 2,
            1 => 4,
            2 => 8,
            3 => 16,
            4 => 32,
            6 => 64,
            7 => 128,
            8 => 256,
            5 => 512,
            _ => 0,
        }
    }

    pub fn get_ram_banks(&self) -> u8 {
        match self.ram_size {
            0 | 1 => 0,
            2 => 1,
            3 => 4,
            4 => 16,
            5 => 8,
            _ => 0,
        }
    }

    pub fn has_battery(&self) -> bool {
        CART_TYPE_BATTERY.contains(&self.cart_type)
    }

    pub fn get_ram_size_bytes(&self) -> usize {
        if self.cart_type == 5 || self.cart_type == 6 {
            512
        } else {
            (self.get_ram_banks() as usize) * 8192
        }
    }

    pub fn get_rom_size_bytes(&self) -> usize {
        (self.get_rom_banks() as usize) * 16384
    }

    pub fn get_ram_address(&self, add: usize) -> usize {
        if self.get_ram_banks() > 0 {
            add % self.get_ram_size_bytes()
        } else {
            0
        }
    }

    pub fn get_rom_address(&self, add: usize) -> usize {
        add % self.get_rom_size_bytes()
    }
}
