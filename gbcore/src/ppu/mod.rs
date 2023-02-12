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
    x_position: u8,
    discarded: u8,
    bg_fetcher: BgFetcher,
    ticks: u16,
}

const PALETTE: &'static [u32] = &[0xffffff, 0xaaaaaa, 0x555555,  0x000000];

impl Ppu {
    pub fn new() -> Ppu {
        Self {
            state: PpuState::OAM_SEARCH,
            sprites: Vec::new(),
            window_line_counter: 0,
            x_position: 0,
            discarded: 0,
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
            self.change_state(PpuState::PIXEL_TRANSFER);
        }
    }

    fn pixel_transfer(&mut self, mmu: &mut Mmu, buffer: &mut Vec<u32>) {

        self.bg_fetcher.tick(mmu, self.window_line_counter);

        let pixel: Option<u8> = self.bg_fetcher.fifo.pop_front();

        if let Some(color) = pixel {
            if self.discarded >= (mmu.io.lcd.scx % 8) {
                let pixel_index: u32 = (self.x_position as u32) + ((mmu.io.lcd.ly as u32) * 160);
                buffer[pixel_index as usize] = PALETTE[color as usize];  
                self.x_position += 1;
            } else {
                self.discarded += 1;
            }
        } 


        if self.x_position == 160 {
            self.bg_fetcher = BgFetcher::new();
            self.x_position = 0;
            self.discarded = 0;
            self.change_state(PpuState::H_BLANK);
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
            self.change_state(new_state);
        }
    }

    fn v_blank(&mut self, mmu: &mut Mmu) {
        if (self.ticks % 456) == 0 {
            mmu.io.lcd.ly += 1;
        }

        if mmu.io.lcd.ly == 153 {
            self.reset(mmu);
        }
    }

    fn change_state(&mut self, state: PpuState) {
        self.state = state;
        self.ticks = 0;
    }

    fn reset(&mut self, mmu: &mut Mmu) {
        self.state = PpuState::OAM_SEARCH;
        self.sprites = Vec::new();
        self.bg_fetcher = BgFetcher::new();
        mmu.io.lcd.ly = 0;
        self.discarded = 0;
        self.window_line_counter = 0;
        self.x_position = 0;
        self.ticks = 0;
    }
}
