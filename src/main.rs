use std::{thread::sleep, time::Duration};

#[allow(dead_code)]
struct Chip8 {
    register: [u8; 16],
    index_register: u16,
    stack: [u16; 32],
    stack_pointer: u8,
    delay_timer: u8,
    sound_timer: u8,
    program_counter: u16,
    ram: [u16; 2048],
    clock_rate: Hertz,
}

struct Hertz {
    period: Duration,
}

impl Hertz {
    pub fn new(freq: u64) -> Self {
        Self { period: Duration::from_millis(1/freq * 1000)}
    }

    pub fn period(&self) -> Duration {
        self.period
    }
}

impl Chip8 {

    pub fn new() -> Self {
        Self { 
            register: [0; 16],
            index_register: 0,
            stack: [0; 32],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            program_counter: 0,
            ram: [0; 2048],
            clock_rate: Hertz::new(1),
        }
    }

    pub fn start_program(&mut self) { //Will assume program loaded
        loop {
            //fetch -> read instruction from memory (2-byte instruction)
            //decode -> decode instruction (using match)
            //execute -> execute it inside proper match arm
            let instruction = self.get_instruction();
            match instruction & 0xF000 {

                0x0000 => match instruction & 0x0FFF {
                    0x00E0 => self.cls(),
                    0x00EE => self.ret(),
                    0x0000 => break, //Should throw or exit, SYS not supported.
                    _ => {}
                },

                0x1000 => self.jmp(instruction & 0x0FFF),
                0x2000 => self.call(instruction & 0x0FFF),

                0x3000 => {
                    let reg_index = ((instruction & 0x0F00) << 8) as u8;
                    let byte = (instruction & 0x00FF) as u8;
                    self.se(reg_index, byte)
                },

                0x4000 => {
                    let reg_index = ((instruction & 0x0F00) << 8) as u8;
                    let byte = (instruction & 0x00FF) as u8;
                    self.sne(reg_index, byte)
                },

                0x5000 => {
                    let x = ((instruction & 0x0F00) << 8) as u8;
                    let y = ((instruction & 0x00F0) << 4) as u8;
                    self.ser(x, y)
                },

                0x6000 => {
                    let reg_index = ((instruction & 0x0F00) << 8) as u8;
                    let byte = (instruction & 0x00FF) as u8;
                    self.ld(reg_index, byte)
                },

                0x7000 => {},
                0x8000 => {},
                0x9000 => {},
                0xA000 => {},
                0xB000 => {},
                0xC000 => {},
                0xD000 => {},
                0xE000 => {},
                0xF000 => {},
                _ => {}
            }
            
            sleep(self.clock_rate.period());
            self.program_counter += 1;
        }
    }

    pub fn load_program(&mut self, rom: &[u16]) {
        //Read rom into ram from 0x200 (maybe rom will be array of u16)
        let offset = 0x200;
        for (index, instruction) in rom.iter().enumerate() {
            self.ram[offset + index] = *instruction;
        }
        //Set PC to 0x200
        self.program_counter = offset as u16;
    }

    fn get_instruction(&mut self) -> u16 {
        self.ram[self.program_counter as usize]
    }

    pub fn cls(&mut self) {
        println!("Cleared.");
    }

    fn ret(&mut self) {
        self.program_counter = self.stack[self.stack_pointer as usize];
        self.stack_pointer -= 1;
        println!("Returned.");
    }

    fn jmp(&mut self, location: u16) {
        self.program_counter = location;
        println!("Jumped.");
    }

    fn call(&mut self, location: u16) {
        self.stack_pointer += 1;
        self.stack[self.stack_pointer as usize] = self.program_counter;
        self.program_counter = location;
        println!("Called.");
    }

    fn se(&mut self, x: u8, byte: u8) {
        if self.register[x as usize] == byte {
            self.program_counter += 2;
        }
        println!("SE'd.");
    }

    fn sne(&mut self, x: u8, byte: u8) {
        if self.register[x as usize] != byte {
            self.program_counter += 2;
        }
    }

    fn ser(&mut self, x: u8, y: u8) {
        if self.register[x as usize] == self.register[y as usize] {
            self.program_counter += 2;
        }
    }

    fn ld(&mut self, x: u8, byte: u8) {
        self.register[x as usize] = byte;
    }

}

fn main() {
    let mut chip = Chip8::new();
    let rom = [0x00E0, 0x00E0, 0x1111];
    chip.load_program(&rom);
    chip.start_program();
}
