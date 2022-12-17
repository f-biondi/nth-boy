//use std::env;
#![allow(dead_code)]
#![allow(unused_variables)]
use std::io::Result;
use mmu::Mmu;
use cpu::Cpu;

mod mmu;
mod cpu;

fn main() -> Result<()> {

    let mmu: Mmu = Mmu::from_file("roms/test.gb")?;
    let mut cpu: Cpu = Cpu::new(mmu);

    cpu.cycle();
    cpu.cycle();
    cpu.cycle();

    /*let mut a : u8;
    let mut b : u16 = 0xffaa;
    a=b as u8;
    println!("{:#x}", a);*/

    Ok(())
}
