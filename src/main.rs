pub mod chip8;
pub mod utils;
pub mod instructions;

use chip8::Chip8;
use minifb::{Key, Window, WindowOptions};

fn main() {
    let mut chip = Chip8::new();
    chip.set_clock(1000);
    chip.load_program("/home/gabriel/Documents/chip8/programs/danm8ku.ch8");

    let mut window = Window::new("CHIP-8", 64, 32, WindowOptions {
            scale: minifb::Scale::X8,
            ..WindowOptions::default()
        }).unwrap();


    chip.set_render(move |display, keypad| {
        let buffer: Vec<u32> = display.iter().map(|&px| if px { 0xFFFFFFFF } else { 0x00000000 }).collect();
        window.update_with_buffer(&buffer, 64, 32).unwrap();
        
        keypad.fill(false);
        //CHIP-8    Keyboard
        //1 2 3 C   1 2 3 4
        //4 5 6 D   Q W E R
        //7 8 9 E   A S D F
        //A 0 B F   Z X C V
        window.get_keys().iter().for_each(|key| match key {
            Key::Key1  => keypad[0x1] = true,
            Key::Key2  => keypad[0x2] = true,
            Key::Key3  => keypad[0x3] = true,
            Key::Key4  => keypad[0xC] = true,
            Key::Q     => keypad[0x4] = true,
            Key::W     => keypad[0x5] = true,
            Key::E     => keypad[0x6] = true,
            Key::R     => keypad[0xD] = true,
            Key::A     => keypad[0x7] = true,
            Key::S     => keypad[0x8] = true,
            Key::D     => keypad[0x9] = true,
            Key::F     => keypad[0xE] = true,
            Key::Z     => keypad[0xA] = true,
            Key::X     => keypad[0x0] = true,
            Key::C     => keypad[0xB] = true,
            Key::V     => keypad[0xF] = true,
            _ => {}
        });
    });


    chip.start_program();
}
