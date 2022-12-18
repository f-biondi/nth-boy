use crate::mmu::Mmu;
use registers::Register8;
use registers::Register16;
use registers::Registers;

mod registers;

use crate::mmu::address_spaces::Addressable;

enum PostOp {
    INC,
    DEC,
    NONE
}

pub struct Cpu {
    reg: Registers,
    mmu: Mmu,
    cycles: u128,
}

impl Cpu {
    pub fn new(mmu: Mmu) -> Self {
        Self {
            reg: Registers::new(),
            mmu: mmu,
            cycles: 0,
        }
    }

    pub fn cycle(&mut self) {
        let op: u8 = self.mmu.read(self.reg.pc);
        self.reg.pc += 1;
        self.decode(op);
    }

    fn decode(&mut self, op: u8) {
        println!("{:#01x}", op);
        match op {
            0x00 => self.nop(),
            _ => panic!("{:#x} opcode not implemented", op),
        }
    }

    fn consume_u8(&mut self) -> u8 {
        let r: u8 = self.mmu.read(self.reg.pc);
        self.reg.pc += 1;
        r
    }

    fn consume_u16(&mut self) -> u16 {
        let r: u16 = self.mmu.read_16(self.reg.pc);
        self.reg.pc += 2;
        r
    }

    fn handle_post_op(&mut self, reg: &Register16, op: &PostOp) {
        match op {
            PostOp::INC => {
                self.reg.set16(reg, self.reg.get16(reg)+1)
            }
            PostOp::DEC => {
                self.reg.set16(reg, self.reg.get16(reg)-1)
            }
            _ => {}
        }
    }

    fn ld_r8_r8(&mut self, dest: Register8, src: Register8) {
        let value: u8 = self.reg.get8(&src);
        self.reg.set8(&dest, value);
    }

    fn ld_r8_u8(&mut self, dest: Register8) {
        let value: u8 = self.consume_u8();
        self.reg.set8(&dest, value);
    }

    fn ld_r8_ir16(&mut self, dest: Register8, src: Register16, post_op: PostOp) {
        let add: u16 = self.reg.get16(&src);
        let value: u8 = self.mmu.read(add);
        self.reg.set8(&dest, value);
        self.handle_post_op(&src, &post_op);
    }

    fn ld_ir16_r8(&mut self, dest: Register16, src: Register8, post_op: PostOp) {
        let value: u8 = self.reg.get8(&src);
        let add: u16 = self.reg.get16(&dest);
        self.mmu.write(add, value);
        self.handle_post_op(&dest, &post_op);
    }

    fn ld_ir16_u8(&mut self, dest: Register16) {
        let value: u8 = self.consume_u8();
        let add: u16 = self.reg.get16(&dest);
        self.mmu.write(add, value);
    }

    fn ld_r8_iu16(&mut self, dest: Register8, src: Register16) {
        let add: u16 = self.consume_u16();
        let value: u8 = self.mmu.read(add);
        self.reg.set8(&dest, value);
    }

    fn ld_iu16_r8(&mut self, dest: Register16, src: Register8) {
        let value: u8 = self.reg.get8(&src);
        let add: u16 = self.consume_u16();
        self.mmu.write(add, value);
    }

    fn ldh_r8_ir8(&mut self, dest: Register8, src: Register8) {
        let add_low: u8 = self.reg.get8(&src);
        let add: u16 = 0xff00 & (add_low as u16);
        let value: u8 = self.mmu.read(add);
        self.reg.set8(&dest, value);
    }

    fn ldh_ir8_r8(&mut self, dest: Register8, src: Register8) {
        let value: u8 = self.reg.get8(&src);
        let add_low: u8 = self.reg.get8(&dest);
        let add: u16 = 0xff00 & (add_low as u16);
        self.mmu.write(add, value);
    }

    fn ldh_r8_iu8(&mut self, dest: Register8) {
        let add_low: u8 = self.consume_u8();
        let add: u16 = 0xff00 & (add_low as u16);
        let value: u8 = self.mmu.read(add);
        self.reg.set8(&dest, value);
    }

    fn ldh_iu8_r8(&mut self, dest: Register8, src: Register8) {
        let value : u8 = self.reg.get8(&src);
        let add_low: u8 = self.consume_u8();
        let add: u16 = 0xff00 & (add_low as u16);
        self.mmu.write(add, value);
    }

    fn nop(&mut self) {}
}
