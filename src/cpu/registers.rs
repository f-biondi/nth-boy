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
        Self {
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xd8,
            f: 0b10000000,
            h: 0x01,
            l: 0x4d,
            sp: 0xfffe,
            pc: 0x100,
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
}
