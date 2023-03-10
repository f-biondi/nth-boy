use crate::mmu::address_spaces::Addressable;

pub struct Lcd {
    lcdc: u8,
    pub stat: u8,
    pub scy: u8,
    pub scx: u8,
    ly: u8,
    pub lyc: u8,
    bgp: u8,
    pub obp0: u8,
    pub obp1: u8,
    pub wy: u8,
    pub wx: u8,
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

    pub fn set_ly(&mut self, byte: u8) {
        self.ly = byte;
        self.update_coincidence_flag();
    }

    pub fn inc_ly(&mut self, inc: u8) {
        self.ly = self.ly.wrapping_add(inc);
        self.update_coincidence_flag();
    }

    pub fn get_ly(&self) -> u8 {
        self.ly
    }

    pub fn set_lyc(&mut self, byte: u8) {
        self.lyc = byte;
        self.update_coincidence_flag();
    }

    pub fn get_lyc(&self) -> u8 {
        self.lyc
    }

    pub fn is_window_enabled(&self) -> bool {
        (self.lcdc & 0x20) != 0
    }

    pub fn get_tile_data(&self) -> u16 {
        if (self.lcdc & 0x10) != 0 {
            0x8000
        } else {
            0x9000
        }
    }

    pub fn get_bg_tile_map(&self) -> u16 {
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

    pub fn is_sprite_enabled(&self) -> bool {
        (self.lcdc & 0x02) != 0
    }

    pub fn is_bg_window_enabled(&self) -> bool {
        (self.lcdc & 0x01) != 0
    }

    pub fn oam_stat_enabled(&self) -> bool {
        (self.stat & 0x20) != 0
    }

    pub fn vblank_stat_enabled(&self) -> bool {
        (self.stat & 0x10) != 0
    }

    pub fn hblank_stat_enabled(&self) -> bool {
        (self.stat & 0x08) != 0
    }

    pub fn ly_equal_lyc_stat_enabled(&self) -> bool {
        (self.stat & 0x40) != 0
    }

    fn update_coincidence_flag(&mut self) {
        if self.ly == self.lyc {
            self.stat |= 0x04;
        } else {
            self.stat &= 0xFB;
        }
    }

    fn reset_ppu_mode(&mut self) {
        self.stat &= 0xFC;
    }

    pub fn set_hblank_ppu_mode(&mut self) {
        self.reset_ppu_mode();
    }

    pub fn set_vblank_ppu_mode(&mut self) {
        self.reset_ppu_mode();
        self.stat |= 0x01;
    }

    pub fn set_oam_ppu_mode(&mut self) {
        self.reset_ppu_mode();
        self.stat |= 0x02;
    }

    pub fn set_draw_ppu_mode(&mut self) {
        self.reset_ppu_mode();
        self.stat |= 0x03;
    }

    pub fn get_bgp_index(&self, index: u8) -> u8 {
        match index {
            0 => self.bgp & 0x03,
            1 => (self.bgp & 0x0C) >> 2,
            2 => (self.bgp & 0x30) >> 4,
            3 => (self.bgp & 0xC0) >> 6,
            _ => panic!("BGP index out of range [0;3]"),
        }
    }

    pub fn get_obp0_index(&self, index: u8) -> u8 {
        match index {
            1 => (self.obp0 & 0x0C) >> 2,
            2 => (self.obp0 & 0x30) >> 4,
            3 => (self.obp0 & 0xC0) >> 6,
            _ => panic!("OBP0 index out of range [1;3]"),
        }
    }

    pub fn get_obp1_index(&self, index: u8) -> u8 {
        match index {
            1 => (self.obp1 & 0x0C) >> 2,
            2 => (self.obp1 & 0x30) >> 4,
            3 => (self.obp1 & 0xC0) >> 6,
            _ => panic!("OBP1 index out of range [1;3]"),
        }
    }
}

impl Addressable for Lcd {
    fn write(&mut self, location: u16, byte: u8) {
        match location {
            0xFF40 => self.lcdc = byte,
            0xFF41 => self.stat = byte,
            0xFF42 => self.scy = byte,
            0xFF43 => self.scx = byte,
            0xFF44 => {}
            0xFF45 => self.set_lyc(byte),
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
            0xFF47 => self.bgp,
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            _ => panic!("LCD unsupported write to {:#04X}", location),
        }
    }
}
