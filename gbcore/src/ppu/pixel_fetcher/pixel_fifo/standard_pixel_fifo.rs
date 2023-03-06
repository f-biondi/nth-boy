use crate::ppu::pixel_fetcher::pixel_fifo::PixelFifo;
use crate::ppu::pixel_fetcher::Palette;
use crate::ppu::pixel_fetcher::Pixel;

pub struct StandardPixelFifo {
    buffer: Vec<Option<Pixel>>,
    capacity: u8,
    push_i: u8,
    pop_i: u8,
    len: u8,
}

impl StandardPixelFifo {
    pub fn with_capacity(capacity: u8) -> StandardPixelFifo {
        StandardPixelFifo {
            buffer: vec![None; capacity as usize],
            capacity: capacity,
            len: 0,
            push_i: 0,
            pop_i: 0,
        }
    }
}

impl PixelFifo for StandardPixelFifo {
    fn push(&mut self, pixel: Pixel) {
        if self.len < self.capacity {
            self.buffer[self.push_i as usize] = Some(pixel);
            self.push_i = (self.push_i + 1) % self.capacity;
            self.len += 1;
        }
    }

    fn shift(&mut self) -> Option<Pixel> {
        let mut res: Option<Pixel> = None;
        if self.len > 0 {
            res = self.buffer[self.pop_i as usize];
            self.buffer[self.pop_i as usize] = None;
            self.pop_i = (self.pop_i + 1) % self.capacity;
            self.len -= 1;
        }
        res
    }

    fn clear(&mut self) {
        self.push_i = 0;
        self.pop_i = 0;
        self.len = 0;
    }

    fn len(&mut self) -> u8 {
        self.len
    }
}
