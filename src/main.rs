#![allow(dead_code)]
#![allow(unused_variables)]
use cpu::Cpu;
use mmu::Mmu;
use std::io::Result;

mod cpu;
mod mmu;

fn main() -> Result<()> {
    let mmu: Mmu = Mmu::from_file("roms/test.gb")?;
    let mut cpu: Cpu = Cpu::new(mmu);

    cpu.cycle();
    cpu.cycle();
    cpu.cycle();

    Ok(())
}
