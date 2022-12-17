pub mod rom;

pub trait Addressable {
    fn write(&mut self, location: u16, byte: u8);
    fn read(&self, location: u16) -> u8;
}

