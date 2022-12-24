pub enum Register8 {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

pub enum Register16 {
    AF,
    BC,
    HL,
    DE,
    SP,
    PC,
}

#[derive(Debug)]
pub enum Flag {
    Z,
    N,
    H,
    C,
    NZ,
    NN,
    NH,
    NC,
}

pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    f: u8,
    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    pub fn new() -> Self {
        /*Self {
            a: 0x11,
            b: 0x00,
            c: 0x00,
            d: 0xff,
            e: 0x56,
            f: 0x80,
            h: 0x0,
            l: 0xd,
            sp: 0xfffe,
            pc: 0x0100,
        }*/
        Self {
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            f: 0xB0,
            h: 0x01,
            l: 0x4D,
            sp: 0xfffe,
            pc: 0x0100,
        }
    }

    pub fn get8(&self, name: &Register8) -> u8 {
        match name {
            Register8::A => self.a,
            Register8::B => self.b,
            Register8::C => self.c,
            Register8::D => self.d,
            Register8::E => self.e,
            Register8::F => self.f,
            Register8::H => self.h,
            Register8::L => self.l,
        }
    }

    pub fn get16(&self, name: &Register16) -> u16 {
        match name {
            Register16::AF => ((self.a as u16) << 8) + (self.f as u16),
            Register16::BC => ((self.b as u16) << 8) + (self.c as u16),
            Register16::HL => ((self.h as u16) << 8) + (self.l as u16),
            Register16::DE => ((self.d as u16) << 8) + (self.e as u16),
            Register16::SP => self.sp,
            Register16::PC => self.pc,
        }
    }

    pub fn set8(&mut self, name: &Register8, value: u8) {
        match name {
            Register8::A => self.a = value,
            Register8::B => self.b = value,
            Register8::C => self.c = value,
            Register8::D => self.d = value,
            Register8::E => self.e = value,
            Register8::F => self.f = value,
            Register8::H => self.h = value,
            Register8::L => self.l = value,
        };
    }

    pub fn set16(&mut self, name: &Register16, value: u16) {
        match name {
            Register16::AF => {
                self.a = ((value & 0xff00) >> 8) as u8;
                self.f = (value & 0x00ff) as u8;
            }
            Register16::BC => {
                self.b = ((value & 0xff00) >> 8) as u8;
                self.c = (value & 0x00ff) as u8;
            }
            Register16::HL => {
                self.h = ((value & 0xff00) >> 8) as u8;
                self.l = (value & 0x00ff) as u8;
            }
            Register16::DE => {
                self.d = ((value & 0xff00) >> 8) as u8;
                self.e = (value & 0x00ff) as u8;
            }
            Register16::SP => self.sp = value,
            Register16::PC => self.pc = value,
        };
    }

    pub fn getf(&self, name: &Flag) -> u8 {
        match name {
            Flag::Z => (self.f & 0b10000000) >> 7,
            Flag::N => (self.f & 0b01000000) >> 6,
            Flag::H => (self.f & 0b00100000) >> 5,
            Flag::C => (self.f & 0b00010000) >> 4,
            Flag::NZ => (self.f ^ 0b10000000) >> 7,
            Flag::NN => (self.f ^ 0b01000000) >> 6,
            Flag::NH => (self.f ^ 0b00100000) >> 5,
            Flag::NC => (self.f ^ 0b00010000) >> 4,
        }
    }

    pub fn setf(&mut self, name: &Flag) {
        match name {
            Flag::Z => self.f |= 0b10000000,
            Flag::N => self.f |= 0b01000000,
            Flag::H => self.f |= 0b00100000,
            Flag::C => self.f |= 0b00010000,
            Flag::NZ => self.unsetf(&Flag::Z),
            Flag::NN => self.unsetf(&Flag::N),
            Flag::NH => self.unsetf(&Flag::H),
            Flag::NC => self.unsetf(&Flag::C),
        };
    }

    pub fn unsetf(&mut self, name: &Flag) {
        match name {
            Flag::Z => self.f &= 0b01111111,
            Flag::N => self.f &= 0b10111111,
            Flag::H => self.f &= 0b11011111,
            Flag::C => self.f &= 0b11101111,
            Flag::NZ => self.setf(&Flag::Z),
            Flag::NN => self.setf(&Flag::N),
            Flag::NH => self.setf(&Flag::H),
            Flag::NC => self.setf(&Flag::C),
        };
    }
}
