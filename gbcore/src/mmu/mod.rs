use address_spaces::adressable_memory::AdressableMemory;
use address_spaces::io::Io;
use address_spaces::oam::Oam;
use address_spaces::cart::Cart;
use address_spaces::Addressable;
use std::error::Error;
use std::str;

pub mod address_spaces;

pub struct Mmu {
    cart: Cart,
    vram: AdressableMemory,
    wram: AdressableMemory,
    pub oam: Oam,
    pub io: Io,
    dma: u8,
    hram: AdressableMemory,
    pub ie_flag: u8,
}

impl Mmu {
    pub fn from_file(path: &str) -> Result<Mmu, Box<dyn Error>> {
        Ok(Self {
            cart: Cart::from_file(path)?,
            vram: AdressableMemory::new(0x8000, 0x9FFF)?,
            wram: AdressableMemory::new(0xC000, 0xDFFF)?,
            oam: Oam::new()?,
            io: Io::new()?,
            dma: 0,
            hram: AdressableMemory::new(0xFF80, 0xFFFE)?,
            ie_flag: 0,
        })
    }

    /*pub fn dma_tick(&mut self, new_ticks: u8) {
        let mut remaining_ticks = new_ticks;
        while self.dma_running() && remaining_ticks > 0 {
            remaining_ticks -= 1;
            self.dma_ticks += 1;
            if (self.dma_ticks % 4) == 0 && self.dma_ticks > 4 {
                let lsb: u16 = ((self.dma_ticks - 4) / 4) - 1;
                let source_add: u16 = ((self.dma as u16) << 8) | lsb;
                let dest_add: u16 = 0xFE00 | lsb;
                self.dma_current = if self.dma <= 0x7F {
                    self.cart.read(source_add)
                } else {
                    self.wram.read(source_add)
                };
                self.oam.write(dest_add, self.dma_current);
            }
        }
    }*/
    pub fn dma_run(&mut self) {
        let source_msb: u16 = ((self.dma as u16) << 8); 
        for i in 0x0..=0x9f {
            let source_add: u16 = source_msb | i;
            let dest_add: u16 = 0xFE00 | i;
            self.write(dest_add, self.read(source_add));
        }
    }
    pub fn test(&mut self) {
        let res: String = self.io.get_test();
        print!("{}", res);
    }
}

impl Addressable for Mmu {
    fn write(&mut self, location: u16, byte: u8) {
        match location {
            0x0000..=0x3FFF => self.cart.write(location, byte),
            0x4000..=0x7FFF => self.cart.write(location, byte),
            0x8000..=0x9FFF => self.vram.write(location, byte),
            0xA000..=0xBFFF => self.cart.write(location, byte),
            0xC000..=0xDFFF => self.wram.write(location, byte),
            0xE000..=0xFDFF => self.wram.write(location - 0xE000 + 0xC000, byte),
            0xFE00..=0xFE9F => self.oam.write(location, byte),
            0xFEA0..=0xFEFF => {},
            0xFF00..=0xFF45 | 0xFF47..=0xFF7F => self.io.write(location, byte),
            0xFF46 => { self.dma=byte; self.dma_run();},
            0xFF80..=0xFFFE => self.hram.write(location, byte),
            0xFFFF => self.ie_flag = byte,
        }
    }

    fn read(&self, location: u16) -> u8 {
        match location {
            0x0000..=0x3FFF => self.cart.read(location),
            0x4000..=0x7FFF => self.cart.read(location),
            0x8000..=0x9FFF => self.vram.read(location),
            0xA000..=0xBFFF => self.cart.read(location),
            0xC000..=0xDFFF => self.wram.read(location),
            0xE000..=0xFDFF => self.wram.read(location - 0xE000 + 0xC000),
            0xFE00..=0xFE9F => self.oam.read(location),
            0xFEA0..=0xFEFF => 0,
            0xFF00..=0xFF45 | 0xFF47..=0xFF7F => self.io.read(location),
            0xFF46 => self.dma,
            0xFF80..=0xFFFE => self.hram.read(location),
            0xFFFF => self.ie_flag,
        }
    }
}
