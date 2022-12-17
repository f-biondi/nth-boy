use crate::mmu::Mmu;
use registers::Registers;

mod registers;

use crate::mmu::address_spaces::Addressable;

pub struct Cpu {
    registers: Registers,
    mmu: Mmu,
    cycles: u128,
}

impl Cpu {
    pub fn new(mmu: Mmu) -> Self {
        Self {
            registers: Registers::new(),
            mmu: mmu,
            cycles: 0
        }
    }

    pub fn cycle(&mut self) {
        let op: u8 = self.mem_read(self.registers.pc);
        self.registers.pc += 1;
        self.decode(op);
    }

    fn inc_cycles(&mut self, n: u8) {
        self.cycles += 4*(n as u128);
    }

    // +1 cycle
    fn mem_read(&mut self, location: u16) -> u8 {
        self.inc_cycles(1);
        return self.mmu.read(location);
    }

    // +2 cycles
    fn mem_read_u16(&mut self, location: u16) -> u16 {
        let low : u8 = self.mem_read(location);
        let high : u8 = self.mem_read(location+1);
        return ((high as u16) << 8) + (low as u16);
    }

    fn decode(&mut self, op: u8) {
        println!("{:#01x}", op);
        match op {
            0x00 => self.nop(),
            _ => panic!("{:#x} opcode not implemented", op),
        }
    }

    fn nop(&mut self) {}
}
