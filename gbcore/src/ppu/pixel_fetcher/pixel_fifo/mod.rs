use crate::ppu::pixel_fetcher::Pixel;

pub mod merge_pixel_fifo;
pub mod standard_pixel_fifo;

pub trait PixelFifo {
    fn clear(&mut self);
    fn push(&mut self, pixel: Pixel);
    fn shift(&mut self) -> Option<Pixel>;
    fn len(&mut self) -> u8;
}
