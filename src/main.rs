#![allow(dead_code)]
#![allow(unused_variables)]
use cpu::Cpu;
use mmu::Mmu;
use std::io;
use std::io::Result;

mod cpu;
mod mmu;

fn main() {
    let mut cpu: Cpu = Cpu::new("/home/fbiondi/nth-boy/roms/tests/cpu_instrs/individual/9.gb");
    loop {
        cpu.cycle();
        //cpu.dump4();
        cpu.mmu.test();
    }
}
