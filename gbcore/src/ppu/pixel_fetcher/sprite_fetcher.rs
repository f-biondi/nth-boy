use crate::mmu::address_spaces::Addressable;
use crate::ppu::pixel_fetcher::pixel_fifo::merge_pixel_fifo::MergePixelFifo;
use crate::ppu::pixel_fetcher::pixel_fifo::PixelFifo;
use crate::ppu::pixel_fetcher::FetchState;
use crate::ppu::pixel_fetcher::Palette;
use crate::ppu::pixel_fetcher::Pixel;
use crate::ppu::pixel_fetcher::Pixelfetcher;
use crate::ppu::Sprite;
use crate::Mmu;

pub struct SpriteFetcher {
    fifo: MergePixelFifo,
    state: FetchState,
    tile_no: u8,
    data_start_add: u16,
    data_low: u8,
    data_high: u8,
    ready: bool,
    pub done: bool,
}

impl Pixelfetcher for SpriteFetcher {
    fn shift(&mut self) -> Option<Pixel> {
        self.fifo.shift()
    }
}

impl SpriteFetcher {
    pub fn new() -> SpriteFetcher {
        SpriteFetcher {
            fifo: MergePixelFifo::with_capacity(8),
            state: FetchState::FetchNo,
            tile_no: 0,
            data_start_add: 0,
            data_low: 0,
            data_high: 0,
            ready: false,
            done: true,
        }
    }

    pub fn tick(&mut self, mmu: &mut Mmu, sprite: &Sprite) {
        self.done = false;

        match (&self.state, self.ready) {
            (FetchState::FetchNo, true) => self.fetch_no(mmu, sprite),
            (FetchState::FetchDataLow, true) => self.fetch_data_low(mmu, sprite),
            (FetchState::FetchDataHigh, true) => self.fetch_data_high(mmu, sprite),
            (FetchState::Push, _) => {
                self.push(sprite);
                self.done = true;
            }
            _ => (),
        }
        self.ready = true;
    }

    fn fetch_no(&mut self, mmu: &Mmu, sprite: &Sprite) {
        self.tile_no = if mmu.io.lcd.get_sprite_size() == 16 {
            let line: u8 = if sprite.y_flip {
                15 - ((mmu.io.lcd.get_ly() + 16) - sprite.y_position)
            } else {
                (mmu.io.lcd.get_ly() + 16) - sprite.y_position
            };
            if line > 7 {
                sprite.tile_no | 0x01
            } else {
                sprite.tile_no & 0xFE
            }
        } else {
            sprite.tile_no
        };
        self.change_state(FetchState::FetchDataLow);
    }

    fn get_tile_data_start_address(&mut self, mmu: &Mmu, sprite: &Sprite) -> u16 {
        let line_index: u16 = if sprite.y_flip {
            (7u16.wrapping_sub(
                (mmu.io.lcd.get_ly() as u16)
                    .wrapping_sub(sprite.y_position.wrapping_sub(16) as u16),
            )) % 8
        } else {
            (mmu.io.lcd.get_ly() as u16).wrapping_sub(sprite.y_position.wrapping_sub(16) as u16) % 8
        };

        let offset: u16 = 2u16.wrapping_mul(line_index as u16);

        let base_address: u16 = 0x8000u16.wrapping_add(self.tile_no as u16 * 16);

        base_address + offset
    }

    fn fetch_data_low(&mut self, mmu: &Mmu, sprite: &Sprite) {
        let add: u16 = self.get_tile_data_start_address(mmu, sprite);
        self.data_low = mmu.read(add);
        self.change_state(FetchState::FetchDataHigh);
    }

    fn fetch_data_high(&mut self, mmu: &Mmu, sprite: &Sprite) {
        self.data_high = mmu.read(self.get_tile_data_start_address(mmu, sprite) + 1);
        self.change_state(FetchState::Push);
    }

    fn push(&mut self, sprite: &Sprite) {
        //TODO: maybe use standard fifo? refer to GBEDG sprite fetching (pushing)
        self.fifo.clear();
        for i in (0..8).rev() {
            if (sprite.x_position + (7-i)) >= 8 {
                let exp: u32 = if sprite.x_flip {
                    (7 - i).into()
                } else {
                    i.into()
                };
                let mask: u8 = u8::pow(2, exp);
                let msb: u8 = (self.data_high & mask) >> exp;
                let lsb: u8 = (self.data_low & mask) >> exp;
                let color: u8 = (msb << 1) | lsb;
                self.fifo.push(Pixel {
                    color: color,
                    palette: if !sprite.palette {
                        Palette::OBP0
                    } else {
                        Palette::OBP1
                    },
                    priority: false,
                    bg_priority: sprite.priority,
                });
            }
        }
        self.change_state(FetchState::FetchNo);
    }

    fn change_state(&mut self, state: FetchState) {
        self.state = state;
        self.ready = false;
    }

    pub fn reset(&mut self) {
        self.fifo.full_clear();
        self.state = FetchState::FetchNo;
        self.tile_no = 0;
        self.data_start_add = 0;
        self.data_low = 0;
        self.data_high = 0;
        self.ready = false;
        self.done = true;
    }
}
