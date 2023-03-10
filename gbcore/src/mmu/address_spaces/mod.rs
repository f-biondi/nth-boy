pub mod adressable_memory;
pub mod cart;
pub mod io;
pub mod oam;

pub trait Addressable {
    fn write(&mut self, location: u16, byte: u8);
    fn read(&self, location: u16) -> u8;

    fn read_16(&self, location: u16) -> u16 {
        let low: u8 = self.read(location);
        let high: u8 = self.read(location + 1);
        ((high as u16) << 8) + (low as u16)
    }
}
