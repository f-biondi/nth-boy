use address_spaces::adressable_memory::AdressableMemory;

enum PpuState {
    OAM_SEARCH,
    PIXEL_TRANSFER,
    H_BLANK,
    V_BLANK,
}

pub struct Ppu {
    pub vram: AdressableMemory
    state: PpuState,
    ticks: u8,
}

impl Ppu {
    pub fn new() -> Result<Ppu, Box<dyn Error>> {
        Self {
            vram: AdressableMemory::new(0x8000, 0x9FFF)?,
            state: PpuState::OAM_SEARCH,
            ticks: 0,
        }
    }

    pub fn tick(&mut self, new_ticks: u8) {
        while new_ticks > 0 {
            new_ticks -= 1;
            self.ticks += 1;
            match self.state {
                OAM_SEARCH => self.oam_search(),
                PIXEL_TRANSFER => self.pixel_transfer(),
                H_BLANK => self.h_blank(),
                V_BLANK => self.v_blank(),
            }
        }
    }

    fn oam_search(&mut self) {
        if self.ticks == 20 {
            self.change_state(PpuState::PIXEL_TRANSFER);
        }
    }

    fn pixel_transfer(&mut self) {
        if self.ticks == 43 {
            self.change_state(PpuState::H_BLANK);
        }
    }

    fn change_state(&mut self, state: PpuState) {
        self.state = state;
        self.ticks = 0;
    }
}
