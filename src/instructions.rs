use super::Chip8;

impl Chip8 {
    pub(crate) fn get_instruction(&mut self) -> u16 {
        let instruction: u16 = (self.ram[self.program_counter as usize] as u16) << 8 |
            (self.ram[self.program_counter as usize + 1] as u16); 
        
        instruction
    }

    pub(crate) fn cls(&mut self) {
        self.display = [false; 2048];
    }

    pub(crate) fn ret(&mut self) {
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer as usize];
    }

    pub(crate) fn jmp(&mut self, location: u16) {
        self.program_counter = location;
    }

    pub(crate) fn call(&mut self, location: u16) {
        if self.stack_pointer as usize >= self.stack.len() {
            panic!("Stack overflow! PC: {:#06X}", self.program_counter);
        }
        self.stack[self.stack_pointer as usize] = self.program_counter;
        self.stack_pointer += 1;
        self.program_counter = location;
    }

    pub(crate) fn se_vx(&mut self, x: usize, byte: u8) {
        if self.register[x] == byte {
            self.program_counter += 2;
        }
    }

    pub(crate) fn sne_vx(&mut self, x: usize, byte: u8) {
        if self.register[x] != byte {
            self.program_counter += 2;
        }
    }

    pub(crate) fn se_vx_vy(&mut self, x: usize, y: usize) {
        if self.register[x] == self.register[y] {
            self.program_counter += 2;
        }
    }

    pub(crate) fn ld_vx_byte(&mut self, x: usize, byte: u8) {
        self.register[x] = byte;
    }

    pub(crate) fn add_vx_byte(&mut self, x: usize, byte: u8) {
        self.register[x] = self.register[x].wrapping_add(byte);
    }

    pub(crate) fn ld_vx_vy(&mut self, x: usize, y: usize) {
        self.register[x] = self.register[y];
    }

    pub(crate) fn or_vx_vy(&mut self, x: usize, y: usize) {
        self.register[x] |= self.register[y];
        self.register[0xF] = 0;
    }

    // 8xy2 - AND Vx, Vy
    pub(crate) fn and_vx_vy(&mut self, x: usize, y: usize) {
        self.register[x] &= self.register[y];
        self.register[0xF] = 0;
    }

    // 8xy3 - XOR Vx, Vy
    pub(crate) fn xor_vx_vy(&mut self, x: usize, y: usize) {
        self.register[x] ^= self.register[y];
        self.register[0xF] = 0;
    }

    // 8xy4 - ADD Vx, Vy
    pub(crate) fn add_vx_vy(&mut self, x: usize, y: usize) {
        let result = self.register[x] as u16 + self.register[y] as u16;
        self.register[x] = result as u8;
        self.register[0xF] = if result > 0xFF { 1 } else { 0 };
    }
    
    // 8xy5 - SUB Vx, Vy
    pub(crate) fn sub_vx_vy(&mut self, x: usize, y: usize) {
        let vx = self.register[x];
        let vy = self.register[y];
        self.register[x] = vx.wrapping_sub(vy);
        self.register[0xF] = if vx >= vy { 1 } else { 0 };
    }
    
    // 8xy6 - SHR Vx
    pub(crate) fn shr_vx(&mut self, x: usize, y: usize) {
        let vy = self.register[y];
        self.register[x] = vy >> 1;
        self.register[0xF] = vy & 0x1;
    }
    
    // 8xy7 - SUBN Vx, Vy
    pub(crate) fn subn_vx_vy(&mut self, x: usize, y: usize) {
        let vx = self.register[x];
        let vy = self.register[y];
        self.register[x] = vy.wrapping_sub(vx);
        self.register[0xF] = if vy >= vx { 1 } else { 0 };
    }
    
    // 8xyE - SHL Vx
    pub(crate) fn shl_vx(&mut self, x: usize, y: usize) {
        let vy = self.register[y];
        self.register[x] = vy << 1;
        self.register[0xF] = (vy & 0x80) >> 7;
    }

    // 9xy0 - SNE Vx, Vy
    pub(crate) fn sne_vx_vy(&mut self, x: usize, y: usize) {
        if self.register[x] != self.register[y] {
            self.program_counter += 2;
        }
    }

    // Annn - LD I, addr
    pub(crate) fn ld_i_addr(&mut self, nnn: u16) {
        self.index_register = nnn;
    }

    // Bnnn - JP V0, addr
    pub(crate) fn jp_v0_addr(&mut self, nnn: u16) {
        self.program_counter = nnn + self.register[0x0] as u16;
    }

    // Cxkk - RND Vx, byte
    pub(crate) fn rnd_vx_byte(&mut self, x: usize, kk: u8) {
        let random: u8 = rand::random();
        self.register[x] = random & kk;
    }

    // Dxyn - DRW Vx, Vy, nibble
    pub(crate) fn drw_vx_vy_nibble(&mut self, x: usize, y: usize, n: u8) {
        self.draw_flag = false;
        self.register[0xF] = 0;
        let x_start = (self.register[x] % 64) as u16;  // wrap starting position
        let y_start = (self.register[y] % 32) as u16;  // wrap starting position
    
        for byte_idx in 0..n as u16 {
            let py = y_start + byte_idx;
            if py >= 32 { break; } // clip, don't wrap
    
            let sprite_byte = self.ram[(self.index_register + byte_idx) as usize];
            for bit in 0..8u16 {
                let px = x_start + bit;
                if px >= 64 { break; } // clip, don't wrap
    
                if sprite_byte & (0x80 >> bit) != 0 {
                    let idx = (py * 64 + px) as usize;
                    if self.display[idx] {
                        self.register[0xF] = 1;
                    }
                    self.display[idx] ^= true;
                }
            }
        }

        self.draw_flag = true;
    }

    // Ex9E - SKP Vx
    pub(crate) fn skp_vx(&mut self, x: usize) {
        if self.keypad[self.register[x] as usize] {
            self.program_counter += 2;
        }
    }

    // ExA1 - SKNP Vx
    pub(crate) fn sknp_vx(&mut self, x: usize) {
        if !self.keypad[self.register[x] as usize] {
            self.program_counter += 2;
        }
    }

    // Fx07 - LD Vx, DT
    pub(crate) fn ld_vx_dt(&mut self, x: usize) {
        self.register[x] = *self.delay_timer.value.lock().unwrap();
    }

    // Fx0A - LD Vx, K
    pub(crate) fn ld_vx_k(&mut self, x: usize) {
        match self.keypad.iter().position(|&k| k) {
            Some(key) => self.register[x] = key as u8,
            None => self.program_counter -= 2, // re-execute this instruction
        }
    }

    // Fx15 - LD DT, Vx
    pub(crate) fn ld_dt_vx(&mut self, x: usize) {
        self.delay_timer.activate(self.register[x]);
    }

    // Fx18 - LD ST, Vx
    pub(crate) fn ld_st_vx(&mut self, x: usize) {
        self.sound_timer.activate(self.register[x]);
    }

    // Fx1E - ADD I, Vx
    pub(crate) fn add_i_vx(&mut self, x: usize) {
        self.index_register += self.register[x] as u16;
    }

    // Fx29 - LD F, Vx
    pub(crate) fn ld_f_vx(&mut self, x: usize) {
        self.index_register = self.register[x] as u16 * 5;
    }

    // Fx33 - LD B, Vx
    pub(crate) fn ld_b_vx(&mut self, x: usize) {
        let vx = self.register[x];
        self.ram[self.index_register as usize]       = vx / 100;
        self.ram[self.index_register as usize + 1]   = (vx % 100) / 10;
        self.ram[self.index_register as usize + 2]   = vx % 10;
    }

    // Fx55 - LD [I], Vx
    pub(crate) fn ld_i_vx(&mut self, x: usize) {
        for i in 0..=x {
            self.ram[self.index_register as usize + i] = self.register[i];
        }
        self.index_register += x as u16 + 1;
    }

    // Fx65 - LD Vx, [I]
    pub(crate) fn ld_vx_i(&mut self, x: usize) {
        for i in 0..=x {
            self.register[i] = self.ram[self.index_register as usize + i] as u8;
        }
        self.index_register += x as u16 + 1;
    }
}
