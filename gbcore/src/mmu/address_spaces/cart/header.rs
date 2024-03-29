use std::error::Error;

const CART_TYPE_BATTERY: &'static [u8] = &[0x03, 0x06, 0x0F, 0x10, 0x13, 0x1B, 0x1E];
const CART_TYPE_RTC: &'static [u8] = &[0x0F, 0x10];
const CART_TYPE_RUMBLE: &'static [u8] = &[0x1C, 0x1D, 0x1E];
pub const ROM_BANK_SIZE: usize = 16384;
pub const RAM_BANK_SIZE: usize = 8192;
pub const MBC2_RAM_SIZE: usize = 512;

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
        u16::pow(2, (self.rom_size + 1).into())
    }

    pub fn get_ram_banks(&self) -> u8 {
        match self.ram_size {
            0x0 | 0x01 => 0,
            0x02 => 1,
            0x03 => 4,
            0x04 => 16,
            0x05 => 8,
            _ => 0,
        }
    }

    pub fn has_battery(&self) -> bool {
        CART_TYPE_BATTERY.contains(&self.cart_type)
    }

    pub fn has_rtc(&self) -> bool {
        CART_TYPE_RTC.contains(&self.cart_type)
    }

    pub fn has_rumble(&self) -> bool {
        CART_TYPE_RUMBLE.contains(&self.cart_type)
    }

    pub fn get_ram_size_bytes(&self) -> usize {
        if self.cart_type == 5 || self.cart_type == 6 {
            MBC2_RAM_SIZE
        } else {
            (self.get_ram_banks() as usize) * RAM_BANK_SIZE
        }
    }

    pub fn get_rom_size_bytes(&self) -> usize {
        (self.get_rom_banks() as usize) * ROM_BANK_SIZE
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
