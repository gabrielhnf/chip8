use std::thread::sleep;
use super::utils::Hertz;


#[allow(dead_code)]
pub struct Chip8 {
    pub(crate) register: [u8; 16],
    pub(crate) index_register: u16,
    pub(crate) stack: [u16; 32],
    pub(crate) stack_pointer: u8,
    pub(crate) delay_timer: u8,
    pub(crate) sound_timer: u8,
    pub(crate) program_counter: u16,
    pub(crate) ram: [u16; 2048],
    pub(crate) clock_rate: Hertz,
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
            clock_rate: Hertz::new(600),
        }
    }

    pub fn start_program(&mut self) { //Will assume program loaded
        loop {
            //fetch -> read instruction from memory (2-byte instruction)
            //decode -> decode instruction (using match)
            //execute -> execute it inside proper match arm
            self.log_state();
            let instruction = self.get_instruction();
            match instruction & 0xF000 {

                0x0000 => match instruction & 0x0FFF {
                    0x00E0 => {
                        self.cls();
                        self.program_counter += 1;
                    },
                    0x00EE => self.ret(),
                    0x0000 => break, //Should throw or exit, SYS not supported.
                    _ => {}
                },

                0x1000 => self.jmp(instruction & 0x0FFF),
                0x2000 => self.call(instruction & 0x0FFF),

                0x3000 => {
                    let reg_index = ((instruction & 0x0F00) >> 8) as u8;
                    let byte = (instruction & 0x00FF) as u8;
                    self.se(reg_index, byte);
                    self.program_counter += 1;
                },

                0x4000 => {
                    let reg_index = ((instruction & 0x0F00) >> 8) as u8;
                    let byte = (instruction & 0x00FF) as u8;
                    self.sne(reg_index, byte);
                    self.program_counter += 1;
                },

                0x5000 => {
                    let x = ((instruction & 0x0F00) >> 8) as u8;
                    let y = ((instruction & 0x00F0) >> 4) as u8;
                    self.ser(x, y);
                    self.program_counter += 1;
                },

                0x6000 => {
                    let reg_index = ((instruction & 0x0F00) >> 8) as u8;
                    let byte = (instruction & 0x00FF) as u8;
                    self.ld(reg_index, byte);
                    self.program_counter += 1;
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

}
