use gbcore::mmu::address_spaces::io::joypad::JoypadState;
use gbcore::ppu::LcdBuffer;
use gbcore::Device;
use minifb::{Key, Scale, Window, WindowOptions};
use std::env;
use std::error::Error;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let mut emulator = Device::new(&args[1])?;

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

    let empty_buffer: Vec<u32> = vec![0xffffff; WIDTH * HEIGHT];

    let mut lcd_buffer: LcdBuffer = LcdBuffer {
        buffer: vec![0; WIDTH * HEIGHT],
        cleared: false,
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let pressed_keys: Vec<Key> = window.get_keys();
        emulator.frame(
            &mut lcd_buffer,
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

        if !lcd_buffer.cleared {
            window
                .update_with_buffer(&lcd_buffer.buffer, WIDTH, HEIGHT)
                .unwrap();
        } else {
            window
                .update_with_buffer(&empty_buffer, WIDTH, HEIGHT)
                .unwrap();
            lcd_buffer.cleared = false;
        }
    }
    emulator.save()?;
    Ok(())
}
