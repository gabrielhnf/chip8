pub mod chip8;
pub mod utils;
pub mod instructions;

use std::fs;

use chip8::Chip8;

fn main() {
    let mut chip = Chip8::new();
    chip.set_clock(1000);

    let rom = fs::read("/home/gabriel/Documents/chip8/programs/test.ch8").unwrap();

    chip.load_program(&rom[..]);
    chip.start_program();
}
