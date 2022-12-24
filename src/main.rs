#![allow(dead_code)]
#![allow(unused_variables)]
use cpu::Cpu;
use crossterm::{cursor, QueueableCommand};
use mmu::Mmu;
use std::io;
use std::io::prelude::*;
use std::io::Result;
use std::io::{stdout, Write};
use std::{thread, time};

mod cpu;
mod mmu;

fn main() {
    let mmu: Mmu = Mmu::from_file("roms/test_ld.gb").unwrap();
    let mut cpu: Cpu = Cpu::new(mmu);
    let mut stdin = io::stdin();
    let mut stdout = stdout();
    let mut auto_inc = true;
    let mut buffer: Vec<u8> = vec![0, 0];
    cpu.dump2();
    loop {
        //print!("\x1B[2J\x1B[1;1H");
        if cpu.getpc() == 0xc6d6 {
            //auto_inc = false;
        }
        cpu.cycle();
        cpu.dump2();
        if !auto_inc {
            stdin.read(&mut buffer).unwrap();
            if buffer[0] == 98 {
                auto_inc = true;
            }
        }
    }
}
