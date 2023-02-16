use crate::mmu::address_spaces::oam::Sprite;
use crate::Mmu;
use bg_fetcher::BgFetcher;
use std::collections::VecDeque;

mod bg_fetcher;

#[derive(PartialEq)]
enum PpuState {
    OAM_SEARCH,
    PIXEL_TRANSFER,
    H_BLANK,
    V_BLANK,
}

pub struct Ppu {
    state: PpuState,
    sprites: Vec<Sprite>,
    window_line_counter: u8,
    wy_equal_ly: bool,
    x_position: u8,
    discarded_pixels: u8,
    window_line: bool,
    stat_block: bool,
    state_stat_check: bool,
    bg_fetcher: BgFetcher,
    ticks: u16,
}

const PALETTE: &'static [u32] = &[0xffffff, 0xaaaaaa, 0x555555,  0x000000];

impl Ppu {
    pub fn new() -> Ppu {
        Self {
            state: PpuState::OAM_SEARCH,
            sprites: Vec::new(),
            wy_equal_ly: false,
            window_line_counter: 0,
            x_position: 0,
            discarded_pixels: 0,
            window_line: false,
            stat_block: false,
            state_stat_check: false,
            bg_fetcher: BgFetcher::new(),
            ticks: 0,
        }
    }

    fn handle_stat_ly_equal_lyc(&mut self, mmu: &mut Mmu) {
        if mmu.io.lcd.ly_equal_lyc_stat_enabled() && mmu.io.lcd.ly == mmu.io.lcd.lyc && !self.stat_block {
            mmu.io.request_lcd_stat_interrupt();
            mmu.io.lcd.set_coincidence_flag();
            self.stat_block = true;
        } else {
            mmu.io.lcd.unset_coincidence_flag();
        }
    }

    fn handle_stat_state(&mut self, mmu: &mut Mmu) {
        if self.state_stat_check {
            match &self.state {
                PpuState::OAM_SEARCH => mmu.io.lcd.set_oam_ppu_mode(),
                PpuState::PIXEL_TRANSFER => mmu.io.lcd.set_draw_ppu_mode(),
                PpuState::H_BLANK => mmu.io.lcd.set_hblank_ppu_mode(),
                PpuState::V_BLANK => mmu.io.lcd.set_vblank_ppu_mode(),
            }

            if !self.stat_block {
                if (mmu.io.lcd.oam_stat_enabled() && self.state == PpuState::OAM_SEARCH) ||
                (mmu.io.lcd.hblank_stat_enabled() && self.state == PpuState::H_BLANK) || 
                (mmu.io.lcd.vblank_stat_enabled() && self.state == PpuState::V_BLANK) 
                {
                    mmu.io.request_lcd_stat_interrupt();
                }
            }

            self.state_stat_check = false;
        }
    }

    pub fn tick(&mut self, mmu: &mut Mmu, buffer: &mut Vec<u32>, new_ticks: u8) {

        let mut ticks_todo = new_ticks;

        while ticks_todo > 0 {
            self.handle_stat_ly_equal_lyc(mmu);
            self.handle_stat_state(mmu);
            ticks_todo -= 1;
            self.ticks += 1;
            match &self.state {
                PpuState::OAM_SEARCH => self.oam_search(mmu),
                PpuState::PIXEL_TRANSFER => self.pixel_transfer(mmu, buffer),
                PpuState::H_BLANK => self.h_blank(mmu),
                PpuState::V_BLANK => self.v_blank(mmu),
            }
        }
    }

    fn oam_search(&mut self, mmu: &mut Mmu) {
        if self.ticks % 2 == 0 && self.sprites.len() < 10 {
            let sprite_id: u8 = if self.ticks > 0 {
                ((self.ticks as u8) / 2) - 1
            } else {
                0
            };
            let sprite: Sprite = mmu.oam.get_sprite(sprite_id);
            let sprite_height: u8 = mmu.io.lcd.get_sprite_size();
            let ly: u8 = mmu.io.lcd.ly + 16;

            if sprite.x_position > 0 && ly >= sprite.y_position && ly <= (sprite.y_position + sprite_height) {
                self.sprites.push(sprite);
            }
        }
        if self.ticks >= 80 {
            self.change_state(PpuState::PIXEL_TRANSFER, true);

            if mmu.io.lcd.ly == mmu.io.lcd.wy {
                self.wy_equal_ly = true;
            }
        }
    }

    fn handle_scanline_end(&mut self) {
        self.bg_fetcher = BgFetcher::new();
        self.x_position = 0;
        self.discarded_pixels = 0;
        self.stat_block = false;

        if self.window_line {
            self.window_line = false;
            self.window_line_counter += 1;
        }
    }

    fn pixel_transfer(&mut self, mmu: &mut Mmu, buffer: &mut Vec<u32>) {

        if mmu.io.lcd.is_window_enabled() && self.wy_equal_ly && self.x_position >= (mmu.io.lcd.wx - 7) && !self.window_line {
            self.window_line = true;
            self.bg_fetcher.switch_to_window_mode();
        }

        self.bg_fetcher.tick(mmu, self.window_line_counter);

        let pixel: Option<u8> = self.bg_fetcher.fifo.pop_front();

        if let Some(color_index) = pixel {
            if self.discarded_pixels >= (mmu.io.lcd.scx % 8) {
                let pixel_index: u32 = (self.x_position as u32) + ((mmu.io.lcd.ly as u32) * 160);
                
                let color: u32 = if mmu.io.lcd.is_bg_window_enabled() {
                    PALETTE[color_index as usize]
                } else {
                    0xffffff
                };

                buffer[pixel_index as usize] = color;  
                self.x_position += 1;
            } else {
                self.discarded_pixels += 1;
            }
        } 


        if self.x_position == 160 {
            self.handle_scanline_end();
            self.change_state(PpuState::H_BLANK, false);
        }
    }

    fn h_blank(&mut self, mmu: &mut Mmu) {
        if self.ticks == 456 {
            let mut new_state: PpuState;
            if mmu.io.lcd.ly < 143 {
                new_state = PpuState::OAM_SEARCH
            } else {
                new_state = PpuState::V_BLANK
            }
            mmu.io.lcd.ly += 1;
            self.change_state(new_state, true);
        }
    }

    fn v_blank(&mut self, mmu: &mut Mmu) {

        if self.ticks == 1 {
            mmu.io.request_vblank_interrupt();
        }

        if (self.ticks % 456) == 0 {
            mmu.io.lcd.ly += 1;
        }

        if mmu.io.lcd.ly == 153 {
            self.reset(mmu);
        }
    }

    fn change_state(&mut self, state: PpuState, reset_ticks: bool) {
        self.state = state;
        if reset_ticks {
            self.ticks = 0;
        }
        self.state_stat_check = true;
    }

    fn reset(&mut self, mmu: &mut Mmu) {
        self.change_state(PpuState::OAM_SEARCH, true);
        self.sprites = Vec::new();
        self.bg_fetcher = BgFetcher::new();
        self.wy_equal_ly = false;
        mmu.io.lcd.ly = 0;
        self.discarded_pixels = 0;
        self.window_line = false;
        self.window_line_counter = 0;
        self.x_position = 0;
        self.stat_block = false;
        self.state_stat_check = false;
    }
}
