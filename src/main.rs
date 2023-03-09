#![allow(dead_code)]
#![allow(unused_variables)]

use gbcore::mmu::address_spaces::io::joypad::JoypadState;
use gbcore::Device;
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use std::error::Error;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

fn main() -> Result<(), Box<dyn Error>> {
    let mut window = Window::new(
        "nth-boy",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X4,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to create window");

    window.limit_update_rate(Some(std::time::Duration::from_micros(16742)));

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    //let mut emulator = Device::new("/home/fbiondi/nth-boy/roms/tests/instr_timing/instr_timing.gb")?;
    //let mut emulator = Device::new("/home/fbiondi/nth-boy/roms/tetris.gb")?;
    //let mut emulator = Device::new("/home/fbiondi/nth-boy/roms/boxxle.gb")?;
    //let mut emulator = Device::new("/home/fbiondi/nth-boy/roms/lode.gb")?;
    //let mut emulator = Device::new("/home/fbiondi/nth-boy/roms/alleyway.gb")?;
    //let mut emulator = Device::new("/home/fbiondi/nth-boy/roms/flipull.gb")?;
    //let mut emulator = Device::new("/home/fbiondi/nth-boy/roms/tests/cpu_instrs/cpu_instrs.gb")?;
    //let mut emulator = Device::new("/home/fbiondi/nth-boy/roms/midori.gb")?;
    let mut emulator = Device::new("/home/fbiondi/nth-boy/roms/kirby.gb")?;
    //let mut emulator = Device::new("/home/fbiondi/nth-boy/roms/mario.gb")?;
    //let mut emulator = Device::new("/home/fbiondi/nth-boy/roms/zelda.gb")?;
    //let mut emulator = Device::new("/home/fbiondi/nth-boy/roms/DMG_ROM.gb")?;
    //let mut emulator = Device::new("/home/fbiondi/nth-boy/roms/tests/cpu_instrs/individual/2.gb")?;
    //let mut emulator = Device::new("/home/fbiondi/nth-boy/roms/dmg-acid2.gb")?;
    //let mut emulator = Device::new("/home/fbiondi/nth-boy/roms/drmario.gb")?;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let pressed_keys: Vec<Key> = window.get_keys();
        emulator.frame(
            &mut buffer,
            JoypadState {
                up: pressed_keys.contains(&Key::W),
                down: pressed_keys.contains(&Key::S),
                left: pressed_keys.contains(&Key::A),
                right: pressed_keys.contains(&Key::D),
                a: pressed_keys.contains(&Key::J),
                b: pressed_keys.contains(&Key::K),
                start: pressed_keys.contains(&Key::Enter),
                select: pressed_keys.contains(&Key::Delete),
            },
        );

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
    Ok(())
}
