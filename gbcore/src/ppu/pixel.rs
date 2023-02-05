enum Palette {
    OBP0,
    OBP1,
}

pub struct Pixel {
    pub color: u8,
    pub palette: Palette,
    pub priority: bool,
    pub bg_priority: bool,
}
