use crate::memory::Memory;

#[derive(Clone, Copy)]
pub struct CPU {
    // General Purpose Registers (r0-r15)
    pub registers: [u32; 16],
    
    // Current Program Status Register
    pub cpsr: u32,
    
    // Spilled Registers (for different modes)
    pub spsr: u32,
    
    // Execution state
    pub thumb_mode: bool,
}

impl CPU {
    pub fn new() -> Self {
        let mut cpu = CPU {
            registers: [0u32; 16],
            cpsr: 0x6000001F, // Initial CPSR (System mode)
            spsr: 0,
            thumb_mode: false,
        };
        
        // PC starts at BIOS entry point
        cpu.registers[15] = 0x0000_0000;
        cpu
    }

    pub fn step(&mut self, memory: &mut Memory) {
        if self.thumb_mode {
            self.execute_thumb_instruction(memory);
        } else {
            self.execute_arm_instruction(memory);
        }
    }

    fn execute_arm_instruction(&mut self, memory: &mut Memory) {
        let pc = self.registers[15];
        let instruction = memory.read_word(pc);
        
        // Check condition codes
        if !self.check_condition((instruction >> 28) as u8) {
            self.registers[15] = self.registers[15].wrapping_add(4);
            return;
        }

        // Decode and execute instruction
        match (instruction >> 26) & 0x3 {
            0 => self.execute_data_processing(instruction, memory),
            1 => self.execute_single_data_transfer(instruction, memory),
            2 => self.execute_block_data_transfer(instruction, memory),
            3 => self.execute_branch(instruction, memory),
            _ => {}
        }

        // Increment PC
        self.registers[15] = self.registers[15].wrapping_add(4);
    }

    fn execute_thumb_instruction(&mut self, memory: &mut Memory) {
        let pc = self.registers[15];
        let instruction = memory.read_halfword(pc);
        
        // Stub: Implement Thumb instruction decoding
        self.registers[15] = self.registers[15].wrapping_add(2);
    }

    fn execute_data_processing(&mut self, instruction: u32, memory: &mut Memory) {
        let opcode = (instruction >> 21) & 0xF;
        let s_flag = (instruction >> 20) & 1 != 0;
        let rn = ((instruction >> 16) & 0xF) as usize;
        let rd = ((instruction >> 12) & 0xF) as usize;
        
        let operand1 = self.registers[rn];
        let operand2 = self.get_operand2(instruction, memory);

        let result = match opcode {
            0x0 => operand1 & operand2, // AND
            0x1 => operand1 ^ operand2, // EOR
            0x2 => operand1.wrapping_sub(operand2), // SUB
            0x3 => operand2.wrapping_sub(operand1), // RSB
            0x4 => operand1.wrapping_add(operand2), // ADD
            0x5 => operand1.wrapping_add(operand2).wrapping_add((self.cpsr >> 29) & 1), // ADC
            0x6 => operand1.wrapping_sub(operand2).wrapping_sub(((self.cpsr >> 29) & 1) ^ 1), // SBC
            0x7 => operand2.wrapping_sub(operand1).wrapping_sub(((self.cpsr >> 29) & 1) ^ 1), // RSC
            0x8 | 0x9 | 0xA | 0xB => operand1, // TST, TEQ, CMP, CMN (only update flags)
            0xC => operand1 | operand2, // ORR
            0xD => operand2, // MOV
            0xE => operand1 & !operand2, // BIC
            0xF => !operand2, // MVN
            _ => 0,
        };

        if rd != 15 {
            self.registers[rd] = result;
        } else {
            self.registers[15] = result & !0x3; // Align to 4 bytes
        }

        if s_flag {
            self.update_flags(result);
        }
    }

    fn execute_single_data_transfer(&mut self, instruction: u32, _memory: &mut Memory) {
        // Stub: Implement LDR/STR
    }

    fn execute_block_data_transfer(&mut self, instruction: u32, _memory: &mut Memory) {
        // Stub: Implement LDM/STM
    }

    fn execute_branch(&mut self, instruction: u32, _memory: &mut Memory) {
        let offset = ((instruction & 0xFF_FFFF) as i32) << 2;
        self.registers[15] = (self.registers[15] as i32).wrapping_add(offset) as u32;
    }

    fn get_operand2(&self, instruction: u32, _memory: &Memory) -> u32 {
        if (instruction >> 25) & 1 != 0 {
            // Immediate value
            let imm = (instruction & 0xFF) as u32;
            let rotate = ((instruction >> 8) & 0xF) as u32 * 2;
            imm.rotate_right(rotate)
        } else {
            // Register operand
            let rm = (instruction & 0xF) as usize;
            self.registers[rm]
        }
    }

    fn check_condition(&self, cond: u8) -> bool {
        let z = (self.cpsr >> 30) & 1 != 0;
        let c = (self.cpsr >> 29) & 1 != 0;
        let n = (self.cpsr >> 31) & 1 != 0;
        let v = (self.cpsr >> 28) & 1 != 0;

        match cond {
            0x0 => z,                           // EQ
            0x1 => !z,                          // NE
            0x2 => c,                           // CS/HS
            0x3 => !c,                          // CC/LO
            0x4 => n,                           // MI
            0x5 => !n,                          // PL
            0x6 => v,                           // VS
            0x7 => !v,                          // VC
            0x8 => c && !z,                     // HI
            0x9 => !c || z,                     // LS
            0xA => n == v,                      // GE
            0xB => n != v,                      // LT
            0xC => !z && (n == v),              // GT
            0xD => z || (n != v),               // LE
            0xE => true,                        // AL (always)
            0xF => true,                        // NV (never, but execute anyway)
            _ => false,
        }
    }

    fn update_flags(&mut self, result: u32) {
        self.cpsr &= 0x0FFF_FFFF;
        
        if result == 0 {
            self.cpsr |= 1 << 30; // Z flag
        }
        
        if (result as i32) < 0 {
            self.cpsr |= 1 << 31; // N flag
        }
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}
