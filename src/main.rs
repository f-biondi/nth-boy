#![allow(dead_code)]
#![allow(unused_variables)]

use gbcore::Device;
use std::error::Error;
use minifb::{Key, ScaleMode, Window, WindowOptions};

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

fn main() -> Result<(), Box<dyn Error>> {
    let mut window = Window::new(
        "nth-boy",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
        /*WindowOptions {
            resize: true,
            scale_mode: ScaleMode::UpperLeft,
            ..WindowOptions::default()
        },*/
    )
    .expect("Unable to create window");

    window.limit_update_rate(Some(std::time::Duration::from_micros(16742)));

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    //let mut emulator = Device::new("/home/fbiondi/nth-boy/roms/tests/instr_timing/instr_timing.gb")?;
    //let mut emulator = Device::new("/home/fbiondi/nth-boy/roms/tetris.gb")?;
    //let mut emulator = Device::new("/home/fbiondi/nth-boy/roms/DMG_ROM.gb")?;
    let mut emulator = Device::new("/home/fbiondi/nth-boy/roms/tests/cpu_instrs/individual/2.gb")?;
    //let mut emulator = Device::new("/home/fbiondi/nth-boy/roms/dmg-acid2.gb")?;

    while window.is_open() && !window.is_key_down(Key::Escape) {

        emulator.frame(&mut buffer);

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
    Ok(())
}
