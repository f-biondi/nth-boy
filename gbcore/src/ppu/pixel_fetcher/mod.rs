pub mod bg_fetcher;
pub mod pixel_fifo;
pub mod sprite_fetcher;

#[derive(PartialEq)]
pub enum FetchState {
    FETCH_NO,
    FETCH_DATA_LOW,
    FETCH_DATA_HIGH,
    PUSH,
}

#[derive(Copy, Clone, Debug)]
pub enum Palette {
    OBP0,
    OBP1,
    BGP,
}

#[derive(Copy, Clone, Debug)]
pub struct Pixel {
    pub color: u8,
    pub palette: Palette,
    pub priority: bool,
    pub bg_priority: bool,
}

pub trait Pixelfetcher {
    fn shift(&mut self) -> Option<Pixel>;
}
