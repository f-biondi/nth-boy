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
        Self{
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

    pub fn get(&self, name: &str) -> u16 {
        match name {
            "A" => self.a as u16,
            "B" => self.b as u16,
            "C" => self.c as u16,
            "D" => self.d as u16,
            "E" => self.e as u16,
            "F" => self.f as u16,
            "H" => self.h as u16,
            "L" => self.l as u16,
            "AF" => ((self.a as u16) << 8) + (self.f as u16),
            "BC" => ((self.b as u16) << 8) + (self.c as u16),
            "HL" => ((self.h as u16) << 8) + (self.l as u16),
            _ => panic!("No register {}", name)
        }
    }

    pub fn set(&mut self, name: &str, value: u16) {
        match name {
            "A" => self.a = value as u8,
            "B" => self.b = value as u8,
            "C" => self.c = value as u8,
            "D" => self.d = value as u8,
            "E" => self.e = value as u8,
            "F" => self.f = value as u8,
            "H" => self.h = value as u8,
            "L" => self.l = value as u8,
            "AF" => {
                self.a = ((value & 0xff00) >> 8) as u8;
                self.f = (value & 0x00ff) as u8;
            },
            "BC" => {
                self.b = ((value & 0xff00) >> 8) as u8;
                self.c = (value & 0x00ff) as u8;
            },
            "HL" => {
                self.h = ((value & 0xff00) >> 8) as u8;
                self.l = (value & 0x00ff) as u8;
            },
            _ => panic!("No register {}", name)
        }
    }
}
