use std::fmt;

use emu_cpu::InstructionInfo;
use emu_macros::EnumCount;
use emu_utils::*;

use crate::registers::RegisterFile;

mod rv32i_instructions;
pub use rv32i_instructions::*;



mod zifencei_instructions;
pub use zifencei_instructions::*;

#[cfg(test)]
mod tests;


#[derive(Clone, Copy)]
pub struct InstructionEncoding32(pub u32);

// Encoding
//
// | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 |  9 |  8 |  7 |  6 |  5 |  4 |  3 |  2 |  1 |  0 |
// |              funct7              |           rs2          |           rs1          |    funct3    |           rd           |              opcode              | R-type
// |                         imm[11:0]                         |           rs1          |    funct3    |           rd           |              opcode              | I-type
// |             imm[11:5]            |           rs2          |           rs1          |    funct3    |        imm[4:0]        |              opcode              | S-type
// |[12]|          imm[10:5]          |           rs2          |           rs1          |    funct3    |      imm[4:1]     |[11]|              opcode              | B-type
// |                                             imm[31:12]                                            |           rd           |              opcode              | U-type
// |[20]|                     imm[10:1]                   |[11]|               imm[19:12]              |           rd           |              opcode              | J-type
//

impl InstructionEncoding32 {
    fn opcode(self) -> u8 {
        (self.0 & 0x3F) as u8
    }

    fn rd(self) -> u8 {
        ((self.0 >> 7) & 0x1F) as u8
    }

    fn rs1(self) -> u8 {
        ((self.0 >> 15) & 0x1F) as u8
    }

    fn rs2(self) -> u8 {
        ((self.0 >> 20) & 0x1F) as u8
    }

    fn funt3(self) -> u8 {
        ((self.0 >> 12) & 0x7) as u8
    }

    fn funt7(self) -> u8 {
        ((self.0 >> 25) & 0x7F) as u8
    }

    fn imm_i(self) -> u16 {
        ((self.0 >> 20) & 0xFFF) as u16
    }

    fn imm_s(self) -> u32 {
        let imm4_0 = (self.0 >> 7) & 0x1F;
        let imm11_5 = (self.0 >> 25) & 0x7F;
        (imm11_5 << 5) & imm4_0
    }

    fn imm_b(self) -> u32 {
        let imm11 = (self.0 >> 7) & 0x1;
        let imm4_1 = (self.0 >> 8) & 0xF;
        let imm10_5 = (self.0 >> 25) & 0x3F;
        let imm12 = (self.0 >> 31) & 0x1; 

        (imm12 << 12) | (imm11 << 11) | (imm10_5 << 5) | (imm4_1 << 1)   
    }

    fn imm_u(self) -> u32 {
        self.0 & 0xFFFF_F000
    }

    fn imm_j(self) -> u32 {
        let imm19_12 = (self.0 >> 12) & 0xFF;
        let imm11 = (self.0 >> 20) & 0x1;
        let imm10_1 = (self.0 >> 21) & 0x2FF;
        let imm20 = (self.0 >> 31) & 0x1;
        (imm20 << 20) | (imm19_12 << 12) | (imm11 << 11) | (imm10_1 << 1)
    }

    fn encode_r(funct7: u8, rs1: u8, rs2: u8, funct3: u8, rd: u8, opcode: u8) -> Self {
        Self (
            (funct7 as u32) << 25 |
            (rs1    as u32) << 20 |
            (rs2    as u32) << 15 |
            (funct3 as u32) << 12 |
            (rd     as u32) <<  7 |
            (opcode as u32)
        )
    }

    fn encode_i(imm: u16, rs1: u8, funct3: u8, rd: u8, opcode: u8) -> Self {
        Self (
            (imm    as u32) << 20 |
            (rs1    as u32) << 15 |
            (funct3 as u32) << 12 |
            (rd     as u32) <<  7 |
            (opcode as u32)
        )
    }

    fn encode_s(imm: u16, rs1: u8, rs2: u8, funct3: u8, opcode: u8) -> Self {
        Self (
            (((imm as u32) >> 5) & 0x7F) << 25 |
            (((imm as u32) >> 1) & 0x1F) <<  7 |
            (rs1    as u32) << 20 |
            (rs2    as u32) << 15 |
            (funct3 as u32) << 12 |
            (opcode as u32)
        )
    }

    fn encode_b(imm: u16, rs1: u8, rs2: u8, funct3: u8, opcode: u8) -> Self {
        Self (
            (((imm as u32) >> 12) & 0x1 ) << 31 |
            (((imm as u32) >> 5 ) & 0x3F) << 25 |
            (((imm as u32) >> 1 ) & 0xF ) <<  8 |
            (((imm as u32) >> 11) & 0x1 ) <<  7 |
            (rs1    as u32) << 20 |
            (rs2    as u32) << 15 |
            (funct3 as u32) << 12 |
            (opcode as u32)
        )
    }

    fn encode_u(imm: u32, rd: u8, opcode: u8) -> Self {
        Self (
            (imm as u32) & 0xFFFF_F000 |
            (rd     as u32) <<  7 |
            (opcode as u32)
        )
    }

    fn encode_j(imm: u32, rd: u8, opcode: u8) -> Self {
        Self (
            (((imm as u32) >> 20) & 0x1  ) << 31 |
            (((imm as u32) >>  1) & 0x2FF) << 21 |
            (((imm as u32) >> 11) & 0x1  ) << 20 |
            (((imm as u32) >> 12) & 0xFF ) << 12 |
            (rd     as u32) <<  7 |
            (opcode as u32)
        )
    }

    pub fn decode(self) -> Option<Instruction> {
        let rd = self.rd();
        let rs1 = self.rs1();
        let rs2 = self.rs2();

        match self.opcode() {
            0b0000011 => {
                match self.funt3() {
                    0b000 => Some(Instruction::RV32I(RV32IInstuction::LB { rd, rs1, imm: self.imm_i() })),
                    0b001 => Some(Instruction::RV32I(RV32IInstuction::LH { rd, rs1, imm: self.imm_i() })),
                    0b010 => Some(Instruction::RV32I(RV32IInstuction::LW { rd, rs1, imm: self.imm_i() })),
                    0b100 => Some(Instruction::RV32I(RV32IInstuction::LBU { rd, rs1, imm: self.imm_i() })),
                    0b101 => Some(Instruction::RV32I(RV32IInstuction::LHU { rd, rs1, imm: self.imm_i() })),
                    _ => None,
                }
            },
            0b0001111 => {
                match self.funt3() {
                    0b000 => {
                        let imm = self.imm_i();
                        let fm = (imm >> 8) as u8;
                        let pred = ((imm >> 4) & 0xF) as u8;
                        let succ = (imm & 0xF) as u8;
                        Some(Instruction::RV32I(RV32IInstuction::FENCE { rd, rs1, succ, pred, fm }))
                    },
                    0b001 => Some(Instruction::Zifencei(ZifenceiInstructions::FenceI)),
                    _ => None,
                }
            },
            0b0010011 => {
                match self.funt3() {
                    0b000 => Some(Instruction::RV32I(RV32IInstuction::ADDI { rd, rs1, imm: self.imm_i() })),
                    0b010 => Some(Instruction::RV32I(RV32IInstuction::SLTI { rd, rs1, imm: self.imm_i() })),
                    0b011 => Some(Instruction::RV32I(RV32IInstuction::SLTI { rd, rs1, imm: self.imm_i() })),
                    0b001 => {
                        let imm = self.imm_i();
                        assert!((imm & !31) == 0);
                        Some(Instruction::RV32I(RV32IInstuction::SLLI { rd, rs1, imm: imm as u8 }))
                    }
                    0b101 => {
                        let imm = self.imm_i();
                        assert!((imm & !0b010000011111) == 0);
                        if imm & 0b010000000000 == 0 {
                            Some(Instruction::RV32I(RV32IInstuction::SRLI { rd, rs1, imm: imm as u8 }))
                        } else {
                            Some(Instruction::RV32I(RV32IInstuction::SRAI { rd, rs1, imm: imm as u8 }))
                        }

                    }
                    _ => None,
                }
            },  
            0b0010111 => Some(Instruction::RV32I(RV32IInstuction::AUIPC { rd, imm: self.imm_u() })),
            0b0100011 => {
                match self.funt3() {
                    0b000 => Some(Instruction::RV32I(RV32IInstuction::SB { rs1, rs2, imm: self.imm_s() as u16 })),
                    0b001 => Some(Instruction::RV32I(RV32IInstuction::SH { rs1, rs2, imm: self.imm_s() as u16 })),
                    0b010 => Some(Instruction::RV32I(RV32IInstuction::SW { rs1, rs2, imm: self.imm_s() as u16 })),
                    _ => None,
                }
            },
            0b0110011 => {
                match self.funt3() {
                    0b000 => {
                        let funct7 = self.funt7();
                        assert!((funct7 & !0b0100000) == 0);
                        if funct7 & 0b0100000 == 0 {
                            Some(Instruction::RV32I(RV32IInstuction::ADD { rd, rs1, rs2 }))
                        } else {
                            Some(Instruction::RV32I(RV32IInstuction::SUB { rd, rs1, rs2 }))
                        }
                    },
                    0b001 => Some(Instruction::RV32I(RV32IInstuction::SLL { rd, rs1, rs2 })),
                    0b010 => Some(Instruction::RV32I(RV32IInstuction::SLT { rd, rs1, rs2 })),
                    0b011 => Some(Instruction::RV32I(RV32IInstuction::SLTU { rd, rs1, rs2 })),
                    0b100 => Some(Instruction::RV32I(RV32IInstuction::XOR { rd, rs1, rs2 })),
                    0b101 => {
                        let funct7 = self.funt7();
                        assert!((funct7 & !0b0100000) == 0);
                        if funct7 & 0b0100000 == 0 {
                            Some(Instruction::RV32I(RV32IInstuction::SRL { rd, rs1, rs2 }))
                        } else {
                            Some(Instruction::RV32I(RV32IInstuction::SRA { rd, rs1, rs2 }))
                        }
                    },
                    0b110 => Some(Instruction::RV32I(RV32IInstuction::OR { rd, rs1, rs2 })),
                    0b111 => Some(Instruction::RV32I(RV32IInstuction::AND { rd, rs1, rs2 })),
                    _ => None,
                }
            },
            0b0110111 => Some(Instruction::RV32I(RV32IInstuction::LUI { rd, imm: self.imm_u() })),
            0b1100011 => {
                match self.funt3() {
                    0b000 => Some(Instruction::RV32I(RV32IInstuction::BEQ  { rs1, rs2, imm: self.imm_b() as u16 })),
                    0b001 => Some(Instruction::RV32I(RV32IInstuction::BNE  { rs1, rs2, imm: self.imm_b() as u16 })),
                    0b100 => Some(Instruction::RV32I(RV32IInstuction::BLT  { rs1, rs2, imm: self.imm_b() as u16 })),
                    0b101 => Some(Instruction::RV32I(RV32IInstuction::BLTU { rs1, rs2, imm: self.imm_b() as u16 })),
                    0b110 => Some(Instruction::RV32I(RV32IInstuction::BGE  { rs1, rs2, imm: self.imm_b() as u16 })),
                    0b111 => Some(Instruction::RV32I(RV32IInstuction::BGEU { rs1, rs2, imm: self.imm_b() as u16 })),
                    _ => None,
                }
            },
            0b1100111 => Some(Instruction::RV32I(RV32IInstuction::JALR { rd, rs1, imm: self.imm_i() })),
            0b1101111 => Some(Instruction::RV32I(RV32IInstuction::JAL { rd, imm: self.imm_j() })),
            _ => None,
        }
    }

    pub fn encode(instr: Instruction) -> Self {
        match instr {
            Instruction::RV32I(instr) => match instr {
                RV32IInstuction::ADDI  { rd, rs1, imm  } => Self::encode_i(imm, rs1, 0b000, rd, 0b0010011),
                RV32IInstuction::SLTI  { rd, rs1, imm  } => Self::encode_i(imm, rs1, 0b010, rd, 0b0010011),
                RV32IInstuction::SLTIU { rd, rs1, imm  } => Self::encode_i(imm, rs1, 0b011, rd, 0b0010011),
                RV32IInstuction::XORI  { rd, rs1, imm  } => Self::encode_i(imm, rs1, 0b100, rd, 0b0010011),
                RV32IInstuction::ORI   { rd, rs1, imm  } => Self::encode_i(imm, rs1, 0b110, rd, 0b0010011),
                RV32IInstuction::ANDI  { rd, rs1, imm  } => Self::encode_i(imm, rs1, 0b111, rd, 0b0010011),
                RV32IInstuction::SLLI  { rd, rs1, imm  } => Self::encode_i(0b0000000_00000 | imm as u16, rs1, 0b001, rd, 0b0010011),
                RV32IInstuction::SRLI  { rd, rs1, imm  } => Self::encode_i(0b0000000_00000 | imm as u16, rs1, 0b101, rd, 0b0010011),
                RV32IInstuction::SRAI  { rd, rs1, imm  } => Self::encode_i(0b0100000_00000 | imm as u16, rs1, 0b101, rd, 0b0010011),
                RV32IInstuction::LUI   { rd, imm       } => Self::encode_u(imm, rd, 0b0110111),
                RV32IInstuction::AUIPC { rd, imm       } => Self::encode_u(imm, rd, 0b0010111),
                RV32IInstuction::ADD   { rd, rs1, rs2  } => Self::encode_r(0b0000000, rs1, rs2, 0b000, rd, 0b0110011),
                RV32IInstuction::SUB   { rd, rs1, rs2  } => Self::encode_r(0b0100000, rs1, rs2, 0b000, rd, 0b0110011),
                RV32IInstuction::SLL   { rd, rs1, rs2  } => Self::encode_r(0b0000000, rs1, rs2, 0b001, rd, 0b0110011),
                RV32IInstuction::SLT   { rd, rs1, rs2  } => Self::encode_r(0b0000000, rs1, rs2, 0b010, rd, 0b0110011),
                RV32IInstuction::SLTU  { rd, rs1, rs2  } => Self::encode_r(0b0000000, rs1, rs2, 0b011, rd, 0b0110011),
                RV32IInstuction::XOR   { rd, rs1, rs2  } => Self::encode_r(0b0000000, rs1, rs2, 0b100, rd, 0b0110011),
                RV32IInstuction::SRL   { rd, rs1, rs2  } => Self::encode_r(0b0000000, rs1, rs2, 0b101, rd, 0b0110011),
                RV32IInstuction::SRA   { rd, rs1, rs2  } => Self::encode_r(0b0100000, rs1, rs2, 0b101, rd, 0b0110011),
                RV32IInstuction::OR    { rd, rs1, rs2  } => Self::encode_r(0b0000000, rs1, rs2, 0b110, rd, 0b0110011),
                RV32IInstuction::AND   { rd, rs1, rs2  } => Self::encode_r(0b0000000, rs1, rs2, 0b111, rd, 0b0110011),
                RV32IInstuction::JAL   { rd, imm       } => Self::encode_j(imm, rd, 0b1101111),
                RV32IInstuction::JALR  { rd, rs1, imm  } => Self::encode_i(imm, rs1, 0b000, rd, 0b1100111),
                RV32IInstuction::BEQ   { rs1, rs2, imm } => Self::encode_b(imm, rs1, rs2, 0b000, 0b1100011),
                RV32IInstuction::BNE   { rs1, rs2, imm } => Self::encode_b(imm, rs1, rs2, 0b001, 0b1100011),
                RV32IInstuction::BLT   { rs1, rs2, imm } => Self::encode_b(imm, rs1, rs2, 0b100, 0b1100011),
                RV32IInstuction::BLTU  { rs1, rs2, imm } => Self::encode_b(imm, rs1, rs2, 0b101, 0b1100011),
                RV32IInstuction::BGE   { rs1, rs2, imm } => Self::encode_b(imm, rs1, rs2, 0b110, 0b1100011),
                RV32IInstuction::BGEU  { rs1, rs2, imm } => Self::encode_b(imm, rs1, rs2, 0b111, 0b1100011),
                RV32IInstuction::LB    { rd, rs1, imm  } => Self::encode_i(imm, rs1, 0b000, rd, 0b0000011),
                RV32IInstuction::LH    { rd, rs1, imm  } => Self::encode_i(imm, rs1, 0b001, rd, 0b0000011),
                RV32IInstuction::LW    { rd, rs1, imm  } => Self::encode_i(imm, rs1, 0b010, rd, 0b0000011),
                RV32IInstuction::LBU   { rd, rs1, imm  } => Self::encode_i(imm, rs1, 0b100, rd, 0b0000011),
                RV32IInstuction::LHU   { rd, rs1, imm  } => Self::encode_i(imm, rs1, 0b101, rd, 0b0000011),
                RV32IInstuction::SB    { rs1, rs2, imm } => Self::encode_s(imm, rs1, rs2, 0b000, 0b0100011),
                RV32IInstuction::SH    { rs1, rs2, imm } => Self::encode_s(imm, rs1, rs2, 0b001, 0b0100011),
                RV32IInstuction::SW    { rs1, rs2, imm } => Self::encode_s(imm, rs1, rs2, 0b010, 0b0100011),
                RV32IInstuction::FENCE { rd, rs1, succ, pred, fm } => Self::encode_i(((fm as u16) << 8) | ((succ as u16) << 4) | (pred as u16), rs1, 0b000, rd, 0b0001111),
                RV32IInstuction::ECALL                   => Self::encode_i(0b000000000000, 0, 0b000, 0, 0b1110011),
                RV32IInstuction::EBREAK                  => Self::encode_i(0b000000000001, 0, 0b000, 0, 0b1110011),
            },
            Instruction::Zifencei(instr) => match instr {
                ZifenceiInstructions::FenceI => Self::encode_i(0, 0, 0b001, 0, 0b0001111),
            },
        }
    }
}











#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    RV32I(RV32IInstuction),




    Zifencei(ZifenceiInstructions),
}

impl Instruction {
    pub fn exec(&self, register_file: &mut RegisterFile, memory: &mut [u8]) {
        match self {
            Instruction::RV32I(instr) => instr.exec(register_file, memory),
            Instruction::Zifencei(instr) => instr.exec(register_file, memory),
        }
    }

    pub fn log(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        let encoded = InstructionEncoding32::encode(*self);
        write!(f, "{:X} | ", encoded.0)?;

        match self {
            Instruction::RV32I(instr) => instr.log(f),
            Instruction::Zifencei(instr) => instr.log(f),
        }
    }
}
