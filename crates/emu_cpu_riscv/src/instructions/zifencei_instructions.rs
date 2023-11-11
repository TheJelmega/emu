use std::fmt;

use emu_cpu::InstructionInfo;
use emu_macros::EnumCount;
use emu_utils::*;

use crate::registers::RegisterFile;

#[derive(Clone, Copy, PartialEq, Eq, Debug, EnumCount)]
pub enum ZifenceiInstructions {
    FenceI,
}

impl ZifenceiInstructions {
    pub fn exec(&self, register_file: &mut RegisterFile, memory: &mut [u8]) {
        match *self {
            ZifenceiInstructions::FenceI => {} // Dummy, we don't have an i$,
        }
    }

    pub fn log(&self, f: &mut dyn fmt::Write) -> fmt::Result {
        write!(f, "ifencei")
    }
}

pub const ZIFENCEI_INSTUCTION_INFO: [InstructionInfo; ZifenceiInstructions::COUNT] = [
    InstructionInfo { name: "FENCE.I", mnemonic: "fence.i", encoding: "I-Type: 000000000000_00000_001_00000_0001111", desc: "Used to synchronized the instruction and data streams. RISC-V does not guarantee that stores to instruciton memory will be made visible to instruction fetches on a RISC-V hart until thta hard executes a FENCE.I instcruction." },
];