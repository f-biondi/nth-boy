use crate::mmu::address_spaces::Addressable;

pub struct GenericAddressable {
    memory: Vec<u8>,
    start: u16,
    end: u16,
}

impl GenericAddressable {
    pub fn new(start: u16, end: u16) -> Result<Self, String> {
        match end.checked_sub(start) {
            Some(size) => Ok(Self {
                memory: vec![0; (size as usize) + 1usize],
                start: start,
                end: end,
            }),
            None => Err(String::from("End address is greater than start address")),
        }
    }

    fn check_range(&self, location: u16) {
        if location < self.start || location > self.end {
            panic!("Out of range location {:#04x}", location);
        }
    }
}

impl Addressable for GenericAddressable {
    fn write(&mut self, location: u16, byte: u8) {
        self.check_range(location);
        self.memory[(location - self.start) as usize] = byte;
    }

    fn read(&self, location: u16) -> u8 {
        self.check_range(location);
        self.memory[(location - self.start) as usize]
    }
}
