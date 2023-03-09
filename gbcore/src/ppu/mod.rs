use crate::mmu::address_spaces::oam::Sprite;
use crate::ppu::pixel_fetcher::Pixel;
use crate::ppu::pixel_fetcher::Pixelfetcher;
use crate::Mmu;
use pixel_fetcher::bg_fetcher::BgFetcher;
use pixel_fetcher::sprite_fetcher::SpriteFetcher;
use pixel_fetcher::Palette;
use std::collections::VecDeque;
use crate::mmu::address_spaces::Addressable;

mod pixel_fetcher;

#[derive(PartialEq, Debug)]
enum PpuState {
    OAM_SEARCH,
    PIXEL_TRANSFER,
    H_BLANK,
    V_BLANK,
    QUIRK,
}

pub struct Ppu {
    state: PpuState,
    sprites: Vec<Sprite>,
    current_sprite: Option<Sprite>,
    window_line_counter: u8,
    wy_equal_ly: bool,
    x_position: u8,
    discarded_pixels: u8,
    window_line: bool,
    old_stat: bool,
    bg_fetcher: BgFetcher,
    sprite_fetcher: SpriteFetcher,
    needs_reset: bool,
    ticks: u16,
}

const PALETTE: &'static [u32] = &[0xffffff, 0xaaaaaa, 0x555555, 0x000000];

impl Ppu {
    pub fn new() -> Ppu {
        Self {
            state: PpuState::OAM_SEARCH,
            sprites: Vec::new(),
            current_sprite: None,
            wy_equal_ly: false,
            window_line_counter: 0,
            x_position: 0,
            discarded_pixels: 0,
            window_line: false,
            old_stat: false,
            bg_fetcher: BgFetcher::new(),
            sprite_fetcher: SpriteFetcher::new(),
            needs_reset: false,
            ticks: 0,
        }
    }

    fn handle_stat(&mut self, mmu: &mut Mmu) {
        let stat: bool = (
            (
                mmu.io.lcd.ly_equal_lyc_stat_enabled() && 
                mmu.io.lcd.get_ly() == mmu.io.lcd.get_lyc()
            ) ||
            (
                mmu.io.lcd.oam_stat_enabled() &&
                &self.state == &PpuState::OAM_SEARCH
            ) ||
            (
                mmu.io.lcd.vblank_stat_enabled() &&
                &self.state == &PpuState::V_BLANK
            ) ||
            (
                mmu.io.lcd.hblank_stat_enabled() &&
                &self.state == &PpuState::H_BLANK
            ));
        if stat && !self.old_stat {
            mmu.io.request_lcd_stat_interrupt();
        }
        self.old_stat = stat;
    }

    pub fn tick(&mut self, mmu: &mut Mmu, buffer: &mut Vec<u32>, new_ticks: u8) {
        
        if !mmu.io.lcd.is_display_enabled() {
            self.needs_reset = true;
        } else if self.needs_reset {
            self.reset(mmu);
        }
        
        let mut ticks_todo = new_ticks;

        while ticks_todo > 0 {
            self.handle_stat(mmu);
            ticks_todo -= 1;
            match &self.state {
                PpuState::OAM_SEARCH => self.oam_search(mmu),
                PpuState::PIXEL_TRANSFER => self.pixel_transfer(mmu, buffer),
                PpuState::H_BLANK => self.h_blank(mmu),
                PpuState::V_BLANK => self.v_blank(mmu),
                PpuState::QUIRK => self.quirk(mmu),
            }
            self.ticks += 1;
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
            let ly: u8 = mmu.io.lcd.get_ly() + 16;

            if sprite.x_position > 0
                && ly >= sprite.y_position
                && ly < (sprite.y_position + sprite_height)
            {
                self.sprites.push(sprite);
            }

        }
        
        if self.ticks == 80 {
            self.change_state(mmu, PpuState::PIXEL_TRANSFER, true);

            if mmu.io.lcd.get_ly() == mmu.io.lcd.wy {
                self.wy_equal_ly = true;
            }
        }
    }

    fn handle_scanline_end(&mut self) {
        if self.window_line {
            self.window_line_counter += 1;
        }
    }

    fn get_current_sprite(&mut self) -> Option<Sprite> {
        let target_pos = self.x_position.wrapping_add(8);
        let mut res: Option<Sprite> = None;

        for i in 0..self.sprites.len() {
            if res == None && self.sprites[i].x_position <= target_pos {
                res = Some(self.sprites[i]);
                self.sprites.remove(i);
            }
        }
        res
    }

    fn pixel_transfer(&mut self, mmu: &mut Mmu, buffer: &mut Vec<u32>) {
        if self.sprite_fetcher.done {
            if let Some(sprite) = self.get_current_sprite() {
                self.bg_fetcher.restart();
                self.current_sprite = Some(sprite);
                self.sprite_fetcher.done = false;
            }
        }

        if self.sprite_fetcher.done {
            if mmu.io.lcd.is_window_enabled()
                && self.wy_equal_ly
                && (self.x_position as i16) >= ((mmu.io.lcd.wx as i16) - 7)
                && !self.window_line
            {
                self.window_line = true;
                self.bg_fetcher.switch_to_window_mode();
            }
            self.bg_fetcher.tick(mmu, self.window_line_counter);
        } else if let Some(sprite) = self.current_sprite {
            self.sprite_fetcher.tick(mmu, &sprite);
        }

        if self.sprite_fetcher.done {
            let pixel: Option<Pixel> = self.bg_fetcher.shift();

            if let Some(bg_pixel) = pixel {
                if self.window_line || (self.discarded_pixels >= (mmu.io.lcd.scx % 8)) {
                    let sprite_pixel: Option<Pixel> = self.sprite_fetcher.shift();
                    let pixel_index: u32 = (self.x_position as u32) + ((mmu.io.lcd.get_ly() as u32) * 160);
                    buffer[pixel_index as usize] = self.merge_pixels(mmu, bg_pixel, sprite_pixel);
                    self.x_position += 1;
                } else {
                    self.discarded_pixels += 1;
                }
            }
        }

        if self.x_position == 160 {
            self.handle_scanline_end();
            self.change_state(mmu, PpuState::H_BLANK, false);
        }
    }

    fn merge_pixels(
        &mut self,
        mmu: &Mmu,
        bg_pixel: Pixel,
        option_sprite_pixel: Option<Pixel>,
    ) -> u32 {
        let bg_color: u32 = if mmu.io.lcd.is_bg_window_enabled() {
            PALETTE[mmu.io.lcd.get_bgp_index(bg_pixel.color) as usize]
        } else {
            0xffffff
        };

        if let Some(sprite_pixel) = option_sprite_pixel {
            if !mmu.io.lcd.is_sprite_enabled()
                || sprite_pixel.color == 0
                || (sprite_pixel.bg_priority && bg_pixel.color > 0)
            {
                bg_color
            } else {
                match sprite_pixel.palette {
                    Palette::OBP0 => {
                        PALETTE[mmu.io.lcd.get_obp0_index(sprite_pixel.color) as usize]
                    }
                    Palette::OBP1 => {
                        PALETTE[mmu.io.lcd.get_obp1_index(sprite_pixel.color) as usize]
                    }
                    _ => panic!("Invalid palette for sprite"),
                }
            }
        } else {
            bg_color
        }
    }

    fn h_blank(&mut self, mmu: &mut Mmu) {
        if self.ticks == 456 {
            let mut new_state: PpuState;
            if mmu.io.lcd.get_ly() < 143 {
                new_state = PpuState::OAM_SEARCH
            } else {
                new_state = PpuState::V_BLANK
            }
            mmu.io.lcd.inc_ly(1);
            self.change_state(mmu, new_state, true);
        }
    }

    fn v_blank(&mut self, mmu: &mut Mmu) {
        if self.ticks == 1 {
            mmu.io.request_vblank_interrupt();
        }

        if (self.ticks % 456) == 0 {
            mmu.io.lcd.inc_ly(1);
        }

        if mmu.io.lcd.get_ly() == 153 {
            self.change_state(mmu, PpuState::QUIRK, true);
        }
    }

    fn quirk(&mut self, mmu: &mut Mmu) {
        if self.ticks == 4 {
            self.reset(mmu);
        }
    }

    fn change_state(&mut self, mmu: &mut Mmu, state: PpuState, reset_ticks: bool) {
        self.state = state;
        if reset_ticks {
            self.ticks = 0;
        }
        match &self.state {
            PpuState::OAM_SEARCH => {self.line_reset(); mmu.io.lcd.set_oam_ppu_mode()},
            PpuState::PIXEL_TRANSFER => mmu.io.lcd.set_draw_ppu_mode(),
            PpuState::H_BLANK => mmu.io.lcd.set_hblank_ppu_mode(),
            PpuState::V_BLANK => mmu.io.lcd.set_vblank_ppu_mode(),
            _ => {}
        }
    }
    
    fn line_reset(&mut self) {
        self.sprites.clear();
        self.current_sprite = None;
        self.bg_fetcher.reset();
        self.sprite_fetcher.reset();
        self.discarded_pixels = 0;
        self.window_line = false;
        self.x_position = 0;
    }

    fn reset(&mut self, mmu: &mut Mmu) {
        self.change_state(mmu, PpuState::OAM_SEARCH, true);
        self.line_reset();  
        self.wy_equal_ly = false;
        mmu.io.lcd.set_ly(0);
        self.window_line_counter = 0;
        self.needs_reset = false;
    }
}
