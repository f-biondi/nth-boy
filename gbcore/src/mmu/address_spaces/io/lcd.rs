use crate::mmu::address_spaces::Addressable;

pub struct Lcd {
    lcdc: u8,
    stat: u8,
    scy: u8,
    scx: u8,
    pub ly: u8,
    lyc: u8,
    dma: u8,
    bgp: u8,
    obp0: u8,
    obp1: u8,
    wy: u8,
    wx: u8,
}

impl Lcd {
    pub fn new() -> Lcd {
        Self {
            lcdc: 0x91,
            stat: 0x85,
            scy: 0x00,
            scx: 0x00,
            ly: 0x00,
            lyc: 0x00,
            dma: 0xFF,
            bgp: 0xFC,
            obp0: 0x00,
            obp1: 0x00,
            wy: 0x00,
            wx: 0x00,
        }
    }

    pub fn is_display_enabled(&self) -> bool {
        (self.lcdc & 0x80) != 0
    }

    pub fn get_window_tile_map(&self) -> u16 {
        if (self.lcdc & 0x40) != 0 {
            0x9C00
        } else {
            0x9800
        }
    }

    pub fn is_window_enabled(&self) -> bool {
        (self.lcdc & 0x20) != 0
    }

    pub fn get_tile_data(&self) -> u16 {
        if (self.lcdc & 0x10) != 0 {
            0x8000
        } else {
            0x8800
        }
    }

    pub fn get_bg_tile_data(&self) -> u16 {
        if (self.lcdc & 0x08) != 0 {
            0x9C00
        } else {
            0x9800
        }
    }

    pub fn get_sprite_size(&self) -> u8 {
        if (self.lcdc & 0x04) != 0 {
            16
        } else {
            8
        }
    }

    pub fn is_sprites_enabled(&self) -> bool {
        (self.lcdc & 0x02) != 0
    }

    pub fn is_bg_window_enabled(&self) -> bool {
        (self.lcdc & 0x01) != 0
    }


}

impl Addressable for Lcd {
    fn write(&mut self, location: u16, byte: u8) {
        match location {
            0xFF40 => self.lcdc = byte,
            0xFF41 => self.stat = byte,
            0xFF42 => self.scy = byte,
            0xFF43 => self.scx = byte,
            0xFF44 => {},
            0xFF45 => self.lyc = byte,
            0xFF46 => self.dma = byte,
            0xFF47 => self.bgp = byte,
            0xFF48 => self.obp0 = byte,
            0xFF49 => self.obp1 = byte,
            0xFF4A => self.wy = byte,
            0xFF4B => self.wx = byte,
            _ => panic!("LCD unsupported write to {:#04X}", location),
        }
    }

    fn read(&self, location: u16) -> u8 {
        match location {
            0xFF40 => self.lcdc,
            0xFF41 => self.stat,
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF46 => self.dma,
            0xFF47 => self.bgp,
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            _ => panic!("LCD unsupported write to {:#04X}", location),
        }
    }
}
