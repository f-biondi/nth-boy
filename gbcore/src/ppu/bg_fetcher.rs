use crate::Mmu;
use crate::mmu::address_spaces::Addressable;
use std::collections::VecDeque;

enum FetchState {
    FETCH_NO,
    FETCH_DATA_LOW,
    FETCH_DATA_HIGH,
    PUSH,
}

pub struct BgFetcher {
    pub fifo: VecDeque<u8>,
    state: FetchState,
    tile_no: u8,
    data_low: u8,
    data_high: u8,
    pub x_counter: u8,
     window: bool,
    ready: bool
}

impl BgFetcher {
    pub fn new() -> BgFetcher {
        BgFetcher {
            fifo: VecDeque::new(),
            state: FetchState::FETCH_NO,
            tile_no: 0,
            data_low: 0,
            data_high: 0,
            x_counter: 0,
            window: false,
            ready: false,
        }
    }

    pub fn tick(&mut self, mmu: &mut Mmu, window_line_counter: u8) {
        if self.ready {
            match &self.state {
                FetchState::FETCH_NO => self.fetch_no(mmu, window_line_counter),
                FetchState::FETCH_DATA_LOW => self.fetch_data_low(mmu, window_line_counter),
                FetchState::FETCH_DATA_HIGH => self.fetch_data_high(mmu, window_line_counter),
                FetchState::PUSH => self.push(),
            }
        }
        self.ready = true;
    }

    pub fn switch_to_window_mode(&mut self) {
        self.reset();
        self.window = true;
    }

    fn fetch_no(&mut self, mmu: &Mmu, window_line_counter: u8) {
        let mut offset: u16 = 0;

        if !self.window {
            offset += ((self.x_counter + (mmu.io.lcd.scx / 8)) & 0x1f) as u16;
        } else {
            offset += self.x_counter as u16;
        }

        offset += if self.window {
            32u16.wrapping_mul((window_line_counter) as u16 / 8)
        } else {
            // cells in line * line number 
            32u16.wrapping_mul((((mmu.io.lcd.ly as u16).wrapping_add(mmu.io.lcd.scy as u16)) & 0xFF) / 8)
        };

        offset &= 0x3ff;

        let tile_no_add: u16 = if self.window {
            mmu.io.lcd.get_window_tile_map().wrapping_add(offset)
        } else {
            mmu.io.lcd.get_bg_tile_map().wrapping_add(offset)
        };

        self.tile_no = mmu.read(tile_no_add);
        self.change_state(FetchState::FETCH_DATA_LOW); 
    }

    fn get_tile_data_start_address(&mut self, mmu: &Mmu, window_line_counter: u8) -> u16 {
        let offset: u16 = if self.window {
            (2u8.wrapping_mul(window_line_counter % 8)).into()
        } else {
            (2u16.wrapping_mul(((mmu.io.lcd.ly as u16).wrapping_add(mmu.io.lcd.scy as u16)) % 8))
        };

        let base_address: u16 = if mmu.io.lcd.get_tile_data() == 0x8000 {
            0x8000u16.wrapping_add(self.tile_no as u16 * 16)
        } else {
            let pointer: i32 = 0x9000;
            let signed_tile_no: i8 = self.tile_no as i8;
            let tile_offset: i32 = (signed_tile_no as i32).wrapping_mul(16);
            (pointer + tile_offset) as u16
        };

        base_address + offset
    }

    fn fetch_data_low(&mut self, mmu: &Mmu, window_line_counter: u8) {
        let add: u16 = self.get_tile_data_start_address(mmu, window_line_counter);
        self.data_low = mmu.read(add);
        self.data_high = mmu.read(add+1);
        self.change_state(FetchState::FETCH_DATA_HIGH); 
    }

    //TODO: needs delay?
    fn fetch_data_high(&mut self, mmu: &Mmu, window_line_counter: u8) {
        //self.data_high = mmu.read((self.data_low + 1);
        self.change_state(FetchState::PUSH); 
    }

    fn push(&mut self) {
        if self.fifo.len() == 0 {
            for i in (0..8).rev() {
                let mask: u8 = u8::pow(2, i);
                let msb: u8 = (self.data_high & mask) >> i;
                let lsb: u8 = (self.data_low & mask) >> i;
                let color: u8 = (msb << 1) | lsb;
                self.fifo.push_back(color);
            }
            self.x_counter += 1;
            self.change_state(FetchState::FETCH_NO); 
        } else {
            self.ready = false;
        }
    }

    fn change_state(&mut self, state: FetchState) {
        self.state = state;
        self.ready = false;
    }

    fn reset(&mut self) {
        self.fifo = VecDeque::new();
        self.state = FetchState::FETCH_NO;
        self.tile_no = 0;
        self.data_low = 0;
        self.data_high = 0;
        self.x_counter = 0;
        self.window = false;
        self.ready = false;
    }
}
