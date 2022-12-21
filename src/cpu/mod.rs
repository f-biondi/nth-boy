use crate::mmu::{address_spaces::Addressable, Mmu};
use registers::{Flag, Register16, Register8, Registers};

mod registers;

const OP_CYCLES: &'static [u8] = &[
    1, 3, 2, 2, 1, 1, 2, 1, 5, 2, 2, 2, 1, 1, 2, 1, 1, 3, 2, 2, 1, 1, 2, 1, 3, 2, 2, 2, 1, 1, 2, 1,
    3, 3, 2, 2, 1, 1, 2, 1, 3, 2, 2, 2, 1, 1, 2, 1, 3, 3, 2, 2, 3, 3, 3, 1, 3, 2, 2, 2, 1, 1, 2, 1,
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1,
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 2, 2, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 2, 1,
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1,
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1,
    5, 3, 4, 4, 6, 4, 2, 4, 5, 4, 4, 1, 6, 6, 2, 4, 5, 3, 4, 1, 6, 4, 2, 4, 5, 4, 4, 1, 6, 1, 2, 4,
    3, 3, 2, 1, 1, 4, 2, 4, 4, 1, 4, 1, 1, 1, 2, 4, 3, 3, 2, 1, 1, 4, 2, 4, 3, 2, 4, 1, 1, 1, 2, 4,
];

enum PostOp {
    INC,
    DEC,
    NONE,
}

pub struct Cpu {
    reg: Registers,
    mmu: Mmu,
    cycles: u128,
    ime: bool,
    ime_scheduled: bool,
}

impl Cpu {
    pub fn new(mmu: Mmu) -> Self {
        Self {
            reg: Registers::new(),
            mmu: mmu,
            cycles: 0,
            ime: false,
            ime_scheduled: false,
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
            0x01 => self.ld_r16_u16(Register16::BC),
            0x02 => self.ld_ir16_r8(Register16::BC, Register8::A, PostOp::NONE),
            0x03 => self.inc_r16(Register16::BC),
            0x04 => self.inc_r8(Register8::B),
            0x05 => self.dec_r8(Register8::B),
            0x06 => self.ld_r8_u8(Register8::B),
            0x07 => self.rlca(),
            0x08 => self.ld_iu16_r16(Register16::SP),
            0x09 => self.add_r16_r16(Register16::HL, Register16::BC),
            0x0A => self.ld_r8_ir16(Register8::A, Register16::BC, PostOp::NONE),
            0x0B => self.dec_r16(Register16::BC),
            0x0C => self.inc_r8(Register8::C),
            0x0D => self.dec_r8(Register8::C),
            0x0E => self.ld_r8_u8(Register8::C),
            0x0F => self.rrca(),
            0x10 => self.stop_u8(),
            0x11 => self.ld_r16_u16(Register16::DE),
            0x12 => self.ld_ir16_r8(Register16::DE, Register8::A, PostOp::NONE),
            0x13 => self.inc_r16(Register16::DE),
            0x14 => self.inc_r8(Register8::D),
            0x15 => self.dec_r8(Register8::D),
            0x16 => self.ld_r8_u8(Register8::D),
            0x17 => self.rla(),
            0x18 => self.jr_i8(),
            0x19 => self.add_r16_r16(Register16::HL, Register16::DE),
            0x1A => self.ld_r8_ir16(Register8::A, Register16::DE, PostOp::NONE),
            0x1B => self.dec_r16(Register16::DE),
            0x1C => self.inc_r8(Register8::E),
            0x1D => self.dec_r8(Register8::E),
            0x1E => self.ld_r8_u8(Register8::E),
            0x1F => self.rra(),
            0x20 => self.jr_f_i8(Flag::NZ),
            0x21 => self.ld_r16_u16(Register16::HL),
            0x22 => self.ld_ir16_r8(Register16::HL, Register8::A, PostOp::INC),
            0x23 => self.inc_r16(Register16::HL),
            0x24 => self.inc_r8(Register8::H),
            0x25 => self.dec_r8(Register8::H),
            0x26 => self.ld_r8_u8(Register8::H),
            0x27 => self.daa(),
            0x28 => self.jr_f_i8(Flag::Z),
            0x29 => self.add_r16_r16(Register16::HL, Register16::HL),
            0x2A => self.ld_r8_ir16(Register8::A, Register16::HL, PostOp::INC),
            0x2B => self.dec_r16(Register16::HL),
            0x2C => self.inc_r8(Register8::L),
            0x2D => self.dec_r8(Register8::L),
            0x2E => self.ld_r8_u8(Register8::L),
            0x2F => self.cpl(),
            0x30 => self.jr_f_i8(Flag::NC),
            0x31 => self.ld_r16_u16(Register16::SP),
            0x32 => self.ld_ir16_r8(Register16::HL, Register8::A, PostOp::DEC),
            0x33 => self.inc_r16(Register16::SP),
            0x34 => self.inc_ir16(Register16::HL),
            0x35 => self.dec_ir16(Register16::HL),
            0x36 => self.ld_ir16_u8(Register16::HL),
            0x37 => self.scf(),
            0x38 => self.jr_f_i8(Flag::C),
            0x39 => self.add_r16_r16(Register16::HL, Register16::SP),
            0x3A => self.ld_r8_ir16(Register8::A, Register16::HL, PostOp::DEC),
            0x3B => self.dec_r16(Register16::SP),
            0x3C => self.inc_r8(Register8::A),
            0x3D => self.dec_r8(Register8::A),
            0x3E => self.ld_r8_u8(Register8::A),
            0x3F => self.ccf(),
            0x40 => self.ld_r8_r8(Register8::B, Register8::B),
            0x41 => self.ld_r8_r8(Register8::B, Register8::C),
            0x42 => self.ld_r8_r8(Register8::B, Register8::D),
            0x43 => self.ld_r8_r8(Register8::B, Register8::E),
            0x44 => self.ld_r8_r8(Register8::B, Register8::H),
            0x45 => self.ld_r8_r8(Register8::B, Register8::L),
            0x46 => self.ld_r8_ir16(Register8::B, Register16::HL, PostOp::NONE),
            0x47 => self.ld_r8_r8(Register8::B, Register8::A),
            0x48 => self.ld_r8_r8(Register8::C, Register8::B),
            0x49 => self.ld_r8_r8(Register8::C, Register8::C),
            0x4A => self.ld_r8_r8(Register8::C, Register8::D),
            0x4B => self.ld_r8_r8(Register8::C, Register8::E),
            0x4C => self.ld_r8_r8(Register8::C, Register8::H),
            0x4D => self.ld_r8_r8(Register8::C, Register8::L),
            0x4E => self.ld_r8_ir16(Register8::C, Register16::HL, PostOp::NONE),
            0x4F => self.ld_r8_r8(Register8::C, Register8::A),
            0x50 => self.ld_r8_r8(Register8::D, Register8::B),
            0x51 => self.ld_r8_r8(Register8::D, Register8::C),
            0x52 => self.ld_r8_r8(Register8::D, Register8::D),
            0x53 => self.ld_r8_r8(Register8::D, Register8::E),
            0x54 => self.ld_r8_r8(Register8::D, Register8::H),
            0x55 => self.ld_r8_r8(Register8::D, Register8::L),
            0x56 => self.ld_r8_ir16(Register8::D, Register16::HL, PostOp::NONE),
            0x57 => self.ld_r8_r8(Register8::D, Register8::A),
            0x58 => self.ld_r8_r8(Register8::E, Register8::B),
            0x59 => self.ld_r8_r8(Register8::E, Register8::C),
            0x5A => self.ld_r8_r8(Register8::E, Register8::D),
            0x5B => self.ld_r8_r8(Register8::E, Register8::E),
            0x5C => self.ld_r8_r8(Register8::E, Register8::H),
            0x5D => self.ld_r8_r8(Register8::E, Register8::L),
            0x5E => self.ld_r8_ir16(Register8::E, Register16::HL, PostOp::NONE),
            0x5F => self.ld_r8_r8(Register8::E, Register8::A),
            0x60 => self.ld_r8_r8(Register8::H, Register8::B),
            0x61 => self.ld_r8_r8(Register8::H, Register8::C),
            0x62 => self.ld_r8_r8(Register8::H, Register8::D),
            0x63 => self.ld_r8_r8(Register8::H, Register8::E),
            0x64 => self.ld_r8_r8(Register8::H, Register8::H),
            0x65 => self.ld_r8_r8(Register8::H, Register8::L),
            0x66 => self.ld_r8_ir16(Register8::H, Register16::HL, PostOp::NONE),
            0x67 => self.ld_r8_r8(Register8::H, Register8::A),
            0x68 => self.ld_r8_r8(Register8::L, Register8::B),
            0x69 => self.ld_r8_r8(Register8::L, Register8::C),
            0x6A => self.ld_r8_r8(Register8::L, Register8::D),
            0x6B => self.ld_r8_r8(Register8::L, Register8::E),
            0x6C => self.ld_r8_r8(Register8::L, Register8::H),
            0x6D => self.ld_r8_r8(Register8::L, Register8::L),
            0x6E => self.ld_r8_ir16(Register8::L, Register16::HL, PostOp::NONE),
            0x6F => self.ld_r8_r8(Register8::L, Register8::A),
            0x70 => self.ld_ir16_r8(Register16::HL, Register8::B, PostOp::NONE),
            0x71 => self.ld_ir16_r8(Register16::HL, Register8::C, PostOp::NONE),
            0x72 => self.ld_ir16_r8(Register16::HL, Register8::D, PostOp::NONE),
            0x73 => self.ld_ir16_r8(Register16::HL, Register8::E, PostOp::NONE),
            0x74 => self.ld_ir16_r8(Register16::HL, Register8::H, PostOp::NONE),
            0x75 => self.ld_ir16_r8(Register16::HL, Register8::L, PostOp::NONE),
            0x76 => self.halt(),
            0x77 => self.ld_ir16_r8(Register16::HL, Register8::A, PostOp::NONE),
            0x78 => self.ld_r8_r8(Register8::A, Register8::B),
            0x79 => self.ld_r8_r8(Register8::A, Register8::C),
            0x7A => self.ld_r8_r8(Register8::A, Register8::D),
            0x7B => self.ld_r8_r8(Register8::A, Register8::E),
            0x7C => self.ld_r8_r8(Register8::A, Register8::H),
            0x7D => self.ld_r8_r8(Register8::A, Register8::L),
            0x7E => self.ld_r8_ir16(Register8::A, Register16::HL, PostOp::NONE),
            0x7F => self.ld_r8_r8(Register8::A, Register8::A),
            0x80 => self.add_r8(Register8::B),
            0x81 => self.add_r8(Register8::C),
            0x82 => self.add_r8(Register8::D),
            0x83 => self.add_r8(Register8::E),
            0x84 => self.add_r8(Register8::H),
            0x85 => self.add_r8(Register8::L),
            0x86 => self.add_ir16(Register16::HL),
            0x87 => self.add_r8(Register8::A),
            0x88 => self.adc_r8(Register8::B),
            0x89 => self.adc_r8(Register8::C),
            0x8A => self.adc_r8(Register8::D),
            0x8B => self.adc_r8(Register8::E),
            0x8C => self.adc_r8(Register8::H),
            0x8D => self.adc_r8(Register8::L),
            0x8E => self.adc_ir16(Register16::HL),
            0x8F => self.adc_r8(Register8::A),
            0x90 => self.sub_r8(Register8::B),
            0x91 => self.sub_r8(Register8::C),
            0x92 => self.sub_r8(Register8::D),
            0x93 => self.sub_r8(Register8::E),
            0x94 => self.sub_r8(Register8::H),
            0x95 => self.sub_r8(Register8::L),
            0x96 => self.sub_ir16(Register16::HL),
            0x97 => self.sub_r8(Register8::A),
            0x98 => self.sbc_r8(Register8::B),
            0x99 => self.sbc_r8(Register8::C),
            0x9A => self.sbc_r8(Register8::D),
            0x9B => self.sbc_r8(Register8::E),
            0x9C => self.sbc_r8(Register8::H),
            0x9D => self.sbc_r8(Register8::L),
            0x9E => self.sbc_ir16(Register16::HL),
            0x9F => self.sbc_r8(Register8::A),
            0xA0 => self.and_r8(Register8::B),
            0xA1 => self.and_r8(Register8::C),
            0xA2 => self.and_r8(Register8::D),
            0xA3 => self.and_r8(Register8::E),
            0xA4 => self.and_r8(Register8::H),
            0xA5 => self.and_r8(Register8::L),
            0xA6 => self.and_ir16(Register16::HL),
            0xA7 => self.and_r8(Register8::A),
            0xA8 => self.xor_r8(Register8::B),
            0xA9 => self.xor_r8(Register8::C),
            0xAA => self.xor_r8(Register8::D),
            0xAB => self.xor_r8(Register8::E),
            0xAC => self.xor_r8(Register8::H),
            0xAD => self.xor_r8(Register8::L),
            0xAE => self.xor_ir16(Register16::HL),
            0xAF => self.xor_r8(Register8::A),
            0xB0 => self.or_r8(Register8::B),
            0xB1 => self.or_r8(Register8::C),
            0xB2 => self.or_r8(Register8::D),
            0xB3 => self.or_r8(Register8::E),
            0xB4 => self.or_r8(Register8::H),
            0xB5 => self.or_r8(Register8::L),
            0xB6 => self.or_ir16(Register16::HL),
            0xB7 => self.or_r8(Register8::A),
            0xB8 => self.cp_r8(Register8::B),
            0xB9 => self.cp_r8(Register8::C),
            0xBA => self.cp_r8(Register8::D),
            0xBB => self.cp_r8(Register8::E),
            0xBC => self.cp_r8(Register8::H),
            0xBD => self.cp_r8(Register8::L),
            0xBE => self.cp_ir16(Register16::HL),
            0xBF => self.cp_r8(Register8::A),
            0xC0 => self.ret_f(Flag::NZ),
            0xC1 => self.pop_r16(Register16::BC),
            0xC2 => self.jp_f_u16(Flag::NZ),
            0xC3 => self.jp_u16(),
            0xC4 => self.call_f_u16(Flag::NZ),
            0xC5 => self.push_r16(Register16::BC),
            0xC6 => self.add_u8(),
            0xC7 => self.rst_f8(0x00),
            0xC8 => self.ret_f(Flag::Z),
            0xC9 => self.ret(),
            0xCA => self.jp_f_u16(Flag::Z),
            0xCB => panic!("CB opcodes not implemented"),
            0xCC => self.call_f_u16(Flag::Z),
            0xCD => self.call_u16(),
            0xCE => self.adc_u8(),
            0xCF => self.rst_f8(0x08),
            0xD0 => self.ret_f(Flag::NC),
            0xD1 => self.pop_r16(Register16::DE),
            0xD2 => self.jp_f_u16(Flag::NC),
            0xD3 => self.nop(),
            0xD4 => self.call_f_u16(Flag::NC),
            0xD5 => self.push_r16(Register16::DE),
            0xD6 => self.sub_u8(),
            0xD7 => self.rst_f8(0x10),
            0xD8 => self.ret_f(Flag::C),
            0xD9 => self.reti(),
            0xDA => self.jp_f_u16(Flag::C),
            0xDB => self.nop(),
            0xDC => self.call_f_u16(Flag::C),
            0xDD => self.nop(),
            0xDE => self.sbc_u8(),
            0xDF => self.rst_f8(0x18),
            0xE0 => self.ldh_iu8_r8(Register8::A),
            0xE1 => self.pop_r16(Register16::HL),
            0xE2 => self.ld_ir8_r8(Register8::C, Register8::A),
            0xE3 => self.nop(),
            0xE4 => self.nop(),
            0xE5 => self.push_r16(Register16::HL),
            0xE6 => self.and_u8(),
            0xE7 => self.rst_f8(0x20),
            0xE8 => self.add_r16_i8(Register16::SP),
            0xE9 => self.jp_r16(Register16::HL),
            0xEA => self.ld_iu16_r8(Register8::A),
            0xEB => self.nop(),
            0xEC => self.nop(),
            0xED => self.nop(),
            0xEE => self.xor_u8(),
            0xEF => self.rst_f8(0x28),
            0xF0 => self.ldh_r8_iu8(Register8::A),
            0xF1 => self.pop_r16(Register16::AF),
            0xF2 => self.ld_r8_ir8(Register8::A, Register8::C),
            0xF3 => self.di(),
            0xF4 => self.nop(),
            0xF5 => self.push_r16(Register16::AF),
            0xF6 => self.or_u8(),
            0xF7 => self.rst_f8(0x30),
            0xF8 => self.ld_r16_r16_i8(Register16::HL, Register16::SP),
            0xF9 => self.ld_r16_r16(Register16::SP, Register16::HL),
            0xFA => self.ld_r8_iu16(Register8::A),
            0xFB => self.ei(),
            0xFC => self.nop(),
            0xFD => self.nop(),
            0xFE => self.cp_u8(),
            0xFF => self.rst_f8(0x38),
            _ => panic!("{:#x} opcode not implemented", op),
        }
        self.step_cycles(OP_CYCLES[op as usize]);
    }

    fn step_cycles(&mut self, steps: u8) {
        self.cycles += 4 * (steps as u128);
    }

    fn consume_u8(&mut self) -> u8 {
        let r: u8 = self.mmu.read(self.reg.pc);
        self.reg.pc += 1;
        r
    }

    fn consume_i8(&mut self) -> i8 {
        let r: i8 = self.mmu.read(self.reg.pc) as i8;
        self.reg.pc += 1;
        r
    }

    fn consume_u16(&mut self) -> u16 {
        let r: u16 = self.mmu.read_16(self.reg.pc);
        self.reg.pc += 2;
        r
    }

    fn stack_push_u16(&mut self, value: u16) {
        let value_high: u8 = ((value & 0xff00) >> 8) as u8;
        let value_low: u8 = (value & 0x00ff) as u8;
        self.stack_push_u8(value_high);
        self.stack_push_u8(value_low);
    }

    fn stack_pop_u16(&mut self) -> u16 {
        let value_low: u8 = self.stack_pop_u8();
        let value_high: u8 = self.stack_pop_u8();
        ((value_high as u16) << 8) + (value_low as u16)
    }

    fn stack_push_u8(&mut self, value: u8) {
        self.reg.sp -= 1;
        self.mmu.write(self.reg.sp, value);
    }

    fn stack_pop_u8(&mut self) -> u8 {
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

    fn ld_r8_ir8(&mut self, dest: Register8, src: Register8) {
        let add_low: u8 = self.reg.get8(&src);
        let add: u16 = 0xff00 & (add_low as u16);
        let value: u8 = self.mmu.read(add);
        self.reg.set8(&dest, value);
    }

    fn ld_ir8_r8(&mut self, dest: Register8, src: Register8) {
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
        self.stack_push_u16(value);
    }

    fn pop_r16(&mut self, dest: Register16) {
        let value: u16 = self.stack_pop_u16();
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

    fn add_u8(&mut self) {
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

    fn adc_u8(&mut self) {
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

    fn sub_u8(&mut self) {
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

    fn sbc_u8(&mut self) {
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

    fn cp_u8(&mut self) {
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

    fn and8_flags(&mut self, op1: u8, op2: u8) -> u8 {
        let res: u8 = op1 & op2;

        if res == 0 {
            self.reg.setf(&Flag::Z);
        } else {
            self.reg.unsetf(&Flag::Z);
        }

        self.reg.unsetf(&Flag::N);
        self.reg.setf(&Flag::H);
        self.reg.unsetf(&Flag::C);
        res
    }

    fn and_r8(&mut self, reg: Register8) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let value: u8 = self.reg.get8(&reg);
        let res: u8 = self.and8_flags(accumulator, value);
        self.reg.set8(&Register8::A, res);
    }

    fn and_ir16(&mut self, reg: Register16) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let add: u16 = self.reg.get16(&reg);
        let value: u8 = self.mmu.read(add);
        let res: u8 = self.and8_flags(accumulator, value);
        self.reg.set8(&Register8::A, res);
    }

    fn and_u8(&mut self) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let value: u8 = self.consume_u8();
        let res: u8 = self.and8_flags(accumulator, value);
        self.reg.set8(&Register8::A, res);
    }

    fn or8_flags(&mut self, op1: u8, op2: u8) -> u8 {
        let res: u8 = op1 | op2;

        if res == 0 {
            self.reg.setf(&Flag::Z);
        } else {
            self.reg.unsetf(&Flag::Z);
        }

        self.reg.unsetf(&Flag::N);
        self.reg.unsetf(&Flag::H);
        self.reg.unsetf(&Flag::C);
        res
    }

    fn or_r8(&mut self, reg: Register8) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let value: u8 = self.reg.get8(&reg);
        let res: u8 = self.or8_flags(accumulator, value);
        self.reg.set8(&Register8::A, res);
    }

    fn or_ir16(&mut self, reg: Register16) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let add: u16 = self.reg.get16(&reg);
        let value: u8 = self.mmu.read(add);
        let res: u8 = self.or8_flags(accumulator, value);
        self.reg.set8(&Register8::A, res);
    }

    fn or_u8(&mut self) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let value: u8 = self.consume_u8();
        let res: u8 = self.or8_flags(accumulator, value);
        self.reg.set8(&Register8::A, res);
    }

    fn xor8_flags(&mut self, op1: u8, op2: u8) -> u8 {
        let res: u8 = op1 ^ op2;

        if res == 0 {
            self.reg.setf(&Flag::Z);
        } else {
            self.reg.unsetf(&Flag::Z);
        }

        self.reg.unsetf(&Flag::N);
        self.reg.unsetf(&Flag::H);
        self.reg.unsetf(&Flag::C);
        res
    }

    fn xor_r8(&mut self, reg: Register8) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let value: u8 = self.reg.get8(&reg);
        let res: u8 = self.xor8_flags(accumulator, value);
        self.reg.set8(&Register8::A, res);
    }

    fn xor_ir16(&mut self, reg: Register16) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let add: u16 = self.reg.get16(&reg);
        let value: u8 = self.mmu.read(add);
        let res: u8 = self.xor8_flags(accumulator, value);
        self.reg.set8(&Register8::A, res);
    }

    fn xor_u8(&mut self) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let value: u8 = self.consume_u8();
        let res: u8 = self.xor8_flags(accumulator, value);
        self.reg.set8(&Register8::A, res);
    }

    fn ccf(&mut self) {
        self.reg.unsetf(&Flag::N);
        self.reg.unsetf(&Flag::H);

        if self.reg.getf(&Flag::C) == 1 {
            self.reg.setf(&Flag::C);
        } else {
            self.reg.unsetf(&Flag::C);
        }
    }

    fn scf(&mut self) {
        self.reg.unsetf(&Flag::N);
        self.reg.unsetf(&Flag::H);
        self.reg.setf(&Flag::C);
    }

    fn cpl(&mut self) {
        self.reg.set8(&Register8::A, !self.reg.get8(&Register8::A));
        self.reg.setf(&Flag::N);
        self.reg.setf(&Flag::H);
    }

    fn jp_u16(&mut self) {
        self.reg.pc = self.consume_u16();
    }

    fn jp_r16(&mut self, reg: Register16) {
        self.reg.pc = self.reg.get16(&reg);
    }

    fn jp_f_u16(&mut self, flag: Flag) {
        let add: u16 = self.consume_u16();

        if self.reg.getf(&flag) == 1 {
            self.step_cycles(1);
            self.reg.pc = add;
        }
    }

    fn get_relative_add(&self, offset: i8) -> u16 {
        (self.reg.pc as i32).wrapping_add(offset as i32) as u16
    }

    fn jr_i8(&mut self) {
        let offset: i8 = self.consume_i8();
        let add: u16 = self.get_relative_add(offset);
        self.reg.pc = add;
    }

    fn jr_f_i8(&mut self, flag: Flag) {
        let offset: i8 = self.consume_i8();
        let add: u16 = self.get_relative_add(offset);

        if self.reg.getf(&flag) == 1 {
            self.step_cycles(1);
            self.reg.pc = add;
        }
    }

    fn call_u16(&mut self) {
        let add: u16 = self.consume_u16();
        self.stack_push_u16(self.reg.pc);
        self.reg.pc = add;
    }

    fn call_f_u16(&mut self, flag: Flag) {
        let add: u16 = self.consume_u16();

        if self.reg.getf(&flag) == 1 {
            self.step_cycles(3);
            self.stack_push_u16(self.reg.pc);
            self.reg.pc = add;
        }
    }

    fn ret(&mut self) {
        self.reg.pc = self.stack_pop_u16();
    }

    fn ret_f(&mut self, flag: Flag) {
        if self.reg.getf(&flag) == 1 {
            self.step_cycles(3);
            self.reg.pc = self.stack_pop_u16();
        }
    }

    fn rst_f8(&mut self, fixed: u8) {
        let add: u16 = fixed as u16;
        self.stack_push_u16(self.reg.pc);
        self.reg.pc = add;
    }

    fn di(&mut self) {
        self.ime = false;
    }

    fn ei(&mut self) {
        self.ime_scheduled = true;
    }

    fn nop(&mut self) {}
}
