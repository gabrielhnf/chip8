pub mod chip8;
pub mod utils;
pub mod instructions;

use chip8::Chip8;

fn main() {
    let rom = [0x00E0, 0x00E0, 0x00E0, 0x7210, 0x1200];

    let mut chip = Chip8::new();
    chip.set_clock(1);

    chip.load_program(&rom);
    chip.start_program();
}
