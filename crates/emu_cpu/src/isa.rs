use std::fmt;

use emu_macros::EnumCount;
use emu_utils::EnumCountT;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IsaStatus {
    /// ISA has been rattified
    Ratified,
    /// ISA is in the drafting stage
    Draft,
}

impl fmt::Display for IsaStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IsaStatus::Ratified => f.pad("ratified"),
            IsaStatus::Draft    => f.pad("draft"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, EnumCount)]
pub enum IsaSize {
    Size8,
    Size16,
    Size32,
    Size64,
    Size128,
}

impl fmt::Display for IsaSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IsaSize::Size8   => f.pad("8-bit"),
            IsaSize::Size16  => f.pad("16-bit"),
            IsaSize::Size32  => f.pad("32-bit"),
            IsaSize::Size64  => f.pad("64-bit"),
            IsaSize::Size128 => f.pad("128-bit"),
        }
    }
}



#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BaseIsaInfo {
    /// Name of the base ISA
    pub name:        &'static str,
    /// Description of the base ISA
    pub desc:        &'static str,
    /// Version of the base ISA
    pub version:     &'static str,
    /// Status of the base ISA
    pub status:      IsaStatus,
    /// Size of the isa
    pub size:        IsaSize,
    /// Instructions
    pub instructions: Option<&'static [InstructionInfo]>,
}

impl fmt::Display for BaseIsaInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:8} ({:7}) | {:64} | {:3} ({:8}) | {} instructions",
            self.name,
            self.size,
            self.desc,
            self.version,
            self.status,
            self.instructions.map_or(0, |arr| arr.len()),
        )?;

        if let Some(instructions) = self.instructions {
            for instr in instructions {
                write!(f, "\n    {}", instr)?;
            }
        }
        Ok(())
    }
}


pub struct ExtensionIsaInfo {
    /// Name of the extension ISA
    pub name:    &'static str,
    /// Description of the extension ISA
    pub desc:    &'static str,
    /// Version of the extension ISA
    pub version: &'static str,
    /// Status of the extension ISA
    pub status:      IsaStatus,
    /// Instruction count per ISA size
    pub instructions: [Option<&'static [InstructionInfo]>; IsaSize::COUNT],
}

impl ExtensionIsaInfo {
    /// Get the instruction count for a given ISA size
    pub fn instruction_count(&self, isa: IsaSize) -> u32 {
        self.instructions[isa as usize].map_or(0, |instrs| instrs.len() as u32)
    }
}

impl fmt::Display for ExtensionIsaInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:18} | {:64} | {:6} ({:8}) | instructions: 8-bit: {:3}, 16-bit: {:3}, 32-bit: {:3}, 64-bit: {:3}, 128-bit: {:3}",
            self.name,
            self.desc,
            self.version,
            self.status,
            self.instructions[0].map_or(0, |instrs| instrs.len()),
            self.instructions[1].map_or(0, |instrs| instrs.len()),
            self.instructions[2].map_or(0, |instrs| instrs.len()),
            self.instructions[3].map_or(0, |instrs| instrs.len()),
            self.instructions[4].map_or(0, |instrs| instrs.len()),
        )?;

        for (idx, instrs) in self.instructions.iter().enumerate() {
            if let Some(instructions) = instrs {
                writeln!(f, "\n    {}-bit", 1 << (idx + 3))?;
                for instr in *instructions {
                    write!(f, "\n        {}", instr)?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct InstructionInfo {
    /// Instruction name
    pub name: &'static str,
    /// Instruction mnemonic
    pub mnemonic: &'static str,
    /// Instrucition encoding
    pub encoding: &'static str,
    /// Instuction description
    pub desc: &'static str,
}

impl fmt::Display for InstructionInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:8} | {:24} | {:64} | {}", self.name, self.mnemonic, self.encoding, self.desc)
    }
}