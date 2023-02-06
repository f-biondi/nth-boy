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
    fifo: VecDeque<u8>,
    state: FetchState,
    tile_no: u8,
    data_low: u8,
    data_high: u8,
    window: bool,
    ticks: u8
}

impl BgFetcher {
    pub fn new() -> BgFetcher {
        BgFetcher {
            fifo: VecDeque::new(),
            state: FetchState::FETCH_NO,
            tile_no: 0,
            data_low: 0,
            data_high: 0,
            window: false,
            ticks: 0,
        }
    }

    pub fn tick(&mut self, mmu: &mut Mmu, x_counter: &mut u8, window_line_counter: u8) {
        match &self.state {
            FETCH_NO => self.fetch_no(mmu, *x_counter, window_line_counter),
            FETCH_DATA_LOW => self.fetch_data_low(mmu, window_line_counter),
            FETCH_DATA_HIGH => self.fetch_data_high(mmu, window_line_counter),
            PUSH => self.push(x_counter),
        }
    }

    fn fetch_no(&mut self, mmu: &Mmu, x_counter: u8, window_line_counter: u8) {
        if self.ticks == 2 {
           self.change_state(FetchState::FETCH_DATA_LOW); 
           return;
        }

        let mut offset: u16 = x_counter as u16;

        if !self.window {
            offset += ((mmu.io.lcd.scx / 8) & 0x1f) as u16;
        }

        offset += if self.window {
            32 * ((window_line_counter) as u16 / 8)
        } else {
            32 * (((mmu.io.lcd.ly + mmu.io.lcd.scy) as u16 & 0xFF) / 8)
        };

        offset &= 0x3ff;

        let tile_no_add: u16 = if self.window {
            mmu.io.lcd.get_window_tile_map() + offset
        } else {
            mmu.io.lcd.get_bg_tile_map() + offset
        };

        self.tile_no = mmu.read(tile_no_add);
    }

    fn get_tile_data_start_address(&mut self, mmu: &Mmu, window_line_counter: u8) -> u16 {
        let offset: u16 = if self.window {
            (2 * (window_line_counter % 8)).into()
        } else {
            (2 * ((mmu.io.lcd.ly + mmu.io.lcd.scy) % 8)).into()
        };

        let base_address: u16 = if mmu.io.lcd.get_tile_data() == 0x8000 {
            0x8000u16 + (self.tile_no * 16) as u16
        } else {
            0x9000u16.wrapping_add(((self.tile_no as i8) * 16) as u16)
        };

        base_address + offset
    }

    fn fetch_data_low(&mut self, mmu: &Mmu, window_line_counter: u8) {
        if self.ticks == 2 {
           self.change_state(FetchState::FETCH_DATA_HIGH); 
           return;
        }
        self.data_low = mmu.read(self.get_tile_data_start_address(mmu, window_line_counter));
    }
    
    //TODO: needs delay?
    fn fetch_data_high(&mut self, mmu: &Mmu, window_line_counter: u8) {
        if self.ticks == 2 {
           self.change_state(FetchState::PUSH); 
           return;
        }
        self.data_high = mmu.read(self.get_tile_data_start_address(mmu, window_line_counter) + 1);
    }

    fn push(&mut self, x_counter: &mut u8) {
        if self.fifo.len() == 0 {
            for i in 0..8 {
                let mask: u8 = u8::pow(2, i);
                let msb: u8 = (self.data_high & mask) >> (mask - 1);
                let lsb: u8 = (self.data_low & mask) >> (mask - 1);
                let color: u8 = (msb << 1) | lsb;
                self.fifo.push_back(color);
            }
            *x_counter += 1;
            self.change_state(FetchState::FETCH_NO); 
        }
    }

    fn change_state(&mut self, state: FetchState) {
        self.state = state;
        self.ticks = 0;
    }
}
