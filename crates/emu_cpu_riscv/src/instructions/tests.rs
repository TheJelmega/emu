use crate::registers::RegisterFile;

use super::*;

#[test]
fn test_rv32i_addi() {
    let mut register_file = RegisterFile::new(false);
    register_file.write_x_register(2, 21);

    let mut memory = [0];
    
    // sign: 0
    let instr = RV32IInstuction::ADDI { rd: 1, rs1: 2, imm: 42 };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 63);
    assert_eq!(register_file.read_pc(), 4);
    
    // sign: 1
    let imm = (-42i16) as u16;
    let imm = imm & (u16::MAX >> 4);
    let instr = RV32IInstuction::ADDI { rd: 1, rs1: 2, imm };
    instr.exec(&mut register_file, &mut memory);
    
    assert_eq!(register_file.read_x_register(1), (-21i64) as u64);
    assert_eq!(register_file.read_pc(), 8);
}

#[test]
fn test_rv32i_slti() {
    let mut register_file = RegisterFile::new(false);
    register_file.write_x_register(2, 21);

    let mut memory = [0];
    
    // less
    let instr = RV32IInstuction::SLTI { rd: 1, rs1: 2, imm: 42 };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 1);
    assert_eq!(register_file.read_pc(), 4);

    // equal: 0
    let instr = RV32IInstuction::SLTI { rd: 1, rs1: 2, imm: 21 };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0);
    assert_eq!(register_file.read_pc(), 8);

    // greater: 0
    let instr = RV32IInstuction::SLTI { rd: 1, rs1: 2, imm: 10 };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0);
    assert_eq!(register_file.read_pc(), 12);

    // Sign extended
    register_file.write_x_register(2, (-21i64) as u64);

    // less sign extended
    let imm = (-12i16) as u16;
    let imm = imm & (u16::MAX >> 4);
    let instr = RV32IInstuction::SLTI { rd: 1, rs1: 2, imm };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 1);
    assert_eq!(register_file.read_pc(), 16);

    // greater sign extended
    let imm = (-42i16) as u16;
    let imm = imm & (u16::MAX >> 4);
    let instr = RV32IInstuction::SLTI { rd: 1, rs1: 2, imm };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0);
    assert_eq!(register_file.read_pc(), 20);
}

#[test]
fn test_rv32i_sltiu() {
    let mut register_file = RegisterFile::new(false);
    register_file.write_x_register(2, 21);

    let mut memory = [0];
    
    // less
    let instr = RV32IInstuction::SLTIU { rd: 1, rs1: 2, imm: 42 };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 1);
    assert_eq!(register_file.read_pc(), 4);

    // equal: 0
    let instr = RV32IInstuction::SLTIU { rd: 1, rs1: 2, imm: 21 };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0);
    assert_eq!(register_file.read_pc(), 8);

    // greater: 0
    let instr = RV32IInstuction::SLTIU { rd: 1, rs1: 2, imm: 10 };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0);
    assert_eq!(register_file.read_pc(), 12);

    // Sign extended
    register_file.write_x_register(2, (-21i64) as u64);

    // less sign extended
    let imm = (-12i16) as u16;
    let imm = imm & (u16::MAX >> 4);
    let instr = RV32IInstuction::SLTIU { rd: 1, rs1: 2, imm };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 1);
    assert_eq!(register_file.read_pc(), 16);

    // greater sign extended
    let imm = (-42i16) as u16;
    let imm = imm & (u16::MAX >> 4);
    let instr = RV32IInstuction::SLTIU { rd: 1, rs1: 2, imm };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0);
    assert_eq!(register_file.read_pc(), 20);
}

#[test]
fn test_rv32i_xori() {
    let mut register_file = RegisterFile::new(false);
    register_file.write_x_register(2, 0x5555_5555);

    let mut memory = [0];
    
    // no sign extended
    let instr = RV32IInstuction::XORI { rd: 1, rs1: 2, imm: 0x75A };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0x5555_520F);
    assert_eq!(register_file.read_pc(), 4);
    
    // sign extended
    let instr = RV32IInstuction::XORI { rd: 1, rs1: 2, imm: 0xF5A };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0xFFFF_FFFF_AAAA_AA0F);
    assert_eq!(register_file.read_pc(), 8);
}

#[test]
fn test_rv32i_ori() {
    let mut register_file = RegisterFile::new(false);
    register_file.write_x_register(2, 0x5555_5555);

    let mut memory = [0];
    
    // no sign extended
    let instr = RV32IInstuction::ORI { rd: 1, rs1: 2, imm: 0x75A };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0x5555_575F);
    assert_eq!(register_file.read_pc(), 4);
    
    // sign extended
    let instr = RV32IInstuction::ORI { rd: 1, rs1: 2, imm: 0xF5A };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0xFFFF_FFFF_FFFF_FF5F);
    assert_eq!(register_file.read_pc(), 8);
}

#[test]
fn test_rv32i_andi() {
    let mut register_file = RegisterFile::new(false);
    register_file.write_x_register(2, 0x5555_5555);

    let mut memory = [0];
    
    // no sign extended
    let instr = RV32IInstuction::ANDI { rd: 1, rs1: 2, imm: 0x75A };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0x0000_0550);
    assert_eq!(register_file.read_pc(), 4);
    
    // sign extended
    let instr = RV32IInstuction::ANDI { rd: 1, rs1: 2, imm: 0xF5A };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0x5555_5550);
    assert_eq!(register_file.read_pc(), 8);
}

#[test]
fn test_rv32i_slli() {
    let mut register_file = RegisterFile::new(false);
    register_file.write_x_register(2, 85); // 01010101

    let mut memory = [0];

    let instr = RV32IInstuction::SRLI { rd: 1, rs1: 2, imm: 1 };
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_x_register(1), 42);
    assert_eq!(register_file.read_pc(), 4);

    let instr = RV32IInstuction::SRLI { rd: 1, rs1: 2, imm: 4 };
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_x_register(1), 5);
    assert_eq!(register_file.read_pc(), 8);
}

#[test]
fn test_rv32i_srli() {
    let mut register_file = RegisterFile::new(false);
    register_file.write_x_register(2, 170); // 10101010

    let mut memory = [0];

    let instr = RV32IInstuction::SRLI { rd: 1, rs1: 2, imm: 1 };
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_x_register(1), 85);
    assert_eq!(register_file.read_pc(), 4);

    let instr = RV32IInstuction::SRLI { rd: 1, rs1: 2, imm: 4 };
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_x_register(1), 10);
    assert_eq!(register_file.read_pc(), 8);
}

#[test]
fn test_rv32i_srai() {
    let mut register_file = RegisterFile::new(false);
    register_file.write_x_register(2, (-170i64) as u64); // 10101010

    let mut memory = [0];

    let instr = RV32IInstuction::SRAI { rd: 1, rs1: 2, imm: 1 };
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_x_register(1) as i32, -85); //11010101
    assert_eq!(register_file.read_pc(), 4);

    let instr = RV32IInstuction::SRAI { rd: 1, rs1: 2, imm: 4 };
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_x_register(1) as i32, -11);
    assert_eq!(register_file.read_pc(), 8);
}

#[test]
fn test_rv32i_lui() {
    let mut register_file = RegisterFile::new(false);
    register_file.write_x_register(2, (-170i64) as u64); // 10101010

    let mut memory = [0];

    let instr = RV32IInstuction::LUI { rd: 1, imm: 0x1_2345 };
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_x_register(1), 0x1234_5000);
    assert_eq!(register_file.read_pc(), 4);
}

#[test]
fn test_rv32i_auipc() {
    let mut register_file = RegisterFile::new(false);
    register_file.write_x_register(2, (-170i64) as u64); // 10101010

    let mut memory = [0];

    let instr = RV32IInstuction::AUIPC { rd: 1, imm: 0x1_2345 };
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_x_register(1), 0x1234_5000);
    assert_eq!(register_file.read_pc(), 4);

    register_file.write_pc(0x67890);
    let instr = RV32IInstuction::AUIPC { rd: 1, imm: 0x1_2345 };
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_x_register(1), 0x1234_5000 + 0x67890);
    assert_eq!(register_file.read_pc(), 0x67894);
}


#[test]
fn test_rv32i_add() {
    let mut register_file = RegisterFile::new(false);
    register_file.write_x_register(2, 21);

    let mut memory = [0];
    let instr = RV32IInstuction::ADD { rd: 1, rs1: 2, rs2: 3 };
    
    // sign: 0
    register_file.write_x_register(3, 42);
    instr.exec(&mut register_file, &mut memory);
    
    assert_eq!(register_file.read_x_register(1), 63);
    assert_eq!(register_file.read_pc(), 4);
    
    // sign: 1
    register_file.write_x_register(3, -42i64 as u64);
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), (-21i64) as u64);
    assert_eq!(register_file.read_pc(), 8);
}

#[test]
fn test_rv32i_slt() {
    let mut register_file = RegisterFile::new(false);
    register_file.write_x_register(2, 21);
    
    let mut memory = [0];

    let instr = RV32IInstuction::SLT { rd: 1, rs1: 2, rs2: 3 };
    
    // less
    register_file.write_x_register(3, 42);
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 1);
    assert_eq!(register_file.read_pc(), 4);

    // equal: 0
    register_file.write_x_register(3, 21);
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0);
    assert_eq!(register_file.read_pc(), 8);

    // greater: 0
    register_file.write_x_register(3, 10);
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0);

    // Sign extended
    register_file.write_x_register(2, (-21i64) as u64);
    assert_eq!(register_file.read_pc(), 12);

    // less sign extended
    let val = (-12i16) as u16;
    let val = val & (u16::MAX >> 4);
    register_file.write_x_register(3, val as u64);
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 1);
    assert_eq!(register_file.read_pc(), 16);

    // greater sign extended
    register_file.write_x_register(3, -42i64 as u64);
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0);
    assert_eq!(register_file.read_pc(), 20);
}

#[test]
fn test_rv32i_sltu() {
    let mut register_file = RegisterFile::new(false);
    register_file.write_x_register(2, 21);

    let mut memory = [0];
    
    let instr = RV32IInstuction::SLTU { rd: 1, rs1: 2, rs2: 3 };

    // less
    register_file.write_x_register(3, 42);
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 1);
    assert_eq!(register_file.read_pc(), 4);

    // equal: 0
    register_file.write_x_register(3, 21);
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0);
    assert_eq!(register_file.read_pc(), 8);

    // greater: 0
    register_file.write_x_register(3, 10);
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0);
    assert_eq!(register_file.read_pc(), 12);

    // Sign extended
    register_file.write_x_register(2, (-21i64) as u64);

    // less sign extended
    register_file.write_x_register(3, -12i64 as u64);
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 1);
    assert_eq!(register_file.read_pc(), 16);

    // greater sign extended
    register_file.write_x_register(3, -42i64 as u64);
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0);
    assert_eq!(register_file.read_pc(), 20);
}

#[test]
fn test_rv32i_xor() {
    let mut register_file = RegisterFile::new(false);
    register_file.write_x_register(2, 0x5555_5555);

    let mut memory = [0];
    
    let instr = RV32IInstuction::XOR { rd: 1, rs1: 2, rs2: 3 };

    // no sign extended
    register_file.write_x_register(3, 0x75A);
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0x5555_520F);
    assert_eq!(register_file.read_pc(), 4);
    
    // sign extended
    register_file.write_x_register(3, 0xFFFF_FF5A);
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0xAAAA_AA0F);
    assert_eq!(register_file.read_pc(), 8);
}

#[test]
fn test_rv32i_or() {
    let mut register_file = RegisterFile::new(false);
    register_file.write_x_register(2, 0x5555_5555);

    let mut memory = [0];
    
    let instr = RV32IInstuction::OR { rd: 1, rs1: 2, rs2: 3 };
    
    // no sign extended
    register_file.write_x_register(3, 0x75A);
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0x5555_575F);
    assert_eq!(register_file.read_pc(), 4);
    
    // sign extended
    register_file.write_x_register(3, 0xFFFF_FF5A);
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0xFFFF_FF5F);
    assert_eq!(register_file.read_pc(), 8);
}

#[test]
fn test_rv32i_and() {
    let mut register_file = RegisterFile::new(false);
    register_file.write_x_register(2, 0x5555_5555);

    let mut memory = [0];
    
    let instr = RV32IInstuction::AND { rd: 1, rs1: 2, rs2: 3 };
    
    // no sign extended
    register_file.write_x_register(3, 0x75A);
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0x0000_0550);
    assert_eq!(register_file.read_pc(), 4);
    
    // sign extended
    register_file.write_x_register(3, 0xFFFF_FF5A);
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0x5555_5550);
    assert_eq!(register_file.read_pc(), 8);
}

#[test]
fn test_rv32i_sll() {
    let mut register_file = RegisterFile::new(false);
    register_file.write_x_register(2, 85); // 01010101

    let mut memory = [0];

    let instr = RV32IInstuction::SRL { rd: 1, rs1: 2, rs2: 3 };
    
    register_file.write_x_register(3, 1);
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_x_register(1), 42);
    assert_eq!(register_file.read_pc(), 4);

    register_file.write_x_register(3, 4);
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_x_register(1), 5);
    assert_eq!(register_file.read_pc(), 8);
}

#[test]
fn test_rv32i_srl() {
    let mut register_file = RegisterFile::new(false);
    register_file.write_x_register(2, 170); // 10101010

    let mut memory = [0];

    let instr = RV32IInstuction::SRL { rd: 1, rs1: 2, rs2: 3 };
    
    register_file.write_x_register(3, 1);
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_x_register(1), 85);
    assert_eq!(register_file.read_pc(), 4);

    register_file.write_x_register(3, 4);
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_x_register(1), 10);
    assert_eq!(register_file.read_pc(), 8);
}

#[test]
fn test_rv32i_sra() {
    let mut register_file = RegisterFile::new(false);
    register_file.write_x_register(2, (-170i64) as u64); // 10101010

    let mut memory = [0];

    let instr = RV32IInstuction::SRA { rd: 1, rs1: 2, rs2: 3 };
    
    register_file.write_x_register(3, 1);
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_x_register(1) as i32, -85); //11010101
    assert_eq!(register_file.read_pc(), 4);

    register_file.write_x_register(3, 4);
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_x_register(1) as i32, -11);
    assert_eq!(register_file.read_pc(), 8);
}

#[test]
fn test_rv32i_jal() {
    let mut register_file = RegisterFile::new(false);
    register_file.write_pc(128);

    let mut memory = [0];

    // pos offset
    let instr = RV32IInstuction::JAL { rd: 1, imm: 64 };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 132);
    assert_eq!(register_file.read_pc(), 192);

    let instr = RV32IInstuction::JAL { rd: 1, imm: (-64i32 as u32) & 0x001F_FFFF };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 196);
    assert_eq!(register_file.read_pc(), 128);
}

#[test]
fn test_rv32i_jalr() {
    let mut register_file = RegisterFile::new(false);
    register_file.write_pc(128);

    let mut memory = [0];

    
    // pos offset
    let instr = RV32IInstuction::JALR { rd: 1, rs1: 2, imm: 32 };
    register_file.write_x_register(2, 32);
    instr.exec(&mut register_file, &mut memory);
    
    assert_eq!(register_file.read_x_register(1), 132);
    assert_eq!(register_file.read_pc(), 192);
    
    let instr = RV32IInstuction::JALR { rd: 1, rs1: 2, imm: (-32i16 as u16) & 0x0FFF };
    register_file.write_x_register(2, -32i64 as u64);
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 196);
    assert_eq!(register_file.read_pc(), 128);
}

#[test]
fn test_rv32i_beq() {
    let mut register_file = RegisterFile::new(false);

    let mut memory = [0];
    
    register_file.write_x_register(2, 42);

    let instr = RV32IInstuction::BEQ { rs1: 1, rs2: 2, imm: 64 };

    // equal
    register_file.write_x_register(1, 42);
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_pc(), 64);

    // less
    register_file.write_x_register(1, 21);
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_pc(), 68);

    // greater
    register_file.write_x_register(1, 48);
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_pc(), 72);

    // signed
    register_file.write_x_register(1, (-21i64) as u64);
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_pc(), 76);
}

#[test]
fn test_rv32i_bne() {
    let mut register_file = RegisterFile::new(false);

    let mut memory = [0];
    
    register_file.write_x_register(2, 42);

    let instr = RV32IInstuction::BNE { rs1: 1, rs2: 2, imm: 64 };

    // equal
    register_file.write_x_register(1, 42);
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_pc(), 4);

    // less
    register_file.write_x_register(1, 21);
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_pc(), 68);

    // greater
    register_file.write_x_register(1, 48);
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_pc(), 132);

    // signed
    register_file.write_x_register(1, (-21i64) as u64);
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_pc(), 196);
}

#[test]
fn test_rv32i_blt() {
    let mut register_file = RegisterFile::new(false);

    let mut memory = [0];
    
    register_file.write_x_register(2, 42);

    let instr = RV32IInstuction::BLT { rs1: 1, rs2: 2, imm: 64 };

    // equal
    register_file.write_x_register(1, 42);
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_pc(), 4);

    // less
    register_file.write_x_register(1, 21);
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_pc(), 68);

    // greater
    register_file.write_x_register(1, 48);
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_pc(), 72);

    // signed
    register_file.write_x_register(1, (-21i64) as u64);
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_pc(), 136);
}

#[test]
fn test_rv32i_bltu() {
    let mut register_file = RegisterFile::new(false);

    let mut memory = [0];
    
    register_file.write_x_register(2, 42);

    let instr = RV32IInstuction::BLTU { rs1: 1, rs2: 2, imm: 64 };

    // equal
    register_file.write_x_register(1, 42);
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_pc(), 4);

    // less
    register_file.write_x_register(1, 21);
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_pc(), 68);

    // greater
    register_file.write_x_register(1, 48);
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_pc(), 72);

    // signed
    register_file.write_x_register(1, (-21i64) as u64);
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_pc(), 76);
}

#[test]
fn test_rv32i_bge() {
    let mut register_file = RegisterFile::new(false);

    let mut memory = [0];
    
    register_file.write_x_register(2, 42);

    let instr = RV32IInstuction::BGE { rs1: 1, rs2: 2, imm: 64 };

    // equal
    register_file.write_x_register(1, 42);
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_pc(), 64);

    // less
    register_file.write_x_register(1, 21);
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_pc(), 68);

    // greater
    register_file.write_x_register(1, 48);
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_pc(), 132);

    // signed
    register_file.write_x_register(1, (-21i64) as u64);
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_pc(), 136);
}

#[test]
fn test_rv32i_bgtu() {
    let mut register_file = RegisterFile::new(false);

    let mut memory = [0];
    
    register_file.write_x_register(2, 42);

    let instr = RV32IInstuction::BGEU { rs1: 1, rs2: 2, imm: 64 };

    // equal
    register_file.write_x_register(1, 42);
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_pc(), 64);

    // less
    register_file.write_x_register(1, 21);
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_pc(), 68);

    // greater
    register_file.write_x_register(1, 48);
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_pc(), 132);

    // signed
    register_file.write_x_register(1, (-21i64) as u64);
    instr.exec(&mut register_file, &mut memory);
    assert_eq!(register_file.read_pc(), 196);
}

#[test]
fn test_rv32i_lb() {
    let mut register_file = RegisterFile::new(false);

    let mut memory = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];

    register_file.write_x_register(2, 8);

    // Aligned load
    let instr = RV32IInstuction::LB { rd: 1, rs1: 2, imm: 8 };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 16);
    assert_eq!(register_file.read_pc(), 4);

    // Unaligned load
    let instr = RV32IInstuction::LB { rd: 1, rs1: 2, imm: 3 };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 11);
    assert_eq!(register_file.read_pc(), 8);

    // Negative offset
    let instr = RV32IInstuction::LB { rd: 1, rs1: 2, imm: (-4i16) as u16 & 0x0FFF };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 4);
    assert_eq!(register_file.read_pc(), 12);
}

#[test]
fn test_rv32i_lh() {
    let mut register_file = RegisterFile::new(false);

    let mut memory = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];

    register_file.write_x_register(2, 8);

    // Aligned load
    let instr = RV32IInstuction::LH { rd: 1, rs1: 2, imm: 8 };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0x1110);
    assert_eq!(register_file.read_pc(), 4);

    // Unaligned load
    let instr = RV32IInstuction::LH { rd: 1, rs1: 2, imm: 3 };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0x0C0B);
    assert_eq!(register_file.read_pc(), 8);

    // Negative offset
    let instr = RV32IInstuction::LH { rd: 1, rs1: 2, imm: (-4i16) as u16 & 0x0FFF };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0x0504);
    assert_eq!(register_file.read_pc(), 12);
}

#[test]
fn test_rv32i_lw() {
    let mut register_file = RegisterFile::new(false);

    let mut memory = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];

    register_file.write_x_register(2, 8);

    // Aligned load
    let instr = RV32IInstuction::LW { rd: 1, rs1: 2, imm: 8 };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0x13121110);
    assert_eq!(register_file.read_pc(), 4);

    // Unaligned load
    let instr = RV32IInstuction::LW { rd: 1, rs1: 2, imm: 3 };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0x0E0D0C0B);
    assert_eq!(register_file.read_pc(), 8);

    // Negative offset
    let instr = RV32IInstuction::LW { rd: 1, rs1: 2, imm: (-4i16) as u16 & 0x0FFF };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0x07060504);
    assert_eq!(register_file.read_pc(), 12);
}


#[test]
fn test_rv32i_lbu() {
    let mut register_file = RegisterFile::new(false);

    let mut memory = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];

    register_file.write_x_register(2, 8);

    // Aligned load
    let instr = RV32IInstuction::LBU { rd: 1, rs1: 2, imm: 8 };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 16);
    assert_eq!(register_file.read_pc(), 4);

    // Unaligned load
    let instr = RV32IInstuction::LBU { rd: 1, rs1: 2, imm: 3 };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 11);
    assert_eq!(register_file.read_pc(), 8);

    // Negative offset
    let instr = RV32IInstuction::LBU { rd: 1, rs1: 2, imm: (-4i16) as u16 & 0x0FFF };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 4);
    assert_eq!(register_file.read_pc(), 12);
}

#[test]
fn test_rv32i_lhu() {
    let mut register_file = RegisterFile::new(false);

    let mut memory = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];

    register_file.write_x_register(2, 8);

    // Aligned load
    let instr = RV32IInstuction::LHU { rd: 1, rs1: 2, imm: 8 };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0x1110);
    assert_eq!(register_file.read_pc(), 4);

    // Unaligned load
    let instr = RV32IInstuction::LHU { rd: 1, rs1: 2, imm: 3 };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0x0C0B);
    assert_eq!(register_file.read_pc(), 8);

    // Negative offset
    let instr = RV32IInstuction::LHU { rd: 1, rs1: 2, imm: (-4i16) as u16 & 0x0FFF };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(register_file.read_x_register(1), 0x0504);
    assert_eq!(register_file.read_pc(), 12);
}

#[test]
fn test_rv32i_sb() {
    let mut register_file = RegisterFile::new(false);

    let mut memory = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];

    register_file.write_x_register(1, 8);
    register_file.write_x_register(2, 42);

    // Aligned store
    let instr = RV32IInstuction::SB { rs1: 1, rs2: 2, imm: 8 };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(memory[16], 42);
    assert_eq!(register_file.read_pc(), 4);

    // Unaligned store
    let instr = RV32IInstuction::SB { rs1: 1, rs2: 2, imm: 3 };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(memory[11], 42);
    assert_eq!(register_file.read_pc(), 8);

    // Negative offset
    let instr = RV32IInstuction::SB { rs1: 1, rs2: 2, imm: (-4i16) as u16 & 0x0FFF };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(memory[4], 42);
    assert_eq!(register_file.read_pc(), 12);
}

#[test]
fn test_rv32i_sh() {
    let mut register_file = RegisterFile::new(false);

    let mut memory = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];

    register_file.write_x_register(1, 8);
    register_file.write_x_register(2, 0x2B2A);

    // Aligned store
    let instr = RV32IInstuction::SH { rs1: 1, rs2: 2, imm: 8 };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(memory[16], 42);
    assert_eq!(memory[17], 43);
    assert_eq!(register_file.read_pc(), 4);

    // Unaligned store
    let instr = RV32IInstuction::SH { rs1: 1, rs2: 2, imm: 3 };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(memory[11], 42);
    assert_eq!(memory[12], 43);
    assert_eq!(register_file.read_pc(), 8);

    // Negative offset
    let instr = RV32IInstuction::SH { rs1: 1, rs2: 2, imm: (-4i16) as u16 & 0x0FFF };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(memory[4], 42);
    assert_eq!(memory[5], 43);
    assert_eq!(register_file.read_pc(), 12);
}

#[test]
fn test_rv32i_sw() {
    let mut register_file = RegisterFile::new(false);

    let mut memory = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31];

    register_file.write_x_register(1, 8);
    register_file.write_x_register(2, 0x2D2C2B2A);

    // Aligned store
    let instr = RV32IInstuction::SW { rs1: 1, rs2: 2, imm: 8 };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(memory[16], 42);
    assert_eq!(memory[17], 43);
    assert_eq!(memory[18], 44);
    assert_eq!(memory[19], 45);
    assert_eq!(register_file.read_pc(), 4);

    // Unaligned store
    let instr = RV32IInstuction::SW { rs1: 1, rs2: 2, imm: 3 };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(memory[11], 42);
    assert_eq!(memory[12], 43);
    assert_eq!(memory[13], 44);
    assert_eq!(memory[14], 45);
    assert_eq!(register_file.read_pc(), 8);

    // Negative offset
    let instr = RV32IInstuction::SW { rs1: 1, rs2: 2, imm: (-4i16) as u16 & 0x0FFF };
    instr.exec(&mut register_file, &mut memory);

    assert_eq!(memory[4], 42);
    assert_eq!(memory[5], 43);
    assert_eq!(memory[6], 44);
    assert_eq!(memory[7], 45);
    assert_eq!(register_file.read_pc(), 12);
}
