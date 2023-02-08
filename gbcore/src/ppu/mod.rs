use crate::mmu::address_spaces::oam::Sprite;
use crate::Mmu;
use bg_fetcher::BgFetcher;
use std::collections::VecDeque;

mod bg_fetcher;

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
    bg_fetcher: BgFetcher,
    ticks: u16,
}

impl Ppu {
    pub fn new() -> Ppu {
        Self {
            state: PpuState::OAM_SEARCH,
            sprites: Vec::new(),
            window_line_counter: 0,
            bg_fetcher: BgFetcher::new(),
            ticks: 0,
        }
    }

    pub fn tick(&mut self, mmu: &mut Mmu, buffer: &mut Vec<u32>, new_ticks: u8) {
        let mut ticks_todo = new_ticks;

        while ticks_todo > 0 {
            ticks_todo -= 1;
            self.ticks += 1;
            match &self.state {
                OAM_SEARCH => self.oam_search(mmu),
                PIXEL_TRANSFER => self.pixel_transfer(mmu, buffer),
                H_BLANK => self.h_blank(mmu),
                V_BLANK => self.v_blank(),
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
            self.change_state(PpuState::PIXEL_TRANSFER);
        }
    }

    fn pixel_transfer(&mut self, mmu: &mut Mmu, buffer: &mut Vec<u32>) {

        self.bg_fetcher.tick(mmu, &mut self.x_counter, self.window_line_counter);

        let pixel: u8 = self.bg_fetcher.fifo.pop_front();

        if let Some(color) = pixel {
            let pixel_index: u32 = self.x_counter + (mmu.io.lc.ly * 160);
            if self.mmu.
        } 

    }

    fn h_blank(&mut self, mmu: &mut Mmu) {
        if self.ticks == 456 {
            let mut new_state: PpuState;
            if mmu.io.lcd.ly < 144 {
                mmu.io.lcd.ly += 1;
                new_state = PpuState::OAM_SEARCH
            } else {
                new_state = PpuState::V_BLANK
            }
            self.change_state(new_state);
        }
    }

    fn v_blank(&mut self) {
    }

    fn change_state(&mut self, state: PpuState) {
        self.state = state;
        self.ticks = 0;
    }

    fn reset(&mut self) {
        self.state = PpuState::OAM_SEARCH;
        self.sprites = Vec::new();
        self.bg_fetcher = BgFetcher::new();
        self.window_line_counter = 0;
        self.ticks = 0;
    }
}
