use crate::mmu::{address_spaces::Addressable, Mmu};
use registers::{Flag, Register16, Register8, Registers};

mod registers;

enum PostOp {
    INC,
    DEC,
    NONE,
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

    fn stack_push(&mut self, value: u8) {
        self.reg.sp -= 1;
        self.mmu.write(self.reg.sp, value);
    }

    fn stack_pop(&mut self) -> u8 {
        let value: u8 = self.mmu.read(self.reg.sp);
        self.reg.sp += 1;
        value
    }

    //
    // load 8-bit
    //

    fn handle_post_op(&mut self, reg: &Register16, op: &PostOp) {
        match op {
            PostOp::INC => self.reg.set16(reg, self.reg.get16(reg) + 1),
            PostOp::DEC => self.reg.set16(reg, self.reg.get16(reg) - 1),
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

    fn ld_r8_iu16(&mut self, dest: Register8) {
        let add: u16 = self.consume_u16();
        let value: u8 = self.mmu.read(add);
        self.reg.set8(&dest, value);
    }

    fn ld_iu16_r8(&mut self, src: Register8) {
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

    fn ldh_iu8_r8(&mut self, src: Register8) {
        let value: u8 = self.reg.get8(&src);
        let add_low: u8 = self.consume_u8();
        let add: u16 = 0xff00 & (add_low as u16);
        self.mmu.write(add, value);
    }

    //
    // load 16-bit
    //

    fn ld_r16_u16(&mut self, dest: Register16) {
        let value: u16 = self.consume_u16();
        self.reg.set16(&dest, value);
    }

    fn ld_iu16_r16(&mut self, src: Register16) {
        let value: u16 = self.reg.get16(&src);
        let value_low: u8 = (value & 0x00ff) as u8;
        let value_high: u8 = ((value & 0xff00) >> 8) as u8;
        let add: u16 = self.consume_u16();
        self.mmu.write(add, value_low);
        self.mmu.write(add + 1, value_high);
    }

    fn ld_r16_r16(&mut self, dest: Register16, src: Register16) {
        let value: u16 = self.reg.get16(&src);
        self.reg.set16(&dest, value);
    }

    fn push_r16(&mut self, src: Register16) {
        let value: u16 = self.reg.get16(&src);
        let value_high: u8 = ((value & 0xff00) >> 8) as u8;
        let value_low: u8 = (value & 0x00ff) as u8;
        self.stack_push(value_high);
        self.stack_push(value_low);
    }

    fn pop_r16(&mut self, dest: Register16) {
        let value_low: u8 = self.stack_pop();
        let value_high: u8 = self.stack_pop();
        let value: u16 = ((value_high as u16) << 8) + (value_low as u16);
        self.reg.set16(&dest, value);
    }

    //
    // alu 8-bit
    //

    fn sum8_flags(&mut self, op1: u8, op2: u8, c: bool, h: bool) -> u8 {
        let res: u8 = op1.wrapping_add(op2);

        if h {
            if ((op1 & 0b00010000) & (op2 & 0b00010000)) == 1 {
                self.reg.setf(&Flag::H);
            } else {
                self.reg.unsetf(&Flag::H);
            }
        }

        if c {
            if ((op1 & 0b10000000) & (op2 & 0b10000000)) == 1 {
                self.reg.setf(&Flag::C);
            } else {
                self.reg.unsetf(&Flag::C);
            }
        }

        if res == 0 {
            self.reg.setf(&Flag::Z);
        } else {
            self.reg.unsetf(&Flag::Z);
        }

        self.reg.unsetf(&Flag::N);
        res
    }

    fn sub8_flags(&mut self, op1: u8, op2: u8, c: bool, h: bool) -> u8 {
        let res: u8 = op1.wrapping_sub(op2);

        if h {
            if (op1 & 0x0f) < (op2 & 0x0f) {
                self.reg.setf(&Flag::H);
            } else {
                self.reg.unsetf(&Flag::H);
            }
        }

        if c {
            if op1 < op2 {
                self.reg.setf(&Flag::C);
            } else {
                self.reg.unsetf(&Flag::C);
            }
        }

        if res == 0 {
            self.reg.setf(&Flag::Z);
        } else {
            self.reg.unsetf(&Flag::Z);
        }

        self.reg.setf(&Flag::N);
        res
    }

    fn add_r8(&mut self, reg: Register8) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let value: u8 = self.reg.get8(&reg);
        let res: u8 = self.sum8_flags(accumulator, value, true, true);
        self.reg.set8(&Register8::A, res);
    }

    fn add_ir16(&mut self, reg: Register16) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let add: u16 = self.reg.get16(&reg);
        let value: u8 = self.mmu.read(add);
        let res: u8 = self.sum8_flags(accumulator, value, true, true);
        self.reg.set8(&Register8::A, res);
    }

    fn add_u8(&mut self, reg: Register8) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let value: u8 = self.consume_u8();
        let res: u8 = self.sum8_flags(accumulator, value, true, true);
        self.reg.set8(&Register8::A, res);
    }

    fn adc_r8(&mut self, reg: Register8) {
        let reg_value: u8 = self.reg.get8(&reg);

        let accumulator: u8 = self.reg.get8(&Register8::A);
        let c_value: u8 = self.reg.getf(&Flag::C);
        let accumulator_carry: u8 = accumulator.wrapping_add(c_value);

        let res: u8 = self.sum8_flags(accumulator_carry, reg_value, true, true);
        self.reg.set8(&Register8::A, res);
    }

    fn adc_ir16(&mut self, reg: Register16) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let c_value: u8 = self.reg.getf(&Flag::C);
        let accumulator_carry: u8 = accumulator.wrapping_add(c_value);

        let add: u16 = self.reg.get16(&reg);
        let value: u8 = self.mmu.read(add);

        let res: u8 = self.sum8_flags(accumulator_carry, value, true, true);
        self.reg.set8(&Register8::A, res);
    }

    fn adc_u8(&mut self, reg: Register8) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let c_value: u8 = self.reg.getf(&Flag::C);
        let accumulator_carry: u8 = accumulator.wrapping_add(c_value);

        let value: u8 = self.consume_u8();

        let res: u8 = self.sum8_flags(accumulator_carry, value, true, true);
        self.reg.set8(&Register8::A, res);
    }

    fn sub_r8(&mut self, reg: Register8) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let value: u8 = self.reg.get8(&reg);
        let res: u8 = self.sub8_flags(accumulator, value, true, true);
        self.reg.set8(&Register8::A, res);
    }

    fn sub_ir16(&mut self, reg: Register16) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let add: u16 = self.reg.get16(&reg);
        let value: u8 = self.mmu.read(add);
        let res: u8 = self.sub8_flags(accumulator, value, true, true);
        self.reg.set8(&Register8::A, res);
    }

    fn sub_u8(&mut self, reg: Register8) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let value: u8 = self.consume_u8();
        let res: u8 = self.sub8_flags(accumulator, value, true, true);
        self.reg.set8(&Register8::A, res);
    }

    fn sbc_r8(&mut self, reg: Register8) {
        let reg_value: u8 = self.reg.get8(&reg);

        let accumulator: u8 = self.reg.get8(&Register8::A);
        let c_value: u8 = self.reg.getf(&Flag::C);
        let accumulator_carry: u8 = accumulator.wrapping_sub(c_value);

        let res: u8 = self.sub8_flags(accumulator_carry, reg_value, true, true);
        self.reg.set8(&Register8::A, res);
    }

    fn sbc_ir16(&mut self, reg: Register16) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let c_value: u8 = self.reg.getf(&Flag::C);
        let accumulator_carry: u8 = accumulator.wrapping_sub(c_value);

        let add: u16 = self.reg.get16(&reg);
        let value: u8 = self.mmu.read(add);

        let res: u8 = self.sub8_flags(accumulator_carry, value, true, true);
        self.reg.set8(&Register8::A, res);
    }

    fn sbc_u8(&mut self, reg: Register8) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let c_value: u8 = self.reg.getf(&Flag::C);
        let accumulator_carry: u8 = accumulator.wrapping_sub(c_value);

        let value: u8 = self.consume_u8();

        let res: u8 = self.sub8_flags(accumulator_carry, value, true, true);
        self.reg.set8(&Register8::A, res);
    }

    fn cp_r8(&mut self, reg: Register8) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let value: u8 = self.reg.get8(&reg);
        self.sub8_flags(accumulator, value, true, true);
    }

    fn cp_ir16(&mut self, reg: Register16) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let add: u16 = self.reg.get16(&reg);
        let value: u8 = self.mmu.read(add);
        self.sub8_flags(accumulator, value, true, true);
    }

    fn cp_u8(&mut self, reg: Register8) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let value: u8 = self.consume_u8();
        self.sub8_flags(accumulator, value, true, true);
    }

    fn inc_r8(&mut self, reg: Register8) {
        let value: u8 = self.reg.get8(&reg);
        let res: u8 = self.sum8_flags(value, 1, false, true);
        self.reg.set8(&reg, res);
    }

    fn inc_ir16(&mut self, reg: Register16) {
        let add: u16 = self.reg.get16(&reg);
        let value: u8 = self.mmu.read(add);
        let res: u8 = self.sum8_flags(value, 1, false, true);
        self.mmu.write(add, res);
    }

    fn dec_r8(&mut self, reg: Register8) {
        let value: u8 = self.reg.get8(&reg);
        let res: u8 = self.sub8_flags(value, 1, false, true);
        self.reg.set8(&reg, res);
    }

    fn dec_ir16(&mut self, reg: Register16) {
        let add: u16 = self.reg.get16(&reg);
        let value: u8 = self.mmu.read(add);
        let res: u8 = self.sub8_flags(value, 1, false, true);
        self.mmu.write(add, res);
    }

    fn nop(&mut self) {}
}
