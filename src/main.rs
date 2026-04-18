pub mod chip8;
pub mod utils;
pub mod instructions;

use chip8::Chip8;

fn main() {
    let rom = [
        0x00E0,       // CLS

        0xA310,       // LD I, 0x210 (I)
        0x6002, 0x6102, 0xD015, // draw I at (2, 2)
    
        0xA320,       // LD I, 0x220 (T)
        0x600C, 0x6102, 0xD015, // draw T at (12, 2)
    
        0xA330,       // LD I, 0x230 (A)
        0x6016, 0x6102, 0xD015, // draw A at (22, 2)
    
        0xA340,       // LD I, 0x240 (L)
        0x6020, 0x6102, 0xD015, // draw L at (32, 2)
    
        0xA350,       // LD I, 0x250 (O)
        0x602A, 0x6102, 0xD015, // draw O at (42, 2)

        // VIADO row, y=12 = 0x0C
        0xA360,       // LD I, 0x310 (V)
        0x6002, 0x610C, 0xD015, // draw V at (2, 12)
        
        0xA370,       // LD I, 0x320 (I)
        0x600C, 0x610C, 0xD015, // draw I at (12, 12)
        
        0xA380,       // LD I, 0x330 (A)
        0x6016, 0x610C, 0xD015, // draw A at (22, 12)
        
        0xA390,       // LD I, 0x340 (D)
        0x6020, 0x610C, 0xD015, // draw D at (32, 12)
        
        0xA3A0,       // LD I, 0x350 (O)
        0x602A, 0x610C, 0xD015, // draw O at (42, 12)
    
        0x1200,       // JP back to start (infinite loop)
    ];

    let mut chip = Chip8::new();
    chip.set_clock(1000);

    // I
    chip.ram[0x310] = 0xF0; // ████
    chip.ram[0x311] = 0x40; // ░█░░
    chip.ram[0x312] = 0x40; // ░█░░
    chip.ram[0x313] = 0x40; // ░█░░
    chip.ram[0x314] = 0xF0; // ████
    
    // T
    chip.ram[0x320] = 0xF0; // ████
    chip.ram[0x321] = 0x40; // ░█░░
    chip.ram[0x322] = 0x40; // ░█░░
    chip.ram[0x323] = 0x40; // ░█░░
    chip.ram[0x324] = 0x40; // ░█░░
    
    // A
    chip.ram[0x330] = 0xF0; // ████
    chip.ram[0x331] = 0x90; // █░░█
    chip.ram[0x332] = 0xF0; // ████
    chip.ram[0x333] = 0x90; // █░░█
    chip.ram[0x334] = 0x90; // █░░█
    
    // L
    chip.ram[0x340] = 0x80; // █░░░
    chip.ram[0x341] = 0x80; // █░░░
    chip.ram[0x342] = 0x80; // █░░░
    chip.ram[0x343] = 0x80; // █░░░
    chip.ram[0x344] = 0xF0; // ████
    
    // O
    chip.ram[0x350] = 0xF0; // ████
    chip.ram[0x351] = 0x90; // █░░█
    chip.ram[0x352] = 0x90; // █░░█
    chip.ram[0x353] = 0x90; // █░░█
    chip.ram[0x354] = 0xF0; // ████
                            //
    // V
    chip.ram[0x360] = 0x90; // █░░█
    chip.ram[0x361] = 0x90; // █░░█
    chip.ram[0x362] = 0x90; // █░░█
    chip.ram[0x363] = 0x60; // ░██░
    chip.ram[0x364] = 0x60; // ░██░
    
    // I
    chip.ram[0x370] = 0xF0; // ████
    chip.ram[0x371] = 0x40; // ░█░░
    chip.ram[0x372] = 0x40; // ░█░░
    chip.ram[0x373] = 0x40; // ░█░░
    chip.ram[0x374] = 0xF0; // ████
    
    // A
    chip.ram[0x380] = 0xF0; // ████
    chip.ram[0x381] = 0x90; // █░░█
    chip.ram[0x382] = 0xF0; // ████
    chip.ram[0x383] = 0x90; // █░░█
    chip.ram[0x384] = 0x90; // █░░█
    
    // D
    chip.ram[0x390] = 0xE0; // ███░
    chip.ram[0x391] = 0x90; // █░█░  
    chip.ram[0x392] = 0x90; // █░█░
    chip.ram[0x393] = 0x90; // █░█░
    chip.ram[0x394] = 0xE0; // ███░
    
    // O
    chip.ram[0x3A0] = 0xF0; // ████
    chip.ram[0x3A1] = 0x90; // █░░█
    chip.ram[0x3A2] = 0x90; // █░░█
    chip.ram[0x3A3] = 0x90; // █░░█
    chip.ram[0x3A4] = 0xF0; // ████

    chip.load_program(&rom);
    chip.start_program();
}
