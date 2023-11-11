use std::fmt;

use emu_cpu::InstructionInfo;
use emu_macros::EnumCount;
use emu_utils::*;

use crate::registers::RegisterFile;

use super::InstructionEncoding32;

#[derive(Clone, Copy, PartialEq, Eq, Debug, EnumCount)]
pub enum RV32IInstuction {

    ADDI  { rd: u8, rs1: u8, imm: u16 },
    SLTI  { rd: u8, rs1: u8, imm: u16 },
    SLTIU { rd: u8, rs1: u8, imm: u16 },
    XORI  { rd: u8, rs1: u8, imm: u16 },
    ORI   { rd: u8, rs1: u8, imm: u16 },
    ANDI  { rd: u8, rs1: u8, imm: u16 },
    SLLI  { rd: u8, rs1: u8, imm: u8 },
    SRLI  { rd: u8, rs1: u8, imm: u8 },
    SRAI  { rd: u8, rs1: u8, imm: u8 },
    
    LUI   { rd: u8, imm: u32 },
    AUIPC { rd: u8, imm: u32 },

    ADD   { rd: u8, rs1: u8, rs2: u8 },
    SUB   { rd: u8, rs1: u8, rs2: u8 },
    SLL   { rd: u8, rs1: u8, rs2: u8 },
    SLT   { rd: u8, rs1: u8, rs2: u8 },
    SLTU  { rd: u8, rs1: u8, rs2: u8 },
    XOR   { rd: u8, rs1: u8, rs2: u8 },
    SRL   { rd: u8, rs1: u8, rs2: u8 },
    SRA   { rd: u8, rs1: u8, rs2: u8 },
    OR    { rd: u8, rs1: u8, rs2: u8 },
    AND   { rd: u8, rs1: u8, rs2: u8 },

    JAL   { rd: u8, imm: u32 },
    JALR  { rd: u8, rs1: u8, imm: u16 },

    BEQ   { rs1: u8, rs2: u8, imm: u16 },
    BNE   { rs1: u8, rs2: u8, imm: u16 },
    BLT   { rs1: u8, rs2: u8, imm: u16 },
    BLTU  { rs1: u8, rs2: u8, imm: u16 },
    BGE   { rs1: u8, rs2: u8, imm: u16 },
    BGEU  { rs1: u8, rs2: u8, imm: u16 },

    LB    { rd: u8, rs1: u8, imm: u16 },
    LH    { rd: u8, rs1: u8, imm: u16 },
    LW    { rd: u8, rs1: u8, imm: u16 },
    LBU   { rd: u8, rs1: u8, imm: u16 },
    LHU   { rd: u8, rs1: u8, imm: u16 },

    SB    { rs1: u8, rs2: u8, imm: u16 },
    SH    { rs1: u8, rs2: u8, imm: u16 },
    SW    { rs1: u8, rs2: u8, imm: u16 },

    FENCE { rd: u8, rs1: u8, succ: u8, pred: u8, fm: u8 },

    ECALL,
    EBREAK,
}

// |                         imm[11:0]                         |           rs1          |    funct3    |           rd           |              opcode              | I-type
// |             imm[11:5]            |           rs2          |           rs1          |    funct3    |        imm[4:0]        |              opcode              | S-type

impl RV32IInstuction {
    pub fn exec(&self, register_file: &mut RegisterFile, memory: &mut [u8]) {
        match *self {
            Self::ADDI { rd, rs1, imm } => {
                if rd == 0 {
                    // Hint, only indrement pc and return
                    register_file.inc_pc(4);
                    return;
                }

                let src = register_file.read_x_register_sign_extended(rs1);
                let imm = sign_extend_64(imm as u64, 11);
                let res = src + imm;
                register_file.write_x_register(rd, res);
                register_file.inc_pc(4);
            },
            Self::SLTI { rd, rs1, imm } => {
                if rd == 0 {
                    // Hint, only indrement pc and return
                    register_file.inc_pc(4);
                    return;
                }

                let src = register_file.read_x_register_sign_extended(rs1) as i64;
                let imm = sign_extend_64(imm as u64, 11) as i64;
                let res = if src < imm { 1 } else { 0 };
                register_file.write_x_register(rd, res);
                register_file.inc_pc(4);
            },
            Self::SLTIU { rd, rs1, imm } => {
                if rd == 0 {
                    // Hint, only indrement pc and return
                    register_file.inc_pc(4);
                    return;
                }

                let src = register_file.read_x_register_sign_extended(rs1);
                let imm = sign_extend_64(imm as u64, 11);
                let res = if src < imm { 1 } else { 0 };
                register_file.write_x_register(rd, res);
                register_file.inc_pc(4);
            },
            Self::XORI { rd, rs1, imm } => {
                if rd == 0 {
                    // Hint, only indrement pc and return
                    register_file.inc_pc(4);
                    return;
                }

                let src = register_file.read_x_register(rs1);
                let imm = sign_extend_64(imm as u64, 11);
                let res = src ^ imm;
                register_file.write_x_register(rd, res);
                register_file.inc_pc(4);
            },
            Self::ORI { rd, rs1, imm } => {
                if rd == 0 {
                    // Hint, only indrement pc and return
                    register_file.inc_pc(4);
                    return;
                }

                let src = register_file.read_x_register(rs1);
                let imm = sign_extend_64(imm as u64, 11);
                let res = src | imm;
                register_file.write_x_register(rd, res);
                register_file.inc_pc(4);
            },
            Self::ANDI { rd, rs1, imm } => {
                if rd == 0 {
                    // Hint, only indrement pc and return
                    register_file.inc_pc(4);
                    return;
                }

                let src = register_file.read_x_register(rs1);
                let imm = sign_extend_64(imm as u64, 11);
                let res = src & imm;
                register_file.write_x_register(rd, res);
                register_file.inc_pc(4);
            },
            Self::SLLI { rd, rs1, imm } => {
                if rd == 0 {
                    // Hint, only indrement pc and return
                    register_file.inc_pc(4);
                    return;
                }

                let src = register_file.read_x_register(rs1);
                let res = src << imm;
                register_file.write_x_register(rd, res);
                register_file.inc_pc(4);
            },
            Self::SRLI { rd, rs1, imm } => {
                if rd == 0 {
                    // Hint, only indrement pc and return
                    register_file.inc_pc(4);
                    return;
                }

                let src = register_file.read_x_register(rs1);
                let res = src >> imm;
                register_file.write_x_register(rd, res);
                register_file.inc_pc(4);
            },
            Self::SRAI { rd, rs1, imm } => {
                if rd == 0 {
                    // Hint, only indrement pc and return
                    register_file.inc_pc(4);
                    return;
                }

                let src = register_file.read_x_register(rs1);
                let res = if register_file.is_32_bit() { ((src as i32) >>imm) as u64 } else { ((src as i64) >> imm) as u64 };
                register_file.write_x_register(rd, res);
                register_file.inc_pc(4);
            },
            Self::LUI { rd, imm } => {
                if rd == 0 {
                    // Hint, only indrement pc and return
                    register_file.inc_pc(4);
                    return;
                }

                register_file.write_x_register(rd, (imm << 12) as u64);
                register_file.inc_pc(4);
            },
            Self::AUIPC { rd, imm } => {
                if rd == 0 {
                    // Hint, only indrement pc and return
                    register_file.inc_pc(4);
                    return;
                }

                let pc = register_file.read_pc(); // Address of AUIPC instruction
                let res = (imm << 12) as u64 + pc;
                register_file.write_x_register(rd, res);
                register_file.inc_pc(4);
            },
            Self::ADD { rd, rs1, rs2 } => {
                if rd == 0 {
                    // Hint, only indrement pc and return
                    register_file.inc_pc(4);
                    return;
                }

                let src1 = register_file.read_x_register_sign_extended(rs1);
                let src2 = register_file.read_x_register_sign_extended(rs2);
                let res = src1.wrapping_add(src2);
                register_file.write_x_register(rd, res);
                register_file.inc_pc(4);
            },
            Self::SUB { rd, rs1, rs2 } => {
                if rd == 0 {
                    // Hint, only indrement pc and return
                    register_file.inc_pc(4);
                    return;
                }

                let src1 = register_file.read_x_register_sign_extended(rs1);
                let src2 = register_file.read_x_register_sign_extended(rs2);
                let res = src1.wrapping_sub(src2);
                register_file.write_x_register(rd, res);
                register_file.inc_pc(4);
            },
            Self::SLL { rd, rs1, rs2 } => {
                if rd == 0 {
                    // Hint, only indrement pc and return
                    register_file.inc_pc(4);
                    return;
                }

                let src1 = register_file.read_x_register(rs1);
                let src2 = register_file.read_x_register(rs2);
                let res = src1 << (src2 & if register_file.is_32_bit() { 0x1F } else { 0x3F });
                register_file.write_x_register(rd, res);
                register_file.inc_pc(4);
            },
            Self::SLT { rd, rs1, rs2 } => {
                if rd == 0 {
                    // Hint, only indrement pc and return
                    register_file.inc_pc(4);
                    return;
                }

                let src1 = register_file.read_x_register_sign_extended(rs1) as i64;
                let src2 = register_file.read_x_register_sign_extended(rs2) as i64;
                let res = if src1 < src2 { 1 } else { 0 };
                register_file.write_x_register(rd, res);
                register_file.inc_pc(4);
            },
            Self::SLTU { rd, rs1, rs2 } => {
                if rd == 0 {
                    // Hint, only indrement pc and return
                    register_file.inc_pc(4);
                    return;
                }

                let src1 = register_file.read_x_register_sign_extended(rs1);
                let src2 = register_file.read_x_register_sign_extended(rs2);
                let res = if src1 < src2 { 1 } else { 0 };
                register_file.write_x_register(rd, res);
                register_file.inc_pc(4);
            },
            Self::XOR { rd, rs1, rs2 } => {
                if rd == 0 {
                    // Hint, only indrement pc and return
                    register_file.inc_pc(4);
                    return;
                }

                let src1 = register_file.read_x_register(rs1);
                let src2 = register_file.read_x_register(rs2);
                let res = src1 ^ src2;
                register_file.write_x_register(rd, res);
                register_file.inc_pc(4);
            },
            Self::SRL { rd, rs1, rs2 } => {
                if rd == 0 {
                    // Hint, only indrement pc and return
                    register_file.inc_pc(4);
                    return;
                }

                let src1 = register_file.read_x_register(rs1);
                let src2 = register_file.read_x_register(rs2);
                let res = src1 >> (src2 & if register_file.is_32_bit() { 0x1F } else { 0x3F });
                register_file.write_x_register(rd, res);
                register_file.inc_pc(4);
            },
            Self::SRA { rd, rs1, rs2 } => {
                if rd == 0 {
                    // Hint, only indrement pc and return
                    register_file.inc_pc(4);
                    return;
                }

                let src1 = register_file.read_x_register(rs1);
                let src2 = register_file.read_x_register(rs2);
                let res = (src1 as i64) >> (src2 & if register_file.is_32_bit() { 0x1F } else { 0x3F });
                register_file.write_x_register(rd, res as u64);
                register_file.inc_pc(4);
            },
            Self::OR { rd, rs1, rs2 } => {
                if rd == 0 {
                    // Hint, only indrement pc and return
                    register_file.inc_pc(4);
                    return;
                }

                let src1 = register_file.read_x_register(rs1);
                let src2 = register_file.read_x_register(rs2);
                let res = src1 | src2;
                register_file.write_x_register(rd, res);
                register_file.inc_pc(4);
            },
            Self::AND { rd, rs1, rs2 } => {
                if rd == 0 {
                    // Hint, only indrement pc and return
                    register_file.inc_pc(4);
                    return;
                }

                let src1 = register_file.read_x_register(rs1);
                let src2 = register_file.read_x_register(rs2);
                let res = src1 & src2;
                register_file.write_x_register(rd, res);
                register_file.inc_pc(4);
            },
            Self::JAL { rd, imm } => {
                let offset = sign_extend_64(imm as u64, 20);
                if rd != 0 {
                    let pc = register_file.read_pc();
                    register_file.write_x_register(rd, pc + 4);
                }
                register_file.offset_pc(offset);
            },
            Self::JALR { rd, rs1, imm } => {
                let src = register_file.read_x_register(rs1);
                let imm = sign_extend_64(imm as u64, 11);
                let offset = src.wrapping_add(imm) & !1;
                if rd != 0 {
                    let pc = register_file.read_pc();
                    register_file.write_x_register(rd, pc + 4);
                }
                register_file.offset_pc(offset);
            },
            Self::BEQ { rs1, rs2, imm } => {
                let src1 = register_file.read_x_register(rs1);
                let src2 = register_file.read_x_register(rs2);
                if src1 == src2 {
                    let offset = sign_extend_64(imm as u64, 11);
                    register_file.offset_pc(offset);
                } else {
                    register_file.inc_pc(4);
                }
            },
            Self::BNE { rs1, rs2, imm } => {
                let src1 = register_file.read_x_register(rs1);
                let src2 = register_file.read_x_register(rs2);
                if src1 != src2 {
                    let offset = sign_extend_64(imm as u64, 11);
                    register_file.offset_pc(offset);
                } else {
                    register_file.inc_pc(4);
                }
            },
            Self::BLT { rs1, rs2, imm } => {
                let src1 = register_file.read_x_register(rs1) as i64;
                let src2 = register_file.read_x_register(rs2) as i64;
                if src1 < src2 {
                    let offset = sign_extend_64(imm as u64, 11);
                    register_file.offset_pc(offset);
                } else {
                    register_file.inc_pc(4);
                }
            },
            Self::BLTU { rs1, rs2, imm } => {
                let src1 = register_file.read_x_register_sign_extended(rs1);
                let src2 = register_file.read_x_register_sign_extended(rs2);
                if src1 < src2 {
                    let offset = sign_extend_64(imm as u64, 11);
                    register_file.offset_pc(offset);
                } else {
                    register_file.inc_pc(4);
                }
            },
            Self::BGE { rs1, rs2, imm } => {
                let src1 = register_file.read_x_register(rs1) as i64;
                let src2 = register_file.read_x_register(rs2) as i64;
                if src1 >= src2 {
                    let offset = sign_extend_64(imm as u64, 11);
                    register_file.offset_pc(offset);
                } else {
                    register_file.inc_pc(4);
                }
            },
            Self::BGEU { rs1, rs2, imm } => {
                let src1 = register_file.read_x_register_sign_extended(rs1);
                let src2 = register_file.read_x_register_sign_extended(rs2);
                if src1 >= src2 {
                    let offset = sign_extend_64(imm as u64, 11);
                    register_file.offset_pc(offset);
                } else {
                    register_file.inc_pc(4);
                }
            },
            Self::LB { rd, rs1, imm } => {
                let src = register_file.read_x_register(rs1);
                let imm = sign_extend_64(imm as u64, 11);
                let addr = src.wrapping_add(imm);
                let val = memory[addr as usize] as u64;
                let val = sign_extend_64(val, 7);
                register_file.write_x_register(rd, val);
                register_file.inc_pc(4);
            },
            Self::LH { rd, rs1, imm } => {
                let src = register_file.read_x_register(rs1);
                let imm = sign_extend_64(imm as u64, 11);
                let addr = src.wrapping_add(imm);

                assert!(addr as usize + 1 < memory.len());
                let val = unsafe { *(memory.as_ptr().add(addr as usize) as *const u16) } as u64;
                let val = sign_extend_64(val, 15);
                register_file.write_x_register(rd, val);
                register_file.inc_pc(4);
            },
            Self::LW { rd, rs1, imm } => {
                let src = register_file.read_x_register(rs1);
                let imm = sign_extend_64(imm as u64, 11);
                let addr = src.wrapping_add(imm);

                assert!(addr as usize + 1 < memory.len());
                let val = unsafe { *(memory.as_ptr().add(addr as usize) as *const u32) } as u64;
                let val = sign_extend_64(val, 31);
                register_file.write_x_register(rd, val);
                register_file.inc_pc(4);
            },
            Self::LBU { rd, rs1, imm } => {
                let src = register_file.read_x_register(rs1);
                let imm = sign_extend_64(imm as u64, 11);
                let addr = src.wrapping_add(imm);
                let val = memory[addr as usize] as u64;
                register_file.write_x_register(rd, val);
                register_file.inc_pc(4);
            },
            Self::LHU { rd, rs1, imm } => {
                let src = register_file.read_x_register(rs1);
                let imm = sign_extend_64(imm as u64, 11);
                let addr = src.wrapping_add(imm);

                assert!(addr as usize + 1 < memory.len());
                let val = unsafe { *(memory.as_ptr().add(addr as usize) as *const u16) } as u64;
                register_file.write_x_register(rd, val);
                register_file.inc_pc(4);
            },
            Self::SB { rs1, rs2, imm } => {
                let src1 = register_file.read_x_register(rs1);
                let src2 = register_file.read_x_register(rs2);
                let imm = sign_extend_64(imm as u64, 11);
                let addr = src1.wrapping_add(imm);
                memory[addr as usize] = src2 as u8;
                register_file.inc_pc(4);
            },
            Self::SH { rs1, rs2, imm } => {
                let src1 = register_file.read_x_register(rs1);
                let src2 = register_file.read_x_register(rs2);
                let imm = sign_extend_64(imm as u64, 11);
                let addr = src1.wrapping_add(imm);
                assert!(addr as usize + 1 < memory.len());
                let addr = unsafe { &mut *(memory.as_mut_ptr().add(addr as usize) as *mut u16) };
                *addr = src2 as u16;
                register_file.inc_pc(4);
            },
            Self::SW { rs1, rs2, imm } => {
                let src1 = register_file.read_x_register(rs1);
                let src2 = register_file.read_x_register(rs2);
                let imm = sign_extend_64(imm as u64, 11);
                let addr = src1.wrapping_add(imm);
                assert!(addr as usize + 1 < memory.len());
                let addr = unsafe { &mut *(memory.as_mut_ptr().add(addr as usize) as *mut u32) };
                *addr = src2 as u32;
                register_file.inc_pc(4);
            },
            RV32IInstuction::FENCE { .. } => {}, // Dummy: no used in emulator atm,
            RV32IInstuction::ECALL => {}, // Dummy, not yet implemented,
            RV32IInstuction::EBREAK => {}, // Dummy, not yet implemented, should pause interpreter for debugging purposes
            
        }
    }

    pub fn log(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        match self {
            RV32IInstuction::ADDI  { rd, rs1, imm }  => write!(f, "addi x{rd}, x{rs1}, {imm}"),
            RV32IInstuction::SLTI  { rd, rs1, imm }  => write!(f, "slti x{rd}, x{rs1}, {imm}"),
            RV32IInstuction::SLTIU { rd, rs1, imm }  => write!(f, "sltiu x{rd}, x{rs1}, {imm}"),
            RV32IInstuction::XORI  { rd, rs1, imm }  => write!(f, "xori x{rd}, x{rs1}, {imm}"),
            RV32IInstuction::ORI   { rd, rs1, imm }  => write!(f, "ori x{rd}, x{rs1}, {imm}"),
            RV32IInstuction::ANDI  { rd, rs1, imm }  => write!(f, "andi x{rd}, x{rs1}, {imm}"),
            RV32IInstuction::SLLI  { rd, rs1, imm }  => write!(f, "slli x{rd}, x{rs1}, {imm}"),
            RV32IInstuction::SRLI  { rd, rs1, imm }  => write!(f, "srli x{rd}, x{rs1}, {imm}"),
            RV32IInstuction::SRAI  { rd, rs1, imm }  => write!(f, "srai x{rd}, x{rs1}, {imm}"),
            RV32IInstuction::LUI   { rd, imm }       => write!(f, "lui x{rd}, {imm:X}"),
            RV32IInstuction::AUIPC { rd, imm }       => write!(f, "auipc x{rd}, {imm:X}"),
            RV32IInstuction::ADD   { rd, rs1, rs2 }  => write!(f, "add x{rd}, x{rs1}, x{rs2}"),
            RV32IInstuction::SUB   { rd, rs1, rs2 }  => write!(f, "sub x{rd}, x{rs1}, x{rs2}"),
            RV32IInstuction::SLL   { rd, rs1, rs2 }  => write!(f, "sll x{rd}, x{rs1}, x{rs2}"),
            RV32IInstuction::SLT   { rd, rs1, rs2 }  => write!(f, "slt x{rd}, x{rs1}, x{rs2}"),
            RV32IInstuction::SLTU  { rd, rs1, rs2 }  => write!(f, "sltu x{rd}, x{rs1}, x{rs2}"),
            RV32IInstuction::XOR   { rd, rs1, rs2 }  => write!(f, "xor x{rd}, x{rs1}, x{rs2}"),
            RV32IInstuction::SRL   { rd, rs1, rs2 }  => write!(f, "srl x{rd}, x{rs1}, x{rs2}"),
            RV32IInstuction::SRA   { rd, rs1, rs2 }  => write!(f, "sra x{rd}, x{rs1}, x{rs2}"),
            RV32IInstuction::OR    { rd, rs1, rs2 }  => write!(f, "or x{rd}, x{rs1}, x{rs2}"),
            RV32IInstuction::AND   { rd, rs1, rs2 }  => write!(f, "and x{rd}, x{rs1}, x{rs2}"),
            RV32IInstuction::JAL   { rd, imm }       => write!(f, "jal x{rd} {imm:X}"),
            RV32IInstuction::JALR  { rd, rs1, imm }  => write!(f, "jalr x{rd}, {rs1} {imm:X}"),
            RV32IInstuction::BEQ   { rs1, rs2, imm } => write!(f, "beq x{rs1}, x{rs2}, {imm:X}"),
            RV32IInstuction::BNE   { rs1, rs2, imm } => write!(f, "bne x{rs1}, x{rs2}, {imm:X}"),
            RV32IInstuction::BLT   { rs1, rs2, imm } => write!(f, "blt x{rs1}, x{rs2}, {imm:X}"),
            RV32IInstuction::BLTU  { rs1, rs2, imm } => write!(f, "bltu x{rs1}, x{rs2}, {imm:X}"),
            RV32IInstuction::BGE   { rs1, rs2, imm } => write!(f, "bge x{rs1}, x{rs2}, {imm:X}"),
            RV32IInstuction::BGEU  { rs1, rs2, imm } => write!(f, "bgeu x{rs1}, x{rs2}, {imm:X}"),
            RV32IInstuction::LB    { rd, rs1, imm }  => write!(f, "lb x{rd}, x{rs1}, {imm:X}"),
            RV32IInstuction::LH    { rd, rs1, imm }  => write!(f, "lh x{rd}, x{rs1}, {imm:X}"),
            RV32IInstuction::LW    { rd, rs1, imm }  => write!(f, "le x{rd}, x{rs1}, {imm:X}"),
            RV32IInstuction::LBU   { rd, rs1, imm }  => write!(f, "lbu x{rd}, x{rs1}, {imm:X}"),
            RV32IInstuction::LHU   { rd, rs1, imm }  => write!(f, "lhb x{rd}, x{rs1}, {imm:X}"),
            RV32IInstuction::SB    { rs1, rs2, imm } => write!(f, "sb x{rs1}, x{rs2}, {imm:X}"),
            RV32IInstuction::SH    { rs1, rs2, imm } => write!(f, "sh x{rs1}, x{rs2}, {imm:X}"),
            RV32IInstuction::SW    { rs1, rs2, imm } => write!(f, "sw x{rs1}, x{rs2}, {imm:X}"),
            RV32IInstuction::FENCE { succ, pred, fm, .. } => write!(f, "fence {succ:b}, {pred:b}, {fm:b}"),
            RV32IInstuction::ECALL  => write!(f, "ecall"),
            RV32IInstuction::EBREAK => write!(f, "ebreak"),
        }
    }
}



pub const RV32I_INSTUCTION_INFO : [InstructionInfo; RV32IInstuction::COUNT] = [
    InstructionInfo { name: "ADDI"  , mnemonic: "addi rd, rs1, imm" , encoding: "I-Type:   iiiiiiiiiiii_aaaaa_000_ddddd_0010011", desc: "Adds the sign extended 12-bit immediate to register rs1. Arithmetic overflow is ignored and the result is simply the low XLEN bits of the result. `ADDI rd, rs1, 0` is used to implement the `MV rd rs1` assembler pseudo-instruction."  },
    InstructionInfo { name: "STLI"  , mnemonic: "stli rd, rs1, imm" , encoding: "I-Type:   iiiiiiiiiiii_aaaaa_010_ddddd_0010011", desc: "Set less than immediate. Places the value 1 in register rd if register rs1 is less than the sign-extended immediate when both are treated as signed numbers, else 0 is written to rd." },
    InstructionInfo { name: "STLIU" , mnemonic: "stliu rd, rs1, imm", encoding: "I-Type:   iiiiiiiiiiii_aaaaa_011_ddddd_0010011", desc: "Set less than immediate. Places the value 1 in register rd if register rs1 is less than the sign-extended immediate when both are treated as unsigned numbers, else 0 is written to rd. Note: `SLTIU rd, rs1, 1` sets rd to 1 if rs1 equals zero, otehrwise sets rd to 0 (assembler pseudo-instruction `SEQZ rd, rs`)." },
    InstructionInfo { name: "XORI"  , mnemonic: "andi rd, rs1, imm" , encoding: "I-Type:   iiiiiiiiiiii_aaaaa_100_ddddd_0010011", desc: "Logical operation that performs a bitwise XOR on register rs1 and the sign-extended 12-bit immediate and places the result in rd. Note: `XORI rd, rs1, -1` performs a bitwise logical inversion of register rs1 (assember pseudo-instruction `NOT rd, rs`)." },
    InstructionInfo { name: "ORI"   , mnemonic: "ori rd, rs1, imm"  , encoding: "I-Type:   iiiiiiiiiiii_aaaaa_110_ddddd_0010011", desc: "Logical operation that performs a bitwise OR on register rs1 and the sign-extended 12-bit immediate and places the result in rd." },
    InstructionInfo { name: "ANDI"  , mnemonic: "xori rd, rs1, imm" , encoding: "I-Type:   iiiiiiiiiiii_aaaaa_111_ddddd_0010011", desc: "Logical operation that performs a bitwise AND on register rs1 and the sign-extended 12-bit immediate and places the result in rd." },
    InstructionInfo { name: "SLLI"  , mnemonic: "slli rd, rs1, imm" , encoding: "I-Type:   0000000iiiii_aaaaa_001_ddddd_0010011", desc: "Logical left shift (zeroes are shifted into the lower bits)."  },
    InstructionInfo { name: "SRLI"  , mnemonic: "srli rd, rs1, imm" , encoding: "I-Type:   0000000iiiii_aaaaa_101_ddddd_0010011", desc: "Logical right shift (zeroes are shifted into the upper bits)."  },
    InstructionInfo { name: "SRAI"  , mnemonic: "srai rd, rs1, imm" , encoding: "I-Type:   0100000iiiii_aaaaa_101_ddddd_0010011", desc: "Arithmatic right shift (the original sign bit is copied into the vacant upper bits)."  },
    
    InstructionInfo { name: "LUI"   , mnemonic: "lui rd, imm"       , encoding: "U-Type:     iiiiiiiiiiiiiiiiiiii_ddddd_0110111", desc: "Load Upper Immediate. Used to build 32-bit constants and use the U-type format. LUI places the U-immediate value in the top 20 bits of the descination register rd, filling in the lowest 12 bits with zeros."  },
    InstructionInfo { name: "AUPC"  , mnemonic: "auipc rd, imm"     , encoding: "U-Type:     iiiiiiiiiiiiiiiiiiii_ddddd_0010111", desc: "Add Upper Immediate ot PC. Used to build pc-relative addresses and uses the U-type format. AUIPC froms a 32-bit offset from the 10-bit U-immediate, filling the lowest 12 bits with zeroes, add this offset to the address of the AUIPC instruction, then places the result in register rd."  },
    
    InstructionInfo { name: "ADD"   , mnemonic: "add rd, rs1, rs2"  , encoding: "R-Type:  0000000_bbbbb_aaaaa_000_ddddd_0110011", desc: "Add performs the addition of rs1 and rs2. Overflows are ignored and the low XLEN bits of results are written to the desination rd."  },
    InstructionInfo { name: "SUB"   , mnemonic: "sub rd, rs1, rs2"  , encoding: "R-Type:  0100000_bbbbb_aaaaa_000_ddddd_0110011", desc: "Add performs the subtraction of rs1 and rs2. Overflows are ignored and the low XLEN bits of results are written to the desination rd."  },
    InstructionInfo { name: "SLL"   , mnemonic: "sll rd, rs1, rs2"  , encoding: "R-Type:  0000000_bbbbb_aaaaa_001_ddddd_0110011", desc: "SLL performs logical left shift on the value in register `rs1` by teh shift amount held in the lower 5 bits of register `rs2`."  },
    InstructionInfo { name: "SLT"   , mnemonic: "slt rd, rs1, rs2"  , encoding: "R-Type:  0000000_bbbbb_aaaaa_010_ddddd_0110011", desc: "SLT preforms signed compares respectively, writing `1` or `rd` if `rs1 < rs2`, `0` otherwise."  },
    InstructionInfo { name: "SLTU"  , mnemonic: "sltu rd, rs1, rs2" , encoding: "R-Type:  0000000_bbbbb_aaaaa_011_ddddd_0110011", desc: "SLTU preforms unsigned compares respectively, writing `1` or `rd` if `rs1 < rs2`, `0` otherwise. Note: `SLTU rd, x0, rs2` sets `rd` to `1` if `rs2` is not euqla to zero, otherwise sets `rd` to zero (assembler pseudo-instruction `SNEZ rd, rs`)." },
    InstructionInfo { name: "XOR"   , mnemonic: "xor rd, rs1, rs2"  , encoding: "R-Type:  0000000_bbbbb_aaaaa_100_ddddd_0110011", desc: "XOR performs a bitwise logical xor operation"  },
    InstructionInfo { name: "SRL"   , mnemonic: "srl rd, rs1, rs2"  , encoding: "R-Type:  0000000_bbbbb_aaaaa_101_ddddd_0110011", desc: "SRL performs logical right shift on the value in register `rs1` by the shift amount held in the lower 5 bits of register `rs2`."  },
    InstructionInfo { name: "SRA"   , mnemonic: "sra rd, rs1, rs2"  , encoding: "R-Type:  0100000_bbbbb_aaaaa_101_ddddd_0110011", desc: "SRA performs arithmetic right shift on the value in register `rs1` by the shift amount held in the lower 5 bits of register `rs2`."  },
    InstructionInfo { name: "OR"    , mnemonic: "or rd, rs1, rs2"   , encoding: "R-Type:  0000000_bbbbb_aaaaa_110_ddddd_0110011", desc: "OR performs a bitwise logical or operation"  },
    InstructionInfo { name: "AND"   , mnemonic: "and rd, rs1, rs2"  , encoding: "R-Type:  0000000_bbbbb_aaaaa_111_ddddd_0110011", desc: "AND performs a bitwise logical and operation"  },
    
    InstructionInfo { name: "JAL"   , mnemonic: "jal rd, imm"       , encoding: "J-Type:  i_iiiiiiiiii_i_iiiiiiii_ddddd_1101111", desc: "The jump and link JAL) instruciton uses the J-type format, where the J-immediate encodes a signed offset in multiples of 2 bytes. The offset is sign extended and added to the address of the jump instruction to form the jump target address. Jumps can therefore target a +-1MiB range. JAL stores the address of the instructi on following the jump (pc+4) into register `rd`. The standard software calling convention uses `x1` as the resturn addresss register and `x5` as teh laternate link register. Plain unconditional jmps (assembler pseudo-instrctuion J) are encoded as a JAL with `rd = x0`." },
    InstructionInfo { name: "JALR"  , mnemonic: "jalr rd, rs1, imm" , encoding: "I-type:   iiiiiiiiiiii_aaaaa_000_ddddd_1100111", desc: "The indrect jump instruction JALR (Jump And Link Register) uses the I-type encoding. The target address is obtained by adding the sign-extended 12-bit immeidate value to the register `rs1`, then setting the least significant bit of hte result to zero. The address of the instruction following the jump (pc + 4) is written to register `rd`. Reisger `x0` can be used as the destination if the result is not required." },
    
    InstructionInfo { name: "BEQ"   , mnemonic: "beq rs1, rs2, imm" , encoding: "B-Type: i_iiiiii_bbbbb_aaaaa_000_ddddd_1100011", desc: "Compare two registers and take a branch if registers `rs1` and `rs2` are equal." },
    InstructionInfo { name: "BNE"   , mnemonic: "bne rs1, rs2, imm" , encoding: "B-Type: i_iiiiii_bbbbb_aaaaa_001_ddddd_1100011", desc: "Compare two registers and take a branch if registers `rs1` and `rs2` are unequal." },
    InstructionInfo { name: "BLT"   , mnemonic: "blt rs1, rs2, imm" , encoding: "B-Type: i_iiiiii_bbbbb_aaaaa_100_ddddd_1100011", desc: "Compare two registers and take a branch if `rs1` is less than `rs2`, using signed comparison. Note: BGT can be synthesized by reversing the operand to BLT." },
    InstructionInfo { name: "BLTU"  , mnemonic: "bltu rs1, rs2, imm", encoding: "B-Type: i_iiiiii_bbbbb_aaaaa_101_ddddd_1100011", desc: "Compare two registers and take a branch if `rs1` is less than `rs2`, using unsigned comparison. Note: BGTU can be synthesized by reversing the operand to BLTU." },
    InstructionInfo { name: "BGE"   , mnemonic: "bge rs1, rs2, imm" , encoding: "B-Type: i_iiiiii_bbbbb_aaaaa_110_ddddd_1100011", desc: "Compare two registers and take a branch if `rs1` is greater than or equal to `rs2`, using signed comparison. Note: BLE can be synthesized by reversing the operand to BGE." },
    InstructionInfo { name: "BGEU"  , mnemonic: "bgeu rs1, rs2, imm", encoding: "B-Type: i_iiiiii_bbbbb_aaaaa_111_ddddd_1100011", desc: "Compare two registers and take a branch if `rs1` is greater than or equal to `rs2`, using unsigned comparison. Note: BLEU can be synthesized by reversing the operand to BGEU." },

    InstructionInfo { name: "LB"    , mnemonic: "lb rd, rs1, imm"   , encoding: "I-Type:   iiiiiiiiiiii_aaaaa_000_ddddd_0000011", desc: "Loads an 8-bit value from memory, then sign-extends to 32-bit before storing into `rd`. The effective address is obtained by adding register `rs1` to the sign-extended 12-bit offset." },
    InstructionInfo { name: "LH"    , mnemonic: "lh rd, rs1, imm"   , encoding: "I-Type:   iiiiiiiiiiii_aaaaa_001_ddddd_0000011", desc: "Loads a 16-bit value from memory, then sign-extends to 32-bit before storing into `rd`. The effective address is obtained by adding register `rs1` to the sign-extended 12-bit offset." },
    InstructionInfo { name: "LW"    , mnemonic: "lw rd, rs1, imm"   , encoding: "I-Type:   iiiiiiiiiiii_aaaaa_010_ddddd_0000011", desc: "Loads a 32-bit value from memory into `rd`. The effective address is obtained by adding register `rs1` to the sign-extended 12-bit offset." },
    InstructionInfo { name: "LBU"   , mnemonic: "lbu rd, rs1, imm"  , encoding: "I-Type:   iiiiiiiiiiii_aaaaa_100_ddddd_0000011", desc: "Loads an 8-bit value from memory, then zero-extends to 32-bits before storing into `rd`. The effective address is obtained by adding register `rs1` to the sign-extended 12-bit offset." },
    InstructionInfo { name: "LHU"   , mnemonic: "lhu rd, rs1, imm"  , encoding: "I-Type:   iiiiiiiiiiii_aaaaa_101_ddddd_0000011", desc: "Loads a 16-bit value from memory, then zero-extends to 32-bits before storing into `rd`. The effective address is obtained by adding register `rs1` to the sign-extended 12-bit offset." },

    InstructionInfo { name: "SB"    , mnemonic: "sb rs1, rs2, imm"  , encoding: "S-Type:   iiiiii_bbbbb_aaaaa_000_iiiii_0100011", desc: "Store an 8-bit value from the lower bits of register `rs2` to memory. The effective address is obtained by adding register `rs1` to the sign-extended 12-bit offset." },
    InstructionInfo { name: "SH"    , mnemonic: "sh rs1, rs2, imm"  , encoding: "S-Type:   iiiiii_bbbbb_aaaaa_000_iiiii_0100011", desc: "Store a 16-bit value from the lower bits of register `rs2` to memory. The effective address is obtained by adding register `rs1` to the sign-extended 12-bit offset." },
    InstructionInfo { name: "SW"    , mnemonic: "sw rs1, rs2, imm"  , encoding: "S-Type:   iiiiii_bbbbb_aaaaa_000_iiiii_0100011", desc: "Store a 32-bit value from register `rs2` to memory. The effective address is obtained by adding register `rs1` to the sign-extended 12-bit offset." },
    
    InstructionInfo { name: "FENCE" , mnemonic: "fence succ, pred"  , encoding: "I-Type:   ff_pppp_ssss_aaaaa_000_ddddd_0001111", desc: "Used to order I/O and memory accesses as view by other RISC-V harts an external devices or coprocessors. Any combination of input (I), device output (O), memory reads (R), memory writes (W) may be ordered with respect to any combination or the same. For more info: see chapter 2.7 of the RIS-V Unpriviledged ISA." },

    InstructionInfo { name: "ECALL" , mnemonic: "ecall"             , encoding: "I-Type:   000000000000_00000_000_00000_1110011", desc: "Used to make a service requrest to the execution environment. The EEI will define how paramters for the service request are passed, but usually these will be in defined locations in the integer register file" },
    InstructionInfo { name: "EBREAK", mnemonic: "ebreak"            , encoding: "I-Type:   000000000001_00000_000_00000_1110011", desc: "Used to return control to a debuffing environment" },

    //InstructionInfo { name: "", mnemonic: "", encoding: "", desc: "" },
];