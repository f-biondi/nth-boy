use crate::mmu::address_spaces::io::joypad::JoypadState;
use cpu::Cpu;
use mmu::Mmu;
use ppu::LcdBuffer;
use ppu::Ppu;
use std::error::Error;

mod cpu;
pub mod mmu;
pub mod ppu;

const CYCLE_LIMIT: u32 = 70224;

pub struct Device {
    cpu: Cpu,
    ppu: Ppu,
    mmu: Mmu,
    tima_overflow: bool,
}

impl Device {
    pub fn new(
        rom: Vec<u8>,
        ram: Option<Vec<u8>>,
        rtc: Option<Vec<u8>>,
    ) -> Result<Device, Box<dyn Error>> {
        Ok(Self {
            cpu: Cpu::new(),
            ppu: Ppu::new(),
            mmu: Mmu::new(rom, ram, rtc)?,
            tima_overflow: false,
        })
    }

    pub fn frame(&mut self, buffer: &mut LcdBuffer, joypad_state: JoypadState) {
        let mut total_cycles: u32 = 0;

        self.mmu.io.joypad.set_state(joypad_state);

        while total_cycles < CYCLE_LIMIT {
            if self.mmu.io.joypad.purge_interrupt() {
                self.mmu.io.request_joypad_interrupt();
            }
            let cycles: u8 = self.cpu.tick(&mut self.mmu);
            self.ppu.tick(&mut self.mmu, buffer, cycles);
            self.update_timers(cycles);
            total_cycles += cycles as u32;
        }
    }

    fn update_timers(&mut self, cycles: u8) {
        let tima_enabled: bool = self.mmu.io.timers.get_tima_enabled();
        let tima_clock: u16 = self.mmu.io.timers.get_tima_clock();

        if self.tima_overflow {
            self.mmu.io.request_timer_interrupt();
            self.tima_overflow = false;
        }

        for _ in 0..cycles {
            self.mmu.io.timers.inc_sysclk();
            let sysclk: u16 = self.mmu.io.timers.get_sysclk();
            if tima_enabled && (sysclk % tima_clock) == 0 {
                self.tima_overflow = self.mmu.io.timers.inc_tima();
            }
        }
    }

    pub fn dump_ram(&self) -> Option<Vec<u8>> {
        self.mmu.cart.dump_ram()
    }

    pub fn dump_rtc(&self) -> Option<Vec<u8>> {
        self.mmu.cart.dump_rtc()
    }

    pub fn update_rtc_now(&mut self, elapsed_secs: u64) {
        self.mmu.cart.update_rtc_now(elapsed_secs);
    }
}
