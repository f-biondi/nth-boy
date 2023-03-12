#![allow(dead_code)]
#![allow(unused_variables)]

use crate::mmu::address_spaces::io::joypad::JoypadState;
use cpu::Cpu;
use mmu::Mmu;
use ppu::Ppu;
use ppu::LcdBuffer;
use std::error::Error;
use std::time::{Duration, Instant};

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
    pub fn new(path: &str) -> Result<Device, Box<dyn Error>> {
        Ok(Self {
            cpu: Cpu::new(),
            ppu: Ppu::new(),
            mmu: Mmu::from_file(path)?,
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
            //self.cpu.dump3(&mut self.mmu);
            let cycles: u8 = self.cpu.tick(&mut self.mmu);
            //self.mmu.dma_run();
            self.ppu.tick(&mut self.mmu, buffer, cycles);
            self.update_timers(cycles);
            for i in 0..cycles {
                total_cycles += 1;
            }
        }
    }

    fn update_timers(&mut self, cycles: u8) {
        let tima_enabled: bool = self.mmu.io.timers.get_tima_enabled();
        let tima_clock: u16 = self.mmu.io.timers.get_tima_clock();

        if self.tima_overflow {
            self.mmu.io.request_timer_interrupt();
            self.tima_overflow = false;
        }

        for i in 0..cycles {
            self.mmu.io.timers.inc_sysclk();
            let sysclk: u16 = self.mmu.io.timers.get_sysclk();
            if tima_enabled && (sysclk % tima_clock) == 0 {
                self.tima_overflow = self.mmu.io.timers.inc_tima();
            }
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        self.mmu.cart.save()
    }
}
