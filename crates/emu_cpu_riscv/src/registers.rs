use std::fmt;

use emu_utils::sign_extend_64;



pub struct RegisterFile {
    is_32_bit: bool,
    x: [u64; 32],
    pc: u64,
}

impl RegisterFile {
    pub fn new(is_32_bit: bool) -> Self {
        Self {
            is_32_bit,
            x: [0;32],
            pc: 0,
        }
    }

    pub fn set_32_bit(&mut self, b: bool) {
        self.is_32_bit = b;
        
        if b {
            for val in &mut self.x {
                *val &= u32::MAX as u64;
            }
        }
    }

    pub fn is_32_bit(&self) -> bool {
        self.is_32_bit
    }

    pub fn read_x_register(&self, index: u8) -> u64 {
        debug_assert!(index < 32);
        self.x[index as usize]
    }

    pub fn read_x_register_sign_extended(&self, index: u8) -> u64 {
        debug_assert!(index < 32);
        let val = self.x[index as usize];
        if self.is_32_bit { sign_extend_64(val, 31) } else { val }
    }

    pub fn write_x_register(&mut self, index: u8, value: u64) {
        debug_assert!(index < 32);
        debug_assert!(index != 0);
        let value = if self.is_32_bit { value & u32::MAX as u64 } else { value }; 
        self.x[index as usize] = value;
    }

    pub fn read_pc(&self) -> u64 {
        self.pc
    }

    pub fn write_pc(&mut self, value: u64) {
        self.pc = if self.is_32_bit { value & u32::MAX as u64 } else { value }; 
    }

    pub fn offset_pc(&mut self, offset: u64) {
        self.pc = self.pc.wrapping_add(offset);
    }

    pub fn inc_pc(&mut self, size: u64) {
        assert!(size & 1 == 0, "Can only increment pc with a multiple of 2");
        self.pc += size;
    }
}

impl fmt::Display for RegisterFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "RISC-V Register File:")?;

        let width = if self.is_32_bit { 8 } else { 16 };

        for i in (0..32).step_by(4) {
            writeln!(f, "    r{:<2}: {:0width$X} | r{:<2}: {:0width$X} | r{:<2}: {:0width$X} | r{:<2}: {:0width$X}",
                i    , self.x[i + 0],
                i + 1, self.x[i + 1],
                i + 2, self.x[i + 2],
                i + 3, self.x[i + 3],
            )?;
        }

        write!(f, "    pc : {:0width$X}", self.pc)
    }
}