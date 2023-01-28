#![allow(dead_code)]
#![allow(unused_variables)]

use crate::mmu::address_spaces::Addressable;
use cpu::Cpu;
use ppu::Ppu;
use mmu::Mmu;
use std::error::Error;
use std::thread;
use std::time::{Duration, Instant};

mod cpu;
mod ppu;
mod mmu;

const CYCLE_LIMIT: u32 = 70221;

pub struct Device {
    cpu: Cpu,
    ppu: Ppu,
    mmu: Mmu,
    tima_overflow: bool
}

impl Device {
    pub fn new(path: &str) -> Result<Device, Box<dyn Error>> {
        Ok(Self {
            cpu: Cpu::new(),
            ppu: Ppu::new(),
            mmu: Mmu::from_file(path)?,
            tima_overflow: false
        })
    }

    pub fn frame(&mut self, buffer: &mut Vec<u32>) {
        let mut total_cycles: u32 = 0;

        while total_cycles < CYCLE_LIMIT {
            let cycles: u8 = self.cpu.tick(&mut self.mmu);
            self.ppu.tick(&mut self.mmu, buffer, cycles);
            self.update_timers(cycles);
            self.mmu.test();
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

       for i in 0..cycles {
           self.mmu.io.timers.inc_sysclk();
           let sysclk: u16 = self.mmu.io.timers.get_sysclk();
           if tima_enabled && (sysclk % tima_clock) == 0  {
               self.tima_overflow = self.mmu.io.timers.inc_tima();
           }
       }
    }

}
