use crate::mmu::{address_spaces::Addressable, Mmu};
use registers::{Flag, Register16, Register8, Registers};
use std::error::Error;

mod registers;

const OP_CYCLES: &'static [u8] = &[
    4, 12, 8, 8, 4, 4, 8, 4, 20, 8, 8, 8, 4, 4, 8, 4, 4, 12, 8, 8, 4, 4, 8, 4, 12, 8, 8, 8, 4, 4,
    8, 4, 8, 12, 8, 8, 4, 4, 8, 4, 8, 8, 8, 8, 4, 4, 8, 4, 8, 12, 8, 8, 12, 12, 12, 4, 8, 8, 8, 8,
    4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4,
    4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 8, 8, 8, 8, 8, 8, 4, 8, 4, 4, 4, 4,
    4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4,
    4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4,
    4, 4, 8, 4, 8, 12, 12, 16, 12, 16, 8, 16, 8, 16, 12, 0, 12, 24, 8, 16, 8, 12, 12, 4, 12, 16, 8,
    16, 8, 16, 12, 4, 12, 4, 8, 16, 12, 12, 8, 4, 4, 16, 8, 16, 16, 4, 16, 4, 4, 4, 8, 16, 12, 12,
    8, 4, 4, 16, 8, 16, 12, 8, 16, 4, 4, 4, 8, 16,
];

const CB_OP_CYCLES: &'static [u8] = &[
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8,
    16, 8, 8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8,
    8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 12, 8, 8, 8, 8, 8, 8, 8, 12, 8, 8, 8, 8, 8, 8, 8, 12, 8, 8, 8,
    8, 8, 8, 8, 12, 8, 8, 8, 8, 8, 8, 8, 12, 8, 8, 8, 8, 8, 8, 8, 12, 8, 8, 8, 8, 8, 8, 8, 12, 8,
    8, 8, 8, 8, 8, 8, 12, 8, 8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8,
    16, 8, 8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8,
    8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8, 8, 8,
    8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
    8, 8, 8, 8, 8, 8, 16, 8, 8, 8, 8, 8, 8, 8, 16, 8,
];

enum PostOp {
    INC,
    DEC,
    NONE,
}

#[derive(PartialEq)]
enum Ime {
    ENABLED,
    DISABLED,
    PENDING,
}

pub struct Cpu {
    reg: Registers,
    pub cycles: u128,
    pub ops: u128,
    ime: Ime,
    halted: bool
}

impl Cpu {
    pub fn new() -> Cpu {
        Self {
            reg: Registers::new(),
            cycles: 0,
            ops: 0,
            ime: Ime::DISABLED,
            halted: false
        }
    }

    pub fn tick(&mut self, mmu: &mut Mmu) -> u8 {
        let start: u128 = self.cycles;

        if self.ime == Ime::PENDING {
            self.ime = Ime::ENABLED;
        }

        if !self.halted {
            self.check_interrupts(mmu);
            let op: u8 = self.consume_u8(mmu);
            self.decode(mmu, op);
        } else {
            if (mmu.ie_flag & mmu.io.if_flag) != 0 {
                self.halted = false;
            }
            self.step_cycles(8);
        }

        (self.cycles - start) as u8
    }

    fn check_interrupts(&mut self, mmu: &mut Mmu) {
       let ie_f: u8 = mmu.ie_flag;
       let if_f: u8 = mmu.io.if_flag;

       if self.ime == Ime::ENABLED {
            if ((ie_f & 0x01) & (if_f & 0x01)) != 0 {
                self.handle_interrupt(mmu, 0x0040, 0x01);
            } else if ((ie_f & 0x02) & (if_f & 0x02)) != 0 {
                self.handle_interrupt(mmu, 0x0048, 0x02);
            } else if ((ie_f & 0x04) & (if_f & 0x04)) != 0 {
                self.handle_interrupt(mmu, 0x0050, 0x04);
            } else if ((ie_f & 0x08) & (if_f & 0x08)) != 0 {
                self.handle_interrupt(mmu, 0x0058, 0x08);
            } else if ((ie_f & 0x10) & (if_f & 0x10)) != 0 {
                self.handle_interrupt(mmu, 0x0060, 0x10);
            }
       }
    }

    fn handle_interrupt(&mut self, mmu: &mut Mmu, int: u16, bit: u8) {
        self.stack_push_u16(mmu, self.reg.pc);
        mmu.io.if_flag ^= bit;
        self.reg.pc = int;
        self.ime = Ime::DISABLED;
        self.step_cycles(20);
    }

    fn decode(&mut self, mmu: &mut Mmu, op: u8) {
        //println!("Executing {:#01x}", op);
        match op {
            0x00 => self.nop(),
            0x01 => self.ld_r16_u16(mmu, Register16::BC),
            0x02 => self.ld_ir16_r8(mmu, Register16::BC, Register8::A, PostOp::NONE),
            0x03 => self.inc_r16(Register16::BC),
            0x04 => self.inc_r8(Register8::B),
            0x05 => self.dec_r8(Register8::B),
            0x06 => self.ld_r8_u8(mmu, Register8::B),
            0x07 => self.rlca(),
            0x08 => self.ld_iu16_r16(mmu, Register16::SP),
            0x09 => self.add_r16_r16(Register16::HL, Register16::BC),
            0x0A => self.ld_r8_ir16(mmu, Register8::A, Register16::BC, PostOp::NONE),
            0x0B => self.dec_r16(Register16::BC),
            0x0C => self.inc_r8(Register8::C),
            0x0D => self.dec_r8(Register8::C),
            0x0E => self.ld_r8_u8(mmu, Register8::C),
            0x0F => self.rrca(),
            0x10 => self.stop(),
            0x11 => self.ld_r16_u16(mmu, Register16::DE),
            0x12 => self.ld_ir16_r8(mmu, Register16::DE, Register8::A, PostOp::NONE),
            0x13 => self.inc_r16(Register16::DE),
            0x14 => self.inc_r8(Register8::D),
            0x15 => self.dec_r8(Register8::D),
            0x16 => self.ld_r8_u8(mmu, Register8::D),
            0x17 => self.rla(),
            0x18 => self.jr_i8(mmu),
            0x19 => self.add_r16_r16(Register16::HL, Register16::DE),
            0x1A => self.ld_r8_ir16(mmu, Register8::A, Register16::DE, PostOp::NONE),
            0x1B => self.dec_r16(Register16::DE),
            0x1C => self.inc_r8(Register8::E),
            0x1D => self.dec_r8(Register8::E),
            0x1E => self.ld_r8_u8(mmu, Register8::E),
            0x1F => self.rra(),
            0x20 => self.jr_f_i8(mmu, Flag::NZ),
            0x21 => self.ld_r16_u16(mmu, Register16::HL),
            0x22 => self.ld_ir16_r8(mmu, Register16::HL, Register8::A, PostOp::INC),
            0x23 => self.inc_r16(Register16::HL),
            0x24 => self.inc_r8(Register8::H),
            0x25 => self.dec_r8(Register8::H),
            0x26 => self.ld_r8_u8(mmu, Register8::H),
            0x27 => self.daa(),
            0x28 => self.jr_f_i8(mmu, Flag::Z),
            0x29 => self.add_r16_r16(Register16::HL, Register16::HL),
            0x2A => self.ld_r8_ir16(mmu, Register8::A, Register16::HL, PostOp::INC),
            0x2B => self.dec_r16(Register16::HL),
            0x2C => self.inc_r8(Register8::L),
            0x2D => self.dec_r8(Register8::L),
            0x2E => self.ld_r8_u8(mmu, Register8::L),
            0x2F => self.cpl(),
            0x30 => self.jr_f_i8(mmu, Flag::NC),
            0x31 => self.ld_r16_u16(mmu, Register16::SP),
            0x32 => self.ld_ir16_r8(mmu, Register16::HL, Register8::A, PostOp::DEC),
            0x33 => self.inc_r16(Register16::SP),
            0x34 => self.inc_ir16(mmu, Register16::HL),
            0x35 => self.dec_ir16(mmu, Register16::HL),
            0x36 => self.ld_ir16_u8(mmu, Register16::HL),
            0x37 => self.scf(),
            0x38 => self.jr_f_i8(mmu, Flag::C),
            0x39 => self.add_r16_r16(Register16::HL, Register16::SP),
            0x3A => self.ld_r8_ir16(mmu, Register8::A, Register16::HL, PostOp::DEC),
            0x3B => self.dec_r16(Register16::SP),
            0x3C => self.inc_r8(Register8::A),
            0x3D => self.dec_r8(Register8::A),
            0x3E => self.ld_r8_u8(mmu, Register8::A),
            0x3F => self.ccf(),
            0x40 => self.ld_r8_r8(Register8::B, Register8::B),
            0x41 => self.ld_r8_r8(Register8::B, Register8::C),
            0x42 => self.ld_r8_r8(Register8::B, Register8::D),
            0x43 => self.ld_r8_r8(Register8::B, Register8::E),
            0x44 => self.ld_r8_r8(Register8::B, Register8::H),
            0x45 => self.ld_r8_r8(Register8::B, Register8::L),
            0x46 => self.ld_r8_ir16(mmu, Register8::B, Register16::HL, PostOp::NONE),
            0x47 => self.ld_r8_r8(Register8::B, Register8::A),
            0x48 => self.ld_r8_r8(Register8::C, Register8::B),
            0x49 => self.ld_r8_r8(Register8::C, Register8::C),
            0x4A => self.ld_r8_r8(Register8::C, Register8::D),
            0x4B => self.ld_r8_r8(Register8::C, Register8::E),
            0x4C => self.ld_r8_r8(Register8::C, Register8::H),
            0x4D => self.ld_r8_r8(Register8::C, Register8::L),
            0x4E => self.ld_r8_ir16(mmu, Register8::C, Register16::HL, PostOp::NONE),
            0x4F => self.ld_r8_r8(Register8::C, Register8::A),
            0x50 => self.ld_r8_r8(Register8::D, Register8::B),
            0x51 => self.ld_r8_r8(Register8::D, Register8::C),
            0x52 => self.ld_r8_r8(Register8::D, Register8::D),
            0x53 => self.ld_r8_r8(Register8::D, Register8::E),
            0x54 => self.ld_r8_r8(Register8::D, Register8::H),
            0x55 => self.ld_r8_r8(Register8::D, Register8::L),
            0x56 => self.ld_r8_ir16(mmu, Register8::D, Register16::HL, PostOp::NONE),
            0x57 => self.ld_r8_r8(Register8::D, Register8::A),
            0x58 => self.ld_r8_r8(Register8::E, Register8::B),
            0x59 => self.ld_r8_r8(Register8::E, Register8::C),
            0x5A => self.ld_r8_r8(Register8::E, Register8::D),
            0x5B => self.ld_r8_r8(Register8::E, Register8::E),
            0x5C => self.ld_r8_r8(Register8::E, Register8::H),
            0x5D => self.ld_r8_r8(Register8::E, Register8::L),
            0x5E => self.ld_r8_ir16(mmu, Register8::E, Register16::HL, PostOp::NONE),
            0x5F => self.ld_r8_r8(Register8::E, Register8::A),
            0x60 => self.ld_r8_r8(Register8::H, Register8::B),
            0x61 => self.ld_r8_r8(Register8::H, Register8::C),
            0x62 => self.ld_r8_r8(Register8::H, Register8::D),
            0x63 => self.ld_r8_r8(Register8::H, Register8::E),
            0x64 => self.ld_r8_r8(Register8::H, Register8::H),
            0x65 => self.ld_r8_r8(Register8::H, Register8::L),
            0x66 => self.ld_r8_ir16(mmu, Register8::H, Register16::HL, PostOp::NONE),
            0x67 => self.ld_r8_r8(Register8::H, Register8::A),
            0x68 => self.ld_r8_r8(Register8::L, Register8::B),
            0x69 => self.ld_r8_r8(Register8::L, Register8::C),
            0x6A => self.ld_r8_r8(Register8::L, Register8::D),
            0x6B => self.ld_r8_r8(Register8::L, Register8::E),
            0x6C => self.ld_r8_r8(Register8::L, Register8::H),
            0x6D => self.ld_r8_r8(Register8::L, Register8::L),
            0x6E => self.ld_r8_ir16(mmu, Register8::L, Register16::HL, PostOp::NONE),
            0x6F => self.ld_r8_r8(Register8::L, Register8::A),
            0x70 => self.ld_ir16_r8(mmu, Register16::HL, Register8::B, PostOp::NONE),
            0x71 => self.ld_ir16_r8(mmu, Register16::HL, Register8::C, PostOp::NONE),
            0x72 => self.ld_ir16_r8(mmu, Register16::HL, Register8::D, PostOp::NONE),
            0x73 => self.ld_ir16_r8(mmu, Register16::HL, Register8::E, PostOp::NONE),
            0x74 => self.ld_ir16_r8(mmu, Register16::HL, Register8::H, PostOp::NONE),
            0x75 => self.ld_ir16_r8(mmu, Register16::HL, Register8::L, PostOp::NONE),
            0x76 => self.halt(),
            0x77 => self.ld_ir16_r8(mmu, Register16::HL, Register8::A, PostOp::NONE),
            0x78 => self.ld_r8_r8(Register8::A, Register8::B),
            0x79 => self.ld_r8_r8(Register8::A, Register8::C),
            0x7A => self.ld_r8_r8(Register8::A, Register8::D),
            0x7B => self.ld_r8_r8(Register8::A, Register8::E),
            0x7C => self.ld_r8_r8(Register8::A, Register8::H),
            0x7D => self.ld_r8_r8(Register8::A, Register8::L),
            0x7E => self.ld_r8_ir16(mmu, Register8::A, Register16::HL, PostOp::NONE),
            0x7F => self.ld_r8_r8(Register8::A, Register8::A),
            0x80 => self.add_r8(Register8::B),
            0x81 => self.add_r8(Register8::C),
            0x82 => self.add_r8(Register8::D),
            0x83 => self.add_r8(Register8::E),
            0x84 => self.add_r8(Register8::H),
            0x85 => self.add_r8(Register8::L),
            0x86 => self.add_ir16(mmu, Register16::HL),
            0x87 => self.add_r8(Register8::A),
            0x88 => self.adc_r8(Register8::B),
            0x89 => self.adc_r8(Register8::C),
            0x8A => self.adc_r8(Register8::D),
            0x8B => self.adc_r8(Register8::E),
            0x8C => self.adc_r8(Register8::H),
            0x8D => self.adc_r8(Register8::L),
            0x8E => self.adc_ir16(mmu, Register16::HL),
            0x8F => self.adc_r8(Register8::A),
            0x90 => self.sub_r8(Register8::B),
            0x91 => self.sub_r8(Register8::C),
            0x92 => self.sub_r8(Register8::D),
            0x93 => self.sub_r8(Register8::E),
            0x94 => self.sub_r8(Register8::H),
            0x95 => self.sub_r8(Register8::L),
            0x96 => self.sub_ir16(mmu, Register16::HL),
            0x97 => self.sub_r8(Register8::A),
            0x98 => self.sbc_r8(Register8::B),
            0x99 => self.sbc_r8(Register8::C),
            0x9A => self.sbc_r8(Register8::D),
            0x9B => self.sbc_r8(Register8::E),
            0x9C => self.sbc_r8(Register8::H),
            0x9D => self.sbc_r8(Register8::L),
            0x9E => self.sbc_ir16(mmu, Register16::HL),
            0x9F => self.sbc_r8(Register8::A),
            0xA0 => self.and_r8(Register8::B),
            0xA1 => self.and_r8(Register8::C),
            0xA2 => self.and_r8(Register8::D),
            0xA3 => self.and_r8(Register8::E),
            0xA4 => self.and_r8(Register8::H),
            0xA5 => self.and_r8(Register8::L),
            0xA6 => self.and_ir16(mmu, Register16::HL),
            0xA7 => self.and_r8(Register8::A),
            0xA8 => self.xor_r8(Register8::B),
            0xA9 => self.xor_r8(Register8::C),
            0xAA => self.xor_r8(Register8::D),
            0xAB => self.xor_r8(Register8::E),
            0xAC => self.xor_r8(Register8::H),
            0xAD => self.xor_r8(Register8::L),
            0xAE => self.xor_ir16(mmu, Register16::HL),
            0xAF => self.xor_r8(Register8::A),
            0xB0 => self.or_r8(Register8::B),
            0xB1 => self.or_r8(Register8::C),
            0xB2 => self.or_r8(Register8::D),
            0xB3 => self.or_r8(Register8::E),
            0xB4 => self.or_r8(Register8::H),
            0xB5 => self.or_r8(Register8::L),
            0xB6 => self.or_ir16(mmu, Register16::HL),
            0xB7 => self.or_r8(Register8::A),
            0xB8 => self.cp_r8(Register8::B),
            0xB9 => self.cp_r8(Register8::C),
            0xBA => self.cp_r8(Register8::D),
            0xBB => self.cp_r8(Register8::E),
            0xBC => self.cp_r8(Register8::H),
            0xBD => self.cp_r8(Register8::L),
            0xBE => self.cp_ir16(mmu, Register16::HL),
            0xBF => self.cp_r8(Register8::A),
            0xC0 => self.ret_f(mmu, Flag::NZ),
            0xC1 => self.pop_r16(mmu, Register16::BC),
            0xC2 => self.jp_f_u16(mmu, Flag::NZ),
            0xC3 => self.jp_u16(mmu),
            0xC4 => self.call_f_u16(mmu, Flag::NZ),
            0xC5 => self.push_r16(mmu, Register16::BC),
            0xC6 => self.add_u8(mmu),
            0xC7 => self.rst_f8(mmu, 0x00),
            0xC8 => self.ret_f(mmu, Flag::Z),
            0xC9 => self.ret(mmu),
            0xCA => self.jp_f_u16(mmu, Flag::Z),
            0xCB => {
                let op: u8 = self.consume_u8(mmu);
                self.decode_cb(mmu, op);
            }
            0xCC => self.call_f_u16(mmu, Flag::Z),
            0xCD => self.call_u16(mmu),
            0xCE => self.adc_u8(mmu),
            0xCF => self.rst_f8(mmu, 0x08),
            0xD0 => self.ret_f(mmu, Flag::NC),
            0xD1 => self.pop_r16(mmu, Register16::DE),
            0xD2 => self.jp_f_u16(mmu, Flag::NC),
            0xD3 => self.nop(),
            0xD4 => self.call_f_u16(mmu, Flag::NC),
            0xD5 => self.push_r16(mmu, Register16::DE),
            0xD6 => self.sub_u8(mmu),
            0xD7 => self.rst_f8(mmu, 0x10),
            0xD8 => self.ret_f(mmu, Flag::C),
            0xD9 => self.reti(mmu),
            0xDA => self.jp_f_u16(mmu, Flag::C),
            0xDB => self.nop(),
            0xDC => self.call_f_u16(mmu, Flag::C),
            0xDD => self.nop(),
            0xDE => self.sbc_u8(mmu),
            0xDF => self.rst_f8(mmu, 0x18),
            0xE0 => self.ldh_iu8_r8(mmu, Register8::A),
            0xE1 => self.pop_r16(mmu, Register16::HL),
            0xE2 => self.ld_ir8_r8(mmu, Register8::C, Register8::A),
            0xE3 => self.nop(),
            0xE4 => self.nop(),
            0xE5 => self.push_r16(mmu, Register16::HL),
            0xE6 => self.and_u8(mmu),
            0xE7 => self.rst_f8(mmu, 0x20),
            0xE8 => self.add_r16_i8(mmu, Register16::SP),
            0xE9 => self.jp_r16(mmu, Register16::HL),
            0xEA => self.ld_iu16_r8(mmu, Register8::A),
            0xEB => self.nop(),
            0xEC => self.nop(),
            0xED => self.nop(),
            0xEE => self.xor_u8(mmu),
            0xEF => self.rst_f8(mmu, 0x28),
            0xF0 => self.ldh_r8_iu8(mmu, Register8::A),
            0xF1 => self.pop_r16(mmu, Register16::AF),
            0xF2 => self.ld_r8_ir8(mmu, Register8::A, Register8::C),
            0xF3 => self.di(),
            0xF4 => self.nop(),
            0xF5 => self.push_r16(mmu, Register16::AF),
            0xF6 => self.or_u8(mmu),
            0xF7 => self.rst_f8(mmu, 0x30),
            0xF8 => self.ld_r16_r16_i8(mmu, Register16::HL, Register16::SP),
            0xF9 => self.ld_r16_r16(Register16::SP, Register16::HL),
            0xFA => self.ld_r8_iu16(mmu, Register8::A),
            0xFB => self.ei(),
            0xFC => self.nop(),
            0xFD => self.nop(),
            0xFE => self.cp_u8(mmu),
            0xFF => self.rst_f8(mmu, 0x38),
        }
        self.step_cycles(OP_CYCLES[op as usize]);
        self.ops += 1;
    }

    fn decode_cb(&mut self, mmu: &mut Mmu,  op: u8) {
        match op {
            0x00 => self.rlc_r8(Register8::B),
            0x01 => self.rlc_r8(Register8::C),
            0x02 => self.rlc_r8(Register8::D),
            0x03 => self.rlc_r8(Register8::E),
            0x04 => self.rlc_r8(Register8::H),
            0x05 => self.rlc_r8(Register8::L),
            0x06 => self.rlc_ir16(mmu, Register16::HL),
            0x07 => self.rlc_r8(Register8::A),
            0x08 => self.rrc_r8(Register8::B),
            0x09 => self.rrc_r8(Register8::C),
            0x0A => self.rrc_r8(Register8::D),
            0x0B => self.rrc_r8(Register8::E),
            0x0C => self.rrc_r8(Register8::H),
            0x0D => self.rrc_r8(Register8::L),
            0x0E => self.rrc_ir16(mmu, Register16::HL),
            0x0F => self.rrc_r8(Register8::A),
            0x10 => self.rl_r8(Register8::B),
            0x11 => self.rl_r8(Register8::C),
            0x12 => self.rl_r8(Register8::D),
            0x13 => self.rl_r8(Register8::E),
            0x14 => self.rl_r8(Register8::H),
            0x15 => self.rl_r8(Register8::L),
            0x16 => self.rl_ir16(mmu, Register16::HL),
            0x17 => self.rl_r8(Register8::A),
            0x18 => self.rr_r8(Register8::B),
            0x19 => self.rr_r8(Register8::C),
            0x1A => self.rr_r8(Register8::D),
            0x1B => self.rr_r8(Register8::E),
            0x1C => self.rr_r8(Register8::H),
            0x1D => self.rr_r8(Register8::L),
            0x1E => self.rr_ir16(mmu, Register16::HL),
            0x1F => self.rr_r8(Register8::A),
            0x20 => self.sla_r8(Register8::B),
            0x21 => self.sla_r8(Register8::C),
            0x22 => self.sla_r8(Register8::D),
            0x23 => self.sla_r8(Register8::E),
            0x24 => self.sla_r8(Register8::H),
            0x25 => self.sla_r8(Register8::L),
            0x26 => self.sla_ir16(mmu, Register16::HL),
            0x27 => self.sla_r8(Register8::A),
            0x28 => self.sra_r8(Register8::B),
            0x29 => self.sra_r8(Register8::C),
            0x2A => self.sra_r8(Register8::D),
            0x2B => self.sra_r8(Register8::E),
            0x2C => self.sra_r8(Register8::H),
            0x2D => self.sra_r8(Register8::L),
            0x2E => self.sra_ir16(mmu, Register16::HL),
            0x2F => self.sra_r8(Register8::A),
            0x30 => self.swap_r8(Register8::B),
            0x31 => self.swap_r8(Register8::C),
            0x32 => self.swap_r8(Register8::D),
            0x33 => self.swap_r8(Register8::E),
            0x34 => self.swap_r8(Register8::H),
            0x35 => self.swap_r8(Register8::L),
            0x36 => self.swap_ir16(mmu, Register16::HL),
            0x37 => self.swap_r8(Register8::A),
            0x38 => self.srl_r8(Register8::B),
            0x39 => self.srl_r8(Register8::C),
            0x3A => self.srl_r8(Register8::D),
            0x3B => self.srl_r8(Register8::E),
            0x3C => self.srl_r8(Register8::H),
            0x3D => self.srl_r8(Register8::L),
            0x3E => self.srl_ir16(mmu, Register16::HL),
            0x3F => self.srl_r8(Register8::A),
            0x40 => self.bit_b_r8(0, Register8::B),
            0x41 => self.bit_b_r8(0, Register8::C),
            0x42 => self.bit_b_r8(0, Register8::D),
            0x43 => self.bit_b_r8(0, Register8::E),
            0x44 => self.bit_b_r8(0, Register8::H),
            0x45 => self.bit_b_r8(0, Register8::L),
            0x46 => self.bit_b_ir16(mmu, 0, Register16::HL),
            0x47 => self.bit_b_r8(0, Register8::A),
            0x48 => self.bit_b_r8(1, Register8::B),
            0x49 => self.bit_b_r8(1, Register8::C),
            0x4A => self.bit_b_r8(1, Register8::D),
            0x4B => self.bit_b_r8(1, Register8::E),
            0x4C => self.bit_b_r8(1, Register8::H),
            0x4D => self.bit_b_r8(1, Register8::L),
            0x4E => self.bit_b_ir16(mmu, 1, Register16::HL),
            0x4F => self.bit_b_r8(1, Register8::A),
            0x50 => self.bit_b_r8(2, Register8::B),
            0x51 => self.bit_b_r8(2, Register8::C),
            0x52 => self.bit_b_r8(2, Register8::D),
            0x53 => self.bit_b_r8(2, Register8::E),
            0x54 => self.bit_b_r8(2, Register8::H),
            0x55 => self.bit_b_r8(2, Register8::L),
            0x56 => self.bit_b_ir16(mmu, 2, Register16::HL),
            0x57 => self.bit_b_r8(2, Register8::A),
            0x58 => self.bit_b_r8(3, Register8::B),
            0x59 => self.bit_b_r8(3, Register8::C),
            0x5A => self.bit_b_r8(3, Register8::D),
            0x5B => self.bit_b_r8(3, Register8::E),
            0x5C => self.bit_b_r8(3, Register8::H),
            0x5D => self.bit_b_r8(3, Register8::L),
            0x5E => self.bit_b_ir16(mmu, 3, Register16::HL),
            0x5F => self.bit_b_r8(3, Register8::A),
            0x60 => self.bit_b_r8(4, Register8::B),
            0x61 => self.bit_b_r8(4, Register8::C),
            0x62 => self.bit_b_r8(4, Register8::D),
            0x63 => self.bit_b_r8(4, Register8::E),
            0x64 => self.bit_b_r8(4, Register8::H),
            0x65 => self.bit_b_r8(4, Register8::L),
            0x66 => self.bit_b_ir16(mmu, 4, Register16::HL),
            0x67 => self.bit_b_r8(4, Register8::A),
            0x68 => self.bit_b_r8(5, Register8::B),
            0x69 => self.bit_b_r8(5, Register8::C),
            0x6A => self.bit_b_r8(5, Register8::D),
            0x6B => self.bit_b_r8(5, Register8::E),
            0x6C => self.bit_b_r8(5, Register8::H),
            0x6D => self.bit_b_r8(5, Register8::L),
            0x6E => self.bit_b_ir16(mmu, 5, Register16::HL),
            0x6F => self.bit_b_r8(5, Register8::A),
            0x70 => self.bit_b_r8(6, Register8::B),
            0x71 => self.bit_b_r8(6, Register8::C),
            0x72 => self.bit_b_r8(6, Register8::D),
            0x73 => self.bit_b_r8(6, Register8::E),
            0x74 => self.bit_b_r8(6, Register8::H),
            0x75 => self.bit_b_r8(6, Register8::L),
            0x76 => self.bit_b_ir16(mmu, 6, Register16::HL),
            0x77 => self.bit_b_r8(6, Register8::A),
            0x78 => self.bit_b_r8(7, Register8::B),
            0x79 => self.bit_b_r8(7, Register8::C),
            0x7A => self.bit_b_r8(7, Register8::D),
            0x7B => self.bit_b_r8(7, Register8::E),
            0x7C => self.bit_b_r8(7, Register8::H),
            0x7D => self.bit_b_r8(7, Register8::L),
            0x7E => self.bit_b_ir16(mmu, 7, Register16::HL),
            0x7F => self.bit_b_r8(7, Register8::A),
            0x80 => self.res_b_r8(0, Register8::B),
            0x81 => self.res_b_r8(0, Register8::C),
            0x82 => self.res_b_r8(0, Register8::D),
            0x83 => self.res_b_r8(0, Register8::E),
            0x84 => self.res_b_r8(0, Register8::H),
            0x85 => self.res_b_r8(0, Register8::L),
            0x86 => self.res_b_ir16(mmu, 0, Register16::HL),
            0x87 => self.res_b_r8(0, Register8::A),
            0x88 => self.res_b_r8(1, Register8::B),
            0x89 => self.res_b_r8(1, Register8::C),
            0x8A => self.res_b_r8(1, Register8::D),
            0x8B => self.res_b_r8(1, Register8::E),
            0x8C => self.res_b_r8(1, Register8::H),
            0x8D => self.res_b_r8(1, Register8::L),
            0x8E => self.res_b_ir16(mmu, 1, Register16::HL),
            0x8F => self.res_b_r8(1, Register8::A),
            0x90 => self.res_b_r8(2, Register8::B),
            0x91 => self.res_b_r8(2, Register8::C),
            0x92 => self.res_b_r8(2, Register8::D),
            0x93 => self.res_b_r8(2, Register8::E),
            0x94 => self.res_b_r8(2, Register8::H),
            0x95 => self.res_b_r8(2, Register8::L),
            0x96 => self.res_b_ir16(mmu, 2, Register16::HL),
            0x97 => self.res_b_r8(2, Register8::A),
            0x98 => self.res_b_r8(3, Register8::B),
            0x99 => self.res_b_r8(3, Register8::C),
            0x9A => self.res_b_r8(3, Register8::D),
            0x9B => self.res_b_r8(3, Register8::E),
            0x9C => self.res_b_r8(3, Register8::H),
            0x9D => self.res_b_r8(3, Register8::L),
            0x9E => self.res_b_ir16(mmu, 3, Register16::HL),
            0x9F => self.res_b_r8(3, Register8::A),
            0xA0 => self.res_b_r8(4, Register8::B),
            0xA1 => self.res_b_r8(4, Register8::C),
            0xA2 => self.res_b_r8(4, Register8::D),
            0xA3 => self.res_b_r8(4, Register8::E),
            0xA4 => self.res_b_r8(4, Register8::H),
            0xA5 => self.res_b_r8(4, Register8::L),
            0xA6 => self.res_b_ir16(mmu, 4, Register16::HL),
            0xA7 => self.res_b_r8(4, Register8::A),
            0xA8 => self.res_b_r8(5, Register8::B),
            0xA9 => self.res_b_r8(5, Register8::C),
            0xAA => self.res_b_r8(5, Register8::D),
            0xAB => self.res_b_r8(5, Register8::E),
            0xAC => self.res_b_r8(5, Register8::H),
            0xAD => self.res_b_r8(5, Register8::L),
            0xAE => self.res_b_ir16(mmu, 5, Register16::HL),
            0xAF => self.res_b_r8(5, Register8::A),
            0xB0 => self.res_b_r8(6, Register8::B),
            0xB1 => self.res_b_r8(6, Register8::C),
            0xB2 => self.res_b_r8(6, Register8::D),
            0xB3 => self.res_b_r8(6, Register8::E),
            0xB4 => self.res_b_r8(6, Register8::H),
            0xB5 => self.res_b_r8(6, Register8::L),
            0xB6 => self.res_b_ir16(mmu, 6, Register16::HL),
            0xB7 => self.res_b_r8(6, Register8::A),
            0xB8 => self.res_b_r8(7, Register8::B),
            0xB9 => self.res_b_r8(7, Register8::C),
            0xBA => self.res_b_r8(7, Register8::D),
            0xBB => self.res_b_r8(7, Register8::E),
            0xBC => self.res_b_r8(7, Register8::H),
            0xBD => self.res_b_r8(7, Register8::L),
            0xBE => self.res_b_ir16(mmu, 7, Register16::HL),
            0xBF => self.res_b_r8(7, Register8::A),
            0xC0 => self.set_b_r8(0, Register8::B),
            0xC1 => self.set_b_r8(0, Register8::C),
            0xC2 => self.set_b_r8(0, Register8::D),
            0xC3 => self.set_b_r8(0, Register8::E),
            0xC4 => self.set_b_r8(0, Register8::H),
            0xC5 => self.set_b_r8(0, Register8::L),
            0xC6 => self.set_b_ir16(mmu, 0, Register16::HL),
            0xC7 => self.set_b_r8(0, Register8::A),
            0xC8 => self.set_b_r8(1, Register8::B),
            0xC9 => self.set_b_r8(1, Register8::C),
            0xCA => self.set_b_r8(1, Register8::D),
            0xCB => self.set_b_r8(1, Register8::E),
            0xCC => self.set_b_r8(1, Register8::H),
            0xCD => self.set_b_r8(1, Register8::L),
            0xCE => self.set_b_ir16(mmu, 1, Register16::HL),
            0xCF => self.set_b_r8(1, Register8::A),
            0xD0 => self.set_b_r8(2, Register8::B),
            0xD1 => self.set_b_r8(2, Register8::C),
            0xD2 => self.set_b_r8(2, Register8::D),
            0xD3 => self.set_b_r8(2, Register8::E),
            0xD4 => self.set_b_r8(2, Register8::H),
            0xD5 => self.set_b_r8(2, Register8::L),
            0xD6 => self.set_b_ir16(mmu, 2, Register16::HL),
            0xD7 => self.set_b_r8(2, Register8::A),
            0xD8 => self.set_b_r8(3, Register8::B),
            0xD9 => self.set_b_r8(3, Register8::C),
            0xDA => self.set_b_r8(3, Register8::D),
            0xDB => self.set_b_r8(3, Register8::E),
            0xDC => self.set_b_r8(3, Register8::H),
            0xDD => self.set_b_r8(3, Register8::L),
            0xDE => self.set_b_ir16(mmu, 3, Register16::HL),
            0xDF => self.set_b_r8(3, Register8::A),
            0xE0 => self.set_b_r8(4, Register8::B),
            0xE1 => self.set_b_r8(4, Register8::C),
            0xE2 => self.set_b_r8(4, Register8::D),
            0xE3 => self.set_b_r8(4, Register8::E),
            0xE4 => self.set_b_r8(4, Register8::H),
            0xE5 => self.set_b_r8(4, Register8::L),
            0xE6 => self.set_b_ir16(mmu, 4, Register16::HL),
            0xE7 => self.set_b_r8(4, Register8::A),
            0xE8 => self.set_b_r8(5, Register8::B),
            0xE9 => self.set_b_r8(5, Register8::C),
            0xEA => self.set_b_r8(5, Register8::D),
            0xEB => self.set_b_r8(5, Register8::E),
            0xEC => self.set_b_r8(5, Register8::H),
            0xED => self.set_b_r8(5, Register8::L),
            0xEE => self.set_b_ir16(mmu, 5, Register16::HL),
            0xEF => self.set_b_r8(5, Register8::A),
            0xF0 => self.set_b_r8(6, Register8::B),
            0xF1 => self.set_b_r8(6, Register8::C),
            0xF2 => self.set_b_r8(6, Register8::D),
            0xF3 => self.set_b_r8(6, Register8::E),
            0xF4 => self.set_b_r8(6, Register8::H),
            0xF5 => self.set_b_r8(6, Register8::L),
            0xF6 => self.set_b_ir16(mmu, 6, Register16::HL),
            0xF7 => self.set_b_r8(6, Register8::A),
            0xF8 => self.set_b_r8(7, Register8::B),
            0xF9 => self.set_b_r8(7, Register8::C),
            0xFA => self.set_b_r8(7, Register8::D),
            0xFB => self.set_b_r8(7, Register8::E),
            0xFC => self.set_b_r8(7, Register8::H),
            0xFD => self.set_b_r8(7, Register8::L),
            0xFE => self.set_b_ir16(mmu, 7, Register16::HL),
            0xFF => self.set_b_r8(7, Register8::A),
        }
        self.step_cycles(CB_OP_CYCLES[op as usize]);
    }

    fn step_cycles(&mut self, steps: u8) {
        self.cycles = self.cycles.wrapping_add(steps as u128);
    }

    fn consume_u8(&mut self, mmu: &mut Mmu) -> u8 {
        let r: u8 = mmu.read(self.reg.pc);
        self.reg.pc = self.reg.pc.wrapping_add(1);
        r
    }

    fn consume_i8(&mut self, mmu: &mut Mmu) -> i8 {
        let r: i8 = mmu.read(self.reg.pc) as i8;
        self.reg.pc = self.reg.pc.wrapping_add(1);
        r
    }

    fn consume_u16(&mut self, mmu: &mut Mmu) -> u16 {
        let r: u16 = mmu.read_16(self.reg.pc);
        self.reg.pc = self.reg.pc.wrapping_add(2);
        r
    }

    fn stack_push_u16(&mut self, mmu: &mut Mmu, value: u16) {
        let value_high: u8 = ((value & 0xff00) >> 8) as u8;
        let value_low: u8 = (value & 0x00ff) as u8;
        self.stack_push_u8(mmu, value_high);
        self.stack_push_u8(mmu, value_low);
    }

    fn stack_pop_u16(&mut self, mmu: &Mmu) -> u16 {
        let value_low: u8 = self.stack_pop_u8(mmu);
        let value_high: u8 = self.stack_pop_u8(mmu);
        ((value_high as u16) << 8) + (value_low as u16)
    }

    fn stack_push_u8(&mut self, mmu: &mut Mmu, value: u8) {
        self.reg.sp = self.reg.sp.wrapping_sub(1);
        mmu.write(self.reg.sp, value);
    }

    fn stack_pop_u8(&mut self, mmu: &Mmu) -> u8 {
        let value: u8 = mmu.read(self.reg.sp);
        self.reg.sp = self.reg.sp.wrapping_add(1);
        value
    }

    //
    // load 8-bit
    //

    fn handle_post_op(&mut self, reg: &Register16, op: &PostOp) {
        match op {
            PostOp::INC => self.reg.set16(reg, self.reg.get16(reg).wrapping_add(1)),
            PostOp::DEC => self.reg.set16(reg, self.reg.get16(reg).wrapping_sub(1)),
            _ => {}
        }
    }

    fn ld_r8_r8(&mut self, dest: Register8, src: Register8) {
        let value: u8 = self.reg.get8(&src);
        self.reg.set8(&dest, value);
    }

    fn ld_r8_u8(&mut self, mmu: &mut Mmu, dest: Register8) {
        let value: u8 = self.consume_u8(mmu);
        self.reg.set8(&dest, value);
    }

    fn ld_r8_ir16(&mut self, mmu: &mut Mmu, dest: Register8, src: Register16, post_op: PostOp) {
        let add: u16 = self.reg.get16(&src);
        let value: u8 = mmu.read(add);
        self.reg.set8(&dest, value);
        self.handle_post_op(&src, &post_op);
    }

    fn ld_ir16_r8(&mut self, mmu: &mut Mmu, dest: Register16, src: Register8, post_op: PostOp) {
        let value: u8 = self.reg.get8(&src);
        let add: u16 = self.reg.get16(&dest);
        mmu.write(add, value);
        self.handle_post_op(&dest, &post_op);
    }

    fn ld_ir16_u8(&mut self, mmu: &mut Mmu, dest: Register16) {
        let value: u8 = self.consume_u8(mmu);
        let add: u16 = self.reg.get16(&dest);
        mmu.write(add, value);
    }

    fn ld_r8_iu16(&mut self, mmu: &mut Mmu, dest: Register8) {
        let add: u16 = self.consume_u16(mmu);
        let value: u8 = mmu.read(add);
        self.reg.set8(&dest, value);
    }

    fn ld_iu16_r8(&mut self, mmu: &mut Mmu, src: Register8) {
        let value: u8 = self.reg.get8(&src);
        let add: u16 = self.consume_u16(mmu);
        mmu.write(add, value);
    }

    fn ld_r8_ir8(&mut self, mmu: &mut Mmu, dest: Register8, src: Register8) {
        let add_low: u8 = self.reg.get8(&src);
        let add: u16 = 0xff00 | (add_low as u16);
        let value: u8 = mmu.read(add);
        self.reg.set8(&dest, value);
    }

    fn ld_ir8_r8(&mut self, mmu: &mut Mmu, dest: Register8, src: Register8) {
        let value: u8 = self.reg.get8(&src);
        let add_low: u8 = self.reg.get8(&dest);
        let add: u16 = 0xff00 | (add_low as u16);
        mmu.write(add, value);
    }

    fn ldh_r8_iu8(&mut self, mmu: &mut Mmu, dest: Register8) {
        let add_low: u8 = self.consume_u8(mmu);
        let add: u16 = 0xff00 | (add_low as u16);
        let value: u8 = mmu.read(add);
        self.reg.set8(&dest, value);
    }

    fn ldh_iu8_r8(&mut self, mmu: &mut Mmu, src: Register8) {
        let value: u8 = self.reg.get8(&src);
        let add_low: u8 = self.consume_u8(mmu);
        let add: u16 = 0xff00 | (add_low as u16);
        mmu.write(add, value);
    }

    //
    // load 16-bit
    //

    fn ld_r16_u16(&mut self, mmu: &mut Mmu, dest: Register16) {
        let value: u16 = self.consume_u16(mmu);
        self.reg.set16(&dest, value);
    }

    fn ld_iu16_r16(&mut self, mmu: &mut Mmu, src: Register16) {
        let value: u16 = self.reg.get16(&src);
        let value_low: u8 = (value & 0x00ff) as u8;
        let value_high: u8 = ((value & 0xff00) >> 8) as u8;
        let add: u16 = self.consume_u16(mmu);
        mmu.write(add, value_low);
        mmu.write(add + 1, value_high);
    }

    fn ld_r16_r16(&mut self, dest: Register16, src: Register16) {
        let value: u16 = self.reg.get16(&src);
        self.reg.set16(&dest, value);
    }

    fn push_r16(&mut self, mmu: &mut Mmu, src: Register16) {
        let value: u16 = self.reg.get16(&src);
        self.stack_push_u16(mmu, value);
    }

    fn pop_r16(&mut self, mmu: &Mmu, dest: Register16) {
        let value: u16 = self.stack_pop_u16(mmu);
        self.reg.set16(&dest, value);
    }

    //
    // alu 8-bit
    //

    fn sum8_flags(&mut self, op1: u8, op2: u8, c: bool, h: bool) -> u8 {
        let res: u8 = op1.wrapping_add(op2);

        if c {
            if (0xff - op1) < op2 {
                self.reg.setf(&Flag::C);
            } else {
                self.reg.unsetf(&Flag::C);
            }
        }

        if h {
            if (0x10 - (op1 & 0x0f)) <= (op2 & 0x0f) {
                self.reg.setf(&Flag::H);
            } else {
                self.reg.unsetf(&Flag::H);
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

    fn add_ir16(&mut self, mmu: &mut Mmu, reg: Register16) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let add: u16 = self.reg.get16(&reg);
        let value: u8 = mmu.read(add);
        let res: u8 = self.sum8_flags(accumulator, value, true, true);
        self.reg.set8(&Register8::A, res);
    }

    fn add_u8(&mut self, mmu: &mut Mmu) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let value: u8 = self.consume_u8(mmu);
        let res: u8 = self.sum8_flags(accumulator, value, true, true);
        self.reg.set8(&Register8::A, res);
    }

    fn adc_flags(&mut self, value: u8) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let c_value: u8 = self.reg.getf(&Flag::C);
        self.reg.unsetf(&Flag::C);
        self.reg.unsetf(&Flag::H);
        let mut res: u8 = self.sum8_flags(accumulator, c_value, true, true);
        res = self.sum8_flags(
            res,
            value,
            self.reg.getf(&Flag::C) != 1,
            self.reg.getf(&Flag::H) != 1,
            );
        self.reg.set8(&Register8::A, res);
    }

    fn adc_r8(&mut self, reg: Register8) {
        /*let reg_value: u8 = self.reg.get8(&reg);
          let c_value: u8 = self.reg.getf(&Flag::C);
          let value_carry: u8 = reg_value.wrapping_add(c_value);

          let accumulator: u8 = self.reg.get8(&Register8::A);

          let res: u8 = self.sum8_flags(accumulator, value_carry, true, true);
          self.reg.set8(&Register8::A, res);*/
        let value: u8 = self.reg.get8(&reg);
        self.adc_flags(value);
    }

    fn adc_ir16(&mut self, mmu: &mut Mmu, reg: Register16) {
        /*let accumulator: u8 = self.reg.get8(&Register8::A);
          let c_value: u8 = self.reg.getf(&Flag::C);

          let add: u16 = self.reg.get16(&reg);
          let value: u8 = mmu.read(add);
          let value_carry: u8 = value.wrapping_add(c_value);

          let res: u8 = self.sum8_flags(accumulator, value_carry, true, true);
          self.reg.set8(&Register8::A, res);*/

        let add: u16 = self.reg.get16(&reg);
        let value: u8 = mmu.read(add);
        self.adc_flags(value);
    }

    fn adc_u8(&mut self, mmu: &mut Mmu) {
        /*let accumulator: u8 = self.reg.get8(&Register8::A);
          let c_value: u8 = self.reg.getf(&Flag::C);

          let value: u8 = self.consume_u8(mmu);
          let value_carry: u8 = value.wrapping_add(c_value);

          let res: u8 = self.sum8_flags(accumulator, value_carry, true, true);*/

        let value: u8 = self.consume_u8(mmu);
        self.adc_flags(value);
    }

    fn sub_r8(&mut self, reg: Register8) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let value: u8 = self.reg.get8(&reg);
        let res: u8 = self.sub8_flags(accumulator, value, true, true);
        self.reg.set8(&Register8::A, res);
        let value: u8 = self.reg.get8(&reg);
    }

    fn sub_ir16(&mut self, mmu: &mut Mmu, reg: Register16) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let add: u16 = self.reg.get16(&reg);
        let value: u8 = mmu.read(add);
        let res: u8 = self.sub8_flags(accumulator, value, true, true);
        self.reg.set8(&Register8::A, res);
    }

    fn sub_u8(&mut self, mmu: &mut Mmu) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let value: u8 = self.consume_u8(mmu);
        let res: u8 = self.sub8_flags(accumulator, value, true, true);
        self.reg.set8(&Register8::A, res);
    }

    fn sbc_flags(&mut self, value: u8) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let c_value: u8 = self.reg.getf(&Flag::C);
        self.reg.unsetf(&Flag::C);
        self.reg.unsetf(&Flag::H);
        let mut res: u8 = self.sub8_flags(accumulator, c_value, true, true);
        res = self.sub8_flags(
            res,
            value,
            self.reg.getf(&Flag::C) != 1,
            self.reg.getf(&Flag::H) != 1,
            );
        self.reg.set8(&Register8::A, res);
    }

    fn sbc_r8(&mut self, reg: Register8) {
        /*let c_value: u8 = self.reg.getf(&Flag::C);
          let reg_value: u8 = self.reg.get8(&reg);
          let value_carry: u8 = reg_value.wrapping_sub(c_value);

          let accumulator: u8 = self.reg.get8(&Register8::A);

          let res: u8 = self.sub8_flags(accumulator, value_carry, true, true);
          self.reg.set8(&Register8::A, res);*/
        let value: u8 = self.reg.get8(&reg);
        self.sbc_flags(value);
    }

    fn sbc_ir16(&mut self, mmu: &mut Mmu, reg: Register16) {
        /*let add: u16 = self.reg.get16(&reg);
          let value: u8 = mmu.read(add);
          let c_value: u8 = self.reg.getf(&Flag::C);
          let value_carry: u8 = value.wrapping_sub(c_value);

          let accumulator: u8 = self.reg.get8(&Register8::A);

          let res: u8 = self.sub8_flags(accumulator, value_carry, true, true);
          self.reg.set8(&Register8::A, res);*/
        let add: u16 = self.reg.get16(&reg);
        let value: u8 = mmu.read(add);
        self.sbc_flags(value);
    }

    fn sbc_u8(&mut self, mmu: &mut Mmu) {
        /*let value: u8 = self.consume_u8(mmu);
          let c_value: u8 = self.reg.getf(&Flag::C);
          let value_carry: u8 = value.wrapping_sub(c_value);


          let accumulator: u8 = self.reg.get8(&Register8::A);
          let res: u8 = self.sub8_flags(accumulator, value_carry, true, true);
          self.reg.set8(&Register8::A, res);*/
        let value: u8 = self.consume_u8(mmu);
        self.sbc_flags(value);
    }

    fn cp_r8(&mut self, reg: Register8) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let value: u8 = self.reg.get8(&reg);
        self.sub8_flags(accumulator, value, true, true);
    }

    fn cp_ir16(&mut self, mmu: &mut Mmu, reg: Register16) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let add: u16 = self.reg.get16(&reg);
        let value: u8 = mmu.read(add);
        self.sub8_flags(accumulator, value, true, true);
    }

    fn cp_u8(&mut self, mmu: &mut Mmu) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let value: u8 = self.consume_u8(mmu);
        self.sub8_flags(accumulator, value, true, true);
    }

    fn inc_r8(&mut self, reg: Register8) {
        let value: u8 = self.reg.get8(&reg);
        let res: u8 = self.sum8_flags(value, 1, false, true);
        self.reg.set8(&reg, res);
    }

    fn inc_ir16(&mut self, mmu: &mut Mmu, reg: Register16) {
        let add: u16 = self.reg.get16(&reg);
        let value: u8 = mmu.read(add);
        let res: u8 = self.sum8_flags(value, 1, false, true);
        mmu.write(add, res);
    }

    fn dec_r8(&mut self, reg: Register8) {
        let value: u8 = self.reg.get8(&reg);
        let res: u8 = self.sub8_flags(value, 1, false, true);
        self.reg.set8(&reg, res);
    }

    fn dec_ir16(&mut self, mmu: &mut Mmu, reg: Register16) {
        let add: u16 = self.reg.get16(&reg);
        let value: u8 = mmu.read(add);
        let res: u8 = self.sub8_flags(value, 1, false, true);
        mmu.write(add, res);
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

    fn and_ir16(&mut self, mmu: &mut Mmu, reg: Register16) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let add: u16 = self.reg.get16(&reg);
        let value: u8 = mmu.read(add);
        let res: u8 = self.and8_flags(accumulator, value);
        self.reg.set8(&Register8::A, res);
    }

    fn and_u8(&mut self, mmu: &mut Mmu) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let value: u8 = self.consume_u8(mmu);
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

    fn or_ir16(&mut self, mmu: &mut Mmu, reg: Register16) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let add: u16 = self.reg.get16(&reg);
        let value: u8 = mmu.read(add);
        let res: u8 = self.or8_flags(accumulator, value);
        self.reg.set8(&Register8::A, res);
    }

    fn or_u8(&mut self, mmu: &mut Mmu) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let value: u8 = self.consume_u8(mmu);
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

    fn xor_ir16(&mut self, mmu: &mut Mmu, reg: Register16) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let add: u16 = self.reg.get16(&reg);
        let value: u8 = mmu.read(add);
        let res: u8 = self.xor8_flags(accumulator, value);
        self.reg.set8(&Register8::A, res);
    }

    fn xor_u8(&mut self, mmu: &mut Mmu) {
        let accumulator: u8 = self.reg.get8(&Register8::A);
        let value: u8 = self.consume_u8(mmu);
        let res: u8 = self.xor8_flags(accumulator, value);
        self.reg.set8(&Register8::A, res);
    }

    fn ccf(&mut self) {
        self.reg.unsetf(&Flag::N);
        self.reg.unsetf(&Flag::H);

        if self.reg.getf(&Flag::C) == 0 {
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

    //
    // alu 16-bit
    //

    fn inc_r16(&mut self, reg: Register16) {
        let value: u16 = self.reg.get16(&reg);
        let value_inc: u16 = value.wrapping_add(1);
        self.reg.set16(&reg, value_inc);
    }

    fn dec_r16(&mut self, reg: Register16) {
        let value: u16 = self.reg.get16(&reg);
        let value_dec: u16 = value.wrapping_sub(1);
        self.reg.set16(&reg, value_dec);
    }

    fn add_r16_r16(&mut self, dest: Register16, src: Register16) {
        let op1: u16 = self.reg.get16(&dest);
        let op2: u16 = self.reg.get16(&src);

        let res: u16 = op1.wrapping_add(op2);

        if (0x0fff - (op1 & 0x0fff)) < (op2 & 0x0fff) {
            self.reg.setf(&Flag::H);
        } else {
            self.reg.unsetf(&Flag::H);
        }

        if (0xffff - op1) < op2 {
            self.reg.setf(&Flag::C);
        } else {
            self.reg.unsetf(&Flag::C);
        }

        self.reg.unsetf(&Flag::N);
        self.reg.set16(&dest, res);
    }

    fn signed_sum_flags(&mut self, op1: u16, op2: i8) -> u16 {
        let res: u16 = self.signed_sum(op1, op2);

        /*if op2 >= 0 {
          if (0x000f - (op1 & 0x000f)) < ((op2.abs() as u16) & 0x0000f) {
          self.reg.setf(&Flag::H);
          } else {
          self.reg.unsetf(&Flag::H);
          }

          if (0x00ff - (op1 & 0x0ff)) < (op2.abs() as u16) {
          self.reg.setf(&Flag::C);
          } else {
          self.reg.unsetf(&Flag::C);
          }
          } else {
          if (op1 & 0x000f) < ((op2.abs() & 0x000f) as u16) {
          self.reg.setf(&Flag::H);
          } else {
          self.reg.unsetf(&Flag::H);
          }

          if (op1 & 0x00ff) < (op2.abs() as u16) {
          self.reg.setf(&Flag::C);
          } else {
          self.reg.unsetf(&Flag::C);
          }
          }*/
        if (0x000f - ((op1 & 0x000f) as u8)) < ((op2 as u8) & 0x0000f) {
            self.reg.setf(&Flag::H);
        } else {
            self.reg.unsetf(&Flag::H);
        }

        if (0x00ff - ((op1 & 0x0ff) as u8)) < (op2 as u8) {
            self.reg.setf(&Flag::C);
        } else {
            self.reg.unsetf(&Flag::C);
        }

        self.reg.unsetf(&Flag::Z);
        self.reg.unsetf(&Flag::N);
        res
    }

    fn add_r16_i8(&mut self, mmu: &mut Mmu, reg: Register16) {
        let reg_value: u16 = self.reg.get16(&reg);
        let op: i8 = self.consume_i8(mmu);
        let res: u16 = self.signed_sum_flags(reg_value, op);
        self.reg.set16(&reg, res);
    }

    fn ld_r16_r16_i8(&mut self, mmu: &mut Mmu, dest: Register16, src: Register16) {
        let op1: u16 = self.reg.get16(&src);
        let op2: i8 = self.consume_i8(mmu);
        let res: u16 = self.signed_sum_flags(op1, op2);
        self.reg.set16(&dest, res);
    }

    //
    // Control Flow
    //

    fn jp_u16(&mut self, mmu: &mut Mmu) {
        let add: u16 = self.consume_u16(mmu);
        self.reg.pc = add;
    }

    fn jp_r16(&mut self, mmu: &mut Mmu, reg: Register16) {
        self.reg.pc = self.reg.get16(&reg);
    }

    fn jp_f_u16(&mut self, mmu: &mut Mmu, flag: Flag) {
        let add: u16 = self.consume_u16(mmu);

        if self.reg.getf(&flag) == 1 {
            self.step_cycles(4);
            self.reg.pc = add;
        }
    }

    fn signed_sum(&self, op1: u16, op2: i8) -> u16 {
        (op1 as i32).wrapping_add(op2 as i32) as u16
    }

    fn get_relative_add(&self, offset: i8) -> u16 {
        self.signed_sum(self.reg.pc, offset)
    }

    fn jr_i8(&mut self, mmu: &mut Mmu) {
        let offset: i8 = self.consume_i8(mmu);
        let add: u16 = self.get_relative_add(offset);
        self.reg.pc = add;
    }

    fn jr_f_i8(&mut self, mmu: &mut Mmu, flag: Flag) {
        let offset: i8 = self.consume_i8(mmu);
        let add: u16 = self.get_relative_add(offset);

        if self.reg.getf(&flag) == 1 {
            self.step_cycles(4);
            self.reg.pc = add;
        }
    }

    fn call_u16(&mut self, mmu: &mut Mmu) {
        let add: u16 = self.consume_u16(mmu);
        self.stack_push_u16(mmu, self.reg.pc);
        self.reg.pc = add;
    }

    fn call_f_u16(&mut self, mmu: &mut Mmu, flag: Flag) {
        let add: u16 = self.consume_u16(mmu);

        if self.reg.getf(&flag) == 1 {
            self.step_cycles(12);
            self.stack_push_u16(mmu, self.reg.pc);
            self.reg.pc = add;
        }
    }

    fn ret(&mut self, mmu: &Mmu) {
        self.reg.pc = self.stack_pop_u16(mmu);
    }

    fn ret_f(&mut self, mmu: &Mmu, flag: Flag) {
        if self.reg.getf(&flag) == 1 {
            self.step_cycles(12);
            self.reg.pc = self.stack_pop_u16(mmu);
        }
    }

    fn rst_f8(&mut self, mmu: &mut Mmu, fixed: u8) {
        let add: u16 = fixed as u16;
        self.stack_push_u16(mmu, self.reg.pc);
        self.reg.pc = add;
    }

    fn di(&mut self) {
        self.ime = Ime::DISABLED;
    }

    fn ei(&mut self) {
        self.ime = Ime::PENDING;
    }

    fn stop(&mut self) {
        panic!("stop not implemented");
    }

    //TODO: halt bug
    fn halt(&mut self) {
        self.halted = true;
    }

    fn reti(&mut self, mmu: &Mmu) {
        self.ime = Ime::ENABLED;
        self.reg.pc = self.stack_pop_u16(mmu);
    }

    //
    // Misc
    //

    fn nop(&mut self) {}

    //
    // CB
    //

    fn setb8(bit: u8, value: u8) -> u8 {
        value | (1 << bit)
    }

    fn set_b_r8(&mut self, bit: u8, reg: Register8) {
        let val: u8 = self.reg.get8(&reg);
        let res: u8 = Cpu::setb8(bit, val);
        self.reg.set8(&reg, res);
    }

    fn set_b_ir16(&mut self, mmu: &mut Mmu, bit: u8, reg: Register16) {
        let add: u16 = self.reg.get16(&reg);
        let val: u8 = mmu.read(add);
        let res: u8 = Cpu::setb8(bit, val);
        mmu.write(add, res);
    }

    fn resetb8(bit: u8, value: u8) -> u8 {
        value & !(1 << bit)
    }

    fn res_b_r8(&mut self, bit: u8, reg: Register8) {
        let val: u8 = self.reg.get8(&reg);
        let res: u8 = Cpu::resetb8(bit, val);
        self.reg.set8(&reg, res);
    }

    fn res_b_ir16(&mut self, mmu: &mut Mmu, bit: u8, reg: Register16) {
        let add: u16 = self.reg.get16(&reg);
        let val: u8 = mmu.read(add);
        let res: u8 = Cpu::resetb8(bit, val);
        mmu.write(add, res);
    }

    fn testb8_flags(&mut self, bit: u8, value: u8) {
        let res: u8 = (value & (1u8 << bit)) >> bit;
        if res == 0 {
            self.reg.setf(&Flag::Z);
        } else {
            self.reg.unsetf(&Flag::Z);
        }
        self.reg.unsetf(&Flag::N);
        self.reg.setf(&Flag::H);
    }

    fn bit_b_r8(&mut self, bit: u8, reg: Register8) {
        let value: u8 = self.reg.get8(&reg);
        self.testb8_flags(bit, value);
    }

    fn bit_b_ir16(&mut self, mmu: &mut Mmu, bit: u8, reg: Register16) {
        let add: u16 = self.reg.get16(&reg);
        let value: u8 = mmu.read(add);
        self.testb8_flags(bit, value);
    }

    fn swap8_flags(&mut self, value: u8) -> u8 {
        let high: u8 = (value & 0xf0) >> 4;
        let low: u8 = value & 0x0f;
        let res: u8 = (low << 4) | high;

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

    fn swap_r8(&mut self, reg: Register8) {
        let val: u8 = self.reg.get8(&reg);
        let res: u8 = self.swap8_flags(val);
        self.reg.set8(&reg, res);
    }

    fn swap_ir16(&mut self, mmu: &mut Mmu, reg: Register16) {
        let add: u16 = self.reg.get16(&reg);
        let val: u8 = mmu.read(add);
        let res: u8 = self.swap8_flags(val);
        mmu.write(add, res);
    }

    fn shiftrl8_flags(&mut self, value: u8) -> u8 {
        let res: u8 = value >> 1;

        if (value & 0x01) == 1 {
            self.reg.setf(&Flag::C);
        } else {
            self.reg.unsetf(&Flag::C);
        }

        if res == 0 {
            self.reg.setf(&Flag::Z);
        } else {
            self.reg.unsetf(&Flag::Z);
        }

        self.reg.unsetf(&Flag::N);
        self.reg.unsetf(&Flag::H);
        res
    }

    fn srl_r8(&mut self, reg: Register8) {
        let value: u8 = self.reg.get8(&reg);
        let res: u8 = self.shiftrl8_flags(value);
        self.reg.set8(&reg, res);
    }

    fn srl_ir16(&mut self, mmu: &mut Mmu, reg: Register16) {
        let add: u16 = self.reg.get16(&reg);
        let value: u8 = mmu.read(add);
        let res: u8 = self.shiftrl8_flags(value);
        mmu.write(add, res);
    }

    fn shiftra8_flags(&mut self, value: u8) -> u8 {
        let mut res: u8 = value >> 1;

        if (value & 0x80) != 0 {
            res |= 0x80;
        }

        if (value & 0x01) == 1 {
            self.reg.setf(&Flag::C);
        } else {
            self.reg.unsetf(&Flag::C);
        }

        if res == 0 {
            self.reg.setf(&Flag::Z);
        } else {
            self.reg.unsetf(&Flag::Z);
        }

        self.reg.unsetf(&Flag::N);
        self.reg.unsetf(&Flag::H);
        res
    }

    fn sra_r8(&mut self, reg: Register8) {
        let value: u8 = self.reg.get8(&reg);
        let res: u8 = self.shiftra8_flags(value);
        self.reg.set8(&reg, res);
    }

    fn sra_ir16(&mut self, mmu: &mut Mmu, reg: Register16) {
        let add: u16 = self.reg.get16(&reg);
        let value: u8 = mmu.read(add);
        let res: u8 = self.shiftra8_flags(value);
        mmu.write(add, res);
    }

    fn shiftla8_flags(&mut self, value: u8) -> u8 {
        let res: u8 = value << 1;

        if (value & 0x80) != 0 {
            self.reg.setf(&Flag::C);
        } else {
            self.reg.unsetf(&Flag::C);
        }

        if res == 0 {
            self.reg.setf(&Flag::Z);
        } else {
            self.reg.unsetf(&Flag::Z);
        }

        self.reg.unsetf(&Flag::N);
        self.reg.unsetf(&Flag::H);
        res
    }

    fn sla_r8(&mut self, reg: Register8) {
        let value: u8 = self.reg.get8(&reg);
        let res: u8 = self.shiftla8_flags(value);
        self.reg.set8(&reg, res);
    }

    fn sla_ir16(&mut self, mmu: &mut Mmu, reg: Register16) {
        let add: u16 = self.reg.get16(&reg);
        let value: u8 = mmu.read(add);
        let res: u8 = self.shiftla8_flags(value);
        mmu.write(add, res);
    }

    fn rotater8_flags(&mut self, value: u8) -> u8 {
        let mut res: u8 = value >> 1;

        if self.reg.getf(&Flag::C) == 1 {
            res |= 0x80;
        }

        if (value & 0x01) == 1 {
            self.reg.setf(&Flag::C);
        } else {
            self.reg.unsetf(&Flag::C);
        }

        if res == 0 {
            self.reg.setf(&Flag::Z);
        } else {
            self.reg.unsetf(&Flag::Z);
        }

        self.reg.unsetf(&Flag::N);
        self.reg.unsetf(&Flag::H);
        res
    }

    fn rr_r8(&mut self, reg: Register8) {
        let value: u8 = self.reg.get8(&reg);
        let res: u8 = self.rotater8_flags(value);
        self.reg.set8(&reg, res);
    }

    fn rr_ir16(&mut self, mmu: &mut Mmu, reg: Register16) {
        let add: u16 = self.reg.get16(&reg);
        let value: u8 = mmu.read(add);
        let res: u8 = self.rotater8_flags(value);
        mmu.write(add, res);
    }

    fn rotatel8_flags(&mut self, value: u8) -> u8 {
        let mut res: u8 = value << 1;

        if self.reg.getf(&Flag::C) == 1 {
            res |= 0x01;
        }

        if (value & 0x80) != 0 {
            self.reg.setf(&Flag::C);
        } else {
            self.reg.unsetf(&Flag::C);
        }

        if res == 0 {
            self.reg.setf(&Flag::Z);
        } else {
            self.reg.unsetf(&Flag::Z);
        }

        self.reg.unsetf(&Flag::N);
        self.reg.unsetf(&Flag::H);
        res
    }

    fn rl_r8(&mut self, reg: Register8) {
        let value: u8 = self.reg.get8(&reg);
        let res: u8 = self.rotatel8_flags(value);
        self.reg.set8(&reg, res);
    }

    fn rl_ir16(&mut self, mmu: &mut Mmu, reg: Register16) {
        let add: u16 = self.reg.get16(&reg);
        let value: u8 = mmu.read(add);
        let res: u8 = self.rotatel8_flags(value);
        mmu.write(add, res);
    }

    fn rotaterc8_flags(&mut self, value: u8) -> u8 {
        let mut res: u8 = value >> 1;

        if (value & 0x01) == 1 {
            self.reg.setf(&Flag::C);
            res |= 0x80;
        } else {
            self.reg.unsetf(&Flag::C);
        }

        if res == 0 {
            self.reg.setf(&Flag::Z);
        } else {
            self.reg.unsetf(&Flag::Z);
        }

        self.reg.unsetf(&Flag::N);
        self.reg.unsetf(&Flag::H);
        res
    }

    fn rrc_r8(&mut self, reg: Register8) {
        let value: u8 = self.reg.get8(&reg);
        let res: u8 = self.rotaterc8_flags(value);
        self.reg.set8(&reg, res);
    }

    fn rrc_ir16(&mut self, mmu: &mut Mmu, reg: Register16) {
        let add: u16 = self.reg.get16(&reg);
        let value: u8 = mmu.read(add);
        let res: u8 = self.rotaterc8_flags(value);
        mmu.write(add, res);
    }

    fn rotatelc8_flags(&mut self, value: u8) -> u8 {
        let mut res: u8 = value << 1;

        if (value & 0x80) != 0 {
            self.reg.setf(&Flag::C);
            res |= 0x01;
        } else {
            self.reg.unsetf(&Flag::C);
        }

        if res == 0 {
            self.reg.setf(&Flag::Z);
        } else {
            self.reg.unsetf(&Flag::Z);
        }

        self.reg.unsetf(&Flag::N);
        self.reg.unsetf(&Flag::H);
        res
    }

    fn rlc_r8(&mut self, reg: Register8) {
        let value: u8 = self.reg.get8(&reg);
        let res: u8 = self.rotatelc8_flags(value);
        self.reg.set8(&reg, res);
    }

    fn rlc_ir16(&mut self, mmu: &mut Mmu, reg: Register16) {
        let add: u16 = self.reg.get16(&reg);
        let value: u8 = mmu.read(add);
        let res: u8 = self.rotatelc8_flags(value);
        mmu.write(add, res);
    }

    fn rra(&mut self) {
        let value: u8 = self.reg.get8(&Register8::A);
        let mut res: u8 = value >> 1;

        if self.reg.getf(&Flag::C) == 1 {
            res |= 0x80;
        }

        if (value & 0x01) == 1 {
            self.reg.setf(&Flag::C);
        } else {
            self.reg.unsetf(&Flag::C);
        }

        self.reg.unsetf(&Flag::Z);
        self.reg.unsetf(&Flag::N);
        self.reg.unsetf(&Flag::H);

        self.reg.set8(&Register8::A, res);
    }

    fn rlca(&mut self) {
        let value: u8 = self.reg.get8(&Register8::A);
        let mut res: u8 = value << 1;

        if (value & 0x80) != 0 {
            self.reg.setf(&Flag::C);
            res |= 0x01;
        } else {
            self.reg.unsetf(&Flag::C);
        }

        self.reg.unsetf(&Flag::Z);
        self.reg.unsetf(&Flag::N);
        self.reg.unsetf(&Flag::H);

        self.reg.set8(&Register8::A, res);
    }

    fn rla(&mut self) {
        let value: u8 = self.reg.get8(&Register8::A);
        let mut res: u8 = value << 1;

        if self.reg.getf(&Flag::C) == 1 {
            res |= 0x01;
        }

        if (value & 0x80) != 0 {
            self.reg.setf(&Flag::C);
        } else {
            self.reg.unsetf(&Flag::C);
        }

        self.reg.unsetf(&Flag::Z);
        self.reg.unsetf(&Flag::N);
        self.reg.unsetf(&Flag::H);

        self.reg.set8(&Register8::A, res);
    }

    fn rrca(&mut self) {
        let value: u8 = self.reg.get8(&Register8::A);
        let mut res: u8 = value >> 1;

        if (value & 0x01) == 1 {
            self.reg.setf(&Flag::C);
            res |= 0x80;
        } else {
            self.reg.unsetf(&Flag::C);
        }

        self.reg.unsetf(&Flag::Z);
        self.reg.unsetf(&Flag::N);
        self.reg.unsetf(&Flag::H);

        self.reg.set8(&Register8::A, res);
    }

    fn daa(&mut self) {
        let mut res: u8 = self.reg.get8(&Register8::A);

        if self.reg.getf(&Flag::N) == 0 {
            if res > 0x99 || self.reg.getf(&Flag::C) == 1 {
                res = res.wrapping_add(0x60);
                self.reg.setf(&Flag::C);
            }

            if (res & 0x0f) > 9 || self.reg.getf(&Flag::H) == 1 {
                res = res.wrapping_add(0x06);
            }
        } else {
            if self.reg.getf(&Flag::C) == 1 {
                res = res.wrapping_sub(0x60);
            }

            if self.reg.getf(&Flag::H) == 1 {
                res = res.wrapping_sub(0x06);
            }
        }

        if res == 0 {
            self.reg.setf(&Flag::Z);
        } else {
            self.reg.unsetf(&Flag::Z);
        }

        self.reg.unsetf(&Flag::H);
        self.reg.set8(&Register8::A, res);
    }

    //
    // TODO
    //

    pub fn dump2(&mut self, mmu: &mut Mmu) {
        println!(
            "A:{:02X?} F:{:02X?} B:{:02X?} C:{:02X?} D:{:02X?} E:{:02X?} H:{:02X?} L:{:02X?} SP:{:04X?} PC:{:04X?} PCMEM:{:02X?},{:02X?},{:02X?},{:02X?}",
            self.reg.get8(&Register8::A),
            self.reg.get8(&Register8::F),
            self.reg.get8(&Register8::B),
            self.reg.get8(&Register8::C),
            self.reg.get8(&Register8::D),
            self.reg.get8(&Register8::E),
            self.reg.get8(&Register8::H),
            self.reg.get8(&Register8::L),
            self.reg.get16(&Register16::SP),
            self.reg.get16(&Register16::PC),
            mmu.read(self.reg.pc),
            mmu.read(self.reg.pc.wrapping_add(1)),
            mmu.read(self.reg.pc.wrapping_add(2)),
            mmu.read(self.reg.pc.wrapping_add(3)),
            );
    }

    fn getimen(&self) -> u8 {
        match self.ime {
            Ime::ENABLED => 1,
            _ => 0,
        }
    }

    pub fn dump3(&mut self, mmu: &mut Mmu) {
        println!(
            "Step:{} Cycles:{} PC:{:04X?} SP:{:04X?} AF:{:04X?} BC:{:04X?} DE:{:04X?} HL:{:04X?} IME:{} PCMEM:{:02X?},{:02X?},{:02X?},{:02X?}",
            self.ops,
            self.cycles,
            self.reg.pc,
            self.reg.sp,
            self.reg.get16(&Register16::AF),
            self.reg.get16(&Register16::BC),
            self.reg.get16(&Register16::DE),
            self.reg.get16(&Register16::HL),
            self.getimen(),
            mmu.read(self.reg.pc),
            mmu.read(self.reg.pc.wrapping_add(1)),
            mmu.read(self.reg.pc.wrapping_add(2)),
            mmu.read(self.reg.pc.wrapping_add(3)),
            );
    }
    pub fn dump4(&mut self, mmu: &mut Mmu) {
        println!(
            "PC:{:04X?} SP:{:04X?} AF:{:04X?} BC:{:04X?} DE:{:04X?} HL:{:04X?} IME:{} PCMEM:{:02X?},{:02X?},{:02X?},{:02X?}",
            self.reg.pc,
            self.reg.sp,
            self.reg.get16(&Register16::AF),
            self.reg.get16(&Register16::BC),
            self.reg.get16(&Register16::DE),
            self.reg.get16(&Register16::HL),
            self.getimen(),
            mmu.read(self.reg.pc),
            mmu.read(self.reg.pc.wrapping_add(1)),
            mmu.read(self.reg.pc.wrapping_add(2)),
            mmu.read(self.reg.pc.wrapping_add(3)),
            );
    }

    pub fn getz(&self) -> u8 {
        self.reg.getf(&Flag::Z)
    }

    pub fn gete(&self) -> u8 {
        self.reg.get8(&Register8::E)
    }
    pub fn getpc(&self) -> u16 {
        self.reg.pc
    }
}
