use std::{fs, thread::sleep};
use super::utils::{Hertz, Timer};

#[allow(dead_code)]
pub struct Chip8 {
    pub(crate) register: [u8; 16],
    pub(crate) index_register: u16,

    pub(crate) stack: [u16; 16],
    pub(crate) stack_pointer: u8,

    pub(crate) delay_timer: Timer,
    pub(crate) sound_timer: Timer,

    pub(crate) program_counter: u16,
    pub(crate) ram: [u8; 4096],

    pub(crate) clock_rate: Hertz,

    pub(crate) keypad: [bool; 16],
    pub(crate) display: [bool; 2048],

    pub(crate) render: Box<dyn FnMut(&[bool; 2048], &mut [bool; 16])>,

    pub debug: bool,
}

const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

impl Chip8 {
    pub fn new() -> Self {
        let mut chip = Chip8 { 
            register: [0; 16],
            index_register: 0,

            stack: [0; 16],
            stack_pointer: 0,

            delay_timer: Timer::new(),
            sound_timer: Timer::new(),

            program_counter: 0,
            ram: [0; 4096],

            clock_rate: Hertz::new(600),

            keypad: [false; 16],
            display: [false; 2048],

            render: Box::new(|_,_| {}),

            debug: false,
        };

        chip.ram[0x000..0x050].copy_from_slice(&FONT);
        chip
    }

    pub fn set_log(&mut self, log: bool) {
        self.debug = log;
    }

    pub fn set_render(&mut self, func: impl FnMut(&[bool; 2048], &mut [bool; 16]) + 'static) {
        self.render = Box::new(func);
    }

    pub fn load_program(&mut self, path: &str) {
        let rom = fs::read(path).unwrap();

        //Read rom into ram from 0x200 (maybe rom will be array of u16)
        let offset = 0x200;
        for (index, instruction) in rom.iter().enumerate() {
            self.ram[offset + index] = *instruction;
        }
        //Set PC to 0x200
        self.program_counter = offset as u16;
    }

    pub fn start_program(&mut self) { //Will assume program loaded
        loop {
            //self.log_state();

            let instruction = self.get_instruction();
            self.program_counter += 2;

            if self.debug {
                println!("Instruction: {:#06X}", instruction);
                self.log_state();
            }

            match instruction & 0xF000 {
                0x0000 => match instruction & 0x0FFF {
                    0x00E0 => {
                        self.cls();
                    },
                    0x00EE => self.ret(),
                    0x0000 => break,
                    _ => {}
                },
                0x1000 => self.jmp(instruction & 0x0FFF),
                0x2000 => self.call(instruction & 0x0FFF),
                0x3000 => {
                    let x = ((instruction & 0x0F00) >> 8) as usize;
                    let kk = (instruction & 0x00FF) as u8;
                    self.se_vx(x, kk);
                },
                0x4000 => {
                    let x = ((instruction & 0x0F00) >> 8) as usize;
                    let kk = (instruction & 0x00FF) as u8;
                    self.sne_vx(x, kk);
                },
                0x5000 => {
                    let x = ((instruction & 0x0F00) >> 8) as usize;
                    let y = ((instruction & 0x00F0) >> 4) as usize;
                    self.se_vx_vy(x, y);
                },
                0x6000 => {
                    let x = ((instruction & 0x0F00) >> 8) as usize;
                    let kk = (instruction & 0x00FF) as u8;
                    self.ld_vx_byte(x, kk);
                },
                0x7000 => {
                    let x = ((instruction & 0x0F00) >> 8) as usize;
                    let kk = (instruction & 0x00FF) as u8;
                    self.add_vx_byte(x, kk);
                },
                0x8000 => {
                    let x = ((instruction & 0x0F00) >> 8) as usize;
                    let y = ((instruction & 0x00F0) >> 4) as usize;
                    match instruction & 0x000F {
                        0x0000 => self.ld_vx_vy(x, y),
                        0x0001 => self.or_vx_vy(x, y),
                        0x0002 => self.and_vx_vy(x, y),
                        0x0003 => self.xor_vx_vy(x, y),
                        0x0004 => self.add_vx_vy(x, y),
                        0x0005 => self.sub_vx_vy(x, y),
                        0x0006 => self.shr_vx(x),
                        0x0007 => self.subn_vx_vy(x, y),
                        0x000E => self.shl_vx(x),
                        _ => {}
                    }
                },
                0x9000 => {
                    let x = ((instruction & 0x0F00) >> 8) as usize;
                    let y = ((instruction & 0x00F0) >> 4) as usize;
                    self.sne_vx_vy(x, y);
                },
                0xA000 => self.ld_i_addr(instruction & 0x0FFF),
                0xB000 => self.jp_v0_addr(instruction & 0x0FFF),
                0xC000 => {
                    let x = ((instruction & 0x0F00) >> 8) as usize;
                    let kk = (instruction & 0x00FF) as u8;
                    self.rnd_vx_byte(x, kk);
                },
                0xD000 => {
                    let x = ((instruction & 0x0F00) >> 8) as usize;
                    let y = ((instruction & 0x00F0) >> 4) as usize;
                    let n = (instruction & 0x000F) as u8;
                    self.drw_vx_vy_nibble(x, y, n);
                },
                0xE000 => {
                    let x = ((instruction & 0x0F00) >> 8) as usize;
                    match instruction & 0x00FF {
                        0x009E => self.skp_vx(x),
                        0x00A1 => self.sknp_vx(x),
                        _ => {}
                    }
                },
                0xF000 => {
                    let x = ((instruction & 0x0F00) >> 8) as usize;
                    match instruction & 0x00FF {
                        0x0007 => self.ld_vx_dt(x),
                        0x000A => self.ld_vx_k(x),
                        0x0015 => self.ld_dt_vx(x),
                        0x0018 => self.ld_st_vx(x),
                        0x001E => self.add_i_vx(x),
                        0x0029 => self.ld_f_vx(x),
                        0x0033 => self.ld_b_vx(x),
                        0x0055 => self.ld_i_vx(x),
                        0x0065 => self.ld_vx_i(x),
                        _ => {}
                    }
                },
                _ => panic!("unknown opcode {:#06X}", instruction)
            }

            (self.render)(&self.display, &mut self.keypad);

            sleep(self.clock_rate.period());
        }
    }

}
