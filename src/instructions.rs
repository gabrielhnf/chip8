use super::Chip8;

impl Chip8 {
    pub(crate) fn get_instruction(&mut self) -> u16 {
        self.ram[self.program_counter as usize]
    }

    pub(crate) fn cls(&mut self) {
    }

    pub(crate) fn ret(&mut self) {
        self.program_counter = self.stack[self.stack_pointer as usize];
        self.stack_pointer -= 1;
    }

    pub(crate) fn jmp(&mut self, location: u16) {
        self.program_counter = location;
    }

    pub(crate) fn call(&mut self, location: u16) {
        self.stack_pointer += 1;
        self.stack[self.stack_pointer as usize] = self.program_counter;
        self.program_counter = location;
    }

    pub(crate) fn se(&mut self, x: u8, byte: u8) {
        if self.register[x as usize] == byte {
            self.program_counter += 2;
        }
    }

    pub(crate) fn sne(&mut self, x: u8, byte: u8) {
        if self.register[x as usize] != byte {
            self.program_counter += 2;
        }
    }

    pub(crate) fn ser(&mut self, x: u8, y: u8) {
        if self.register[x as usize] == self.register[y as usize] {
            self.program_counter += 2;
        }
    }

    pub(crate) fn ld(&mut self, x: u8, byte: u8) {
        self.register[x as usize] = byte;
    }

}
