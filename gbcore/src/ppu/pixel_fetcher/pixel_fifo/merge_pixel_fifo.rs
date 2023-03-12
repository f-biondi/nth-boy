use crate::ppu::pixel_fetcher::pixel_fifo::PixelFifo;
use crate::ppu::pixel_fetcher::Pixel;

pub struct MergePixelFifo {
    buffer: Vec<Option<Pixel>>,
    capacity: u8,
    push_i: u8,
    pop_i: u8,
    len: u8,
}

impl MergePixelFifo {
    pub fn with_capacity(capacity: u8) -> MergePixelFifo {
        MergePixelFifo {
            buffer: vec![None; capacity as usize],
            capacity: capacity,
            len: 0,
            push_i: 0,
            pop_i: 0,
        }
    }

    pub fn full_clear(&mut self) {
        self.clear();
        for i in 0..self.capacity {
            self.buffer[i as usize] = None;
        }
    }
}

impl PixelFifo for MergePixelFifo {
    fn push(&mut self, pixel: Pixel) {
        if self.len < self.capacity {
            let new_pixel: Option<Pixel> =
                if let Some(old_pixel) = self.buffer[self.push_i as usize] {
                    if old_pixel.color == 0 {
                        Some(pixel)
                    } else {
                        self.buffer[self.push_i as usize]
                    }
                } else {
                    Some(pixel)
                };
            self.buffer[self.push_i as usize] = new_pixel;
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
        /*while self.len > 0 {
            self.len -= 1;
            self.push_i = if self.push_i == 0 {
                self.capacity - 1
            } else {
                self.push_i - 1
            };
            self.pop_i = self.push_i;
        }*/
        self.push_i = if self.len > self.push_i {
            self.capacity - (self.len - self.push_i)
        } else {
            self.push_i - self.len
        };

        self.pop_i = self.push_i;
        self.len = 0;
    }

    fn len(&mut self) -> u8 {
        self.len
    }
}
