pub mod chip8;
pub mod utils;
pub mod instructions;

use chip8::Chip8;
use minifb::{Key, Window, WindowOptions};

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: chip8 <rom_path> [--clock <hz>] [--log]");
        std::process::exit(1);
    }

    let rom_path = &args[1];
    let mut clock = 600u64;
    let mut log = false;

    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--clock" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("--clock requires a value");
                    std::process::exit(1);
                }
                clock = args[i].parse().unwrap_or_else(|_| {
                    eprintln!("Invalid clock value: {}", args[i]);
                    std::process::exit(1);
                });
            },
            "--log" => log = true,
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                std::process::exit(1);
            }
        }
        i += 1;
    }

    let mut chip = Chip8::new();
    chip.set_clock(clock);
    chip.set_log(log);
    chip.load_program(rom_path);

    let mut window = Window::new("CHIP-8", 64, 32, WindowOptions {
        scale: minifb::Scale::X8,
        ..WindowOptions::default()
    }).unwrap();

    chip.set_render(move |display, keypad| {
        let buffer: Vec<u32> = display.iter()
            .map(|&px| if px { 0xFFFFFFFF } else { 0x00000000 })
            .collect();
        window.update_with_buffer(&buffer, 64, 32).unwrap();

        keypad.fill(false);
        window.get_keys().iter().for_each(|key| match key {
            Key::Key1 => keypad[0x1] = true,
            Key::Key2 => keypad[0x2] = true,
            Key::Key3 => keypad[0x3] = true,
            Key::Key4 => keypad[0xC] = true,
            Key::Q    => keypad[0x4] = true,
            Key::W    => keypad[0x5] = true,
            Key::E    => keypad[0x6] = true,
            Key::R    => keypad[0xD] = true,
            Key::A    => keypad[0x7] = true,
            Key::S    => keypad[0x8] = true,
            Key::D    => keypad[0x9] = true,
            Key::F    => keypad[0xE] = true,
            Key::Z    => keypad[0xA] = true,
            Key::X    => keypad[0x0] = true,
            Key::C    => keypad[0xB] = true,
            Key::V    => keypad[0xF] = true,
            _ => {}
        });
    });

    chip.start_program();
}
