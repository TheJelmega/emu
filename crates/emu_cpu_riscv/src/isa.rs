use emu_cpu::{BaseIsaInfo, IsaStatus, IsaSize, ExtensionIsaInfo};
use emu_macros::{EnumCount, flags};
use emu_utils::EnumCountT;

use crate::instructions::{RV32I_INSTUCTION_INFO, ZIFENCEI_INSTUCTION_INFO};


#[derive(Clone, Copy, PartialEq, Eq, Debug, EnumCount)]
pub enum BaseIsa {
    RVWMO,
    RV32I,
    RV32E,
    RV64I,
    RV64E,
    RV128I,
}

#[derive(EnumCount)]
#[flags]
pub enum ExtensionIsa {
    M,
    A,
    F,
    D,
    Zicsr,
    Zifencei,
    G = M | A | F | D | Zicsr | Zifencei,
    Q,
    L,
    C,
    B,
    J,
    T,
    P,
    V,
    Zk,
    H,
    S,
    Zam,
    Zihintpause,
    Zihintntl,
    Zfa,
    Zfh,
    Zfhmin,
    Zfinx,
    Zdinx,
    Zhinx,
    Zhinxmin,
    Zmmul,
    Zlso
}


pub const BASE_ISA_INFO: [BaseIsaInfo; BaseIsa::COUNT] = [
    BaseIsaInfo { name: "RVWMO" , desc: "Weak Memory Ordering"                   , version: "2.0", status: IsaStatus::Ratified, size: IsaSize::Size32 , instructions: None /*instr_count: 0 */ },
    BaseIsaInfo { name: "RV32I" , desc: "Base Integer Instruction Set"           , version: "2.1", status: IsaStatus::Ratified, size: IsaSize::Size32 , instructions: Some(&RV32I_INSTUCTION_INFO) /*instr_count: 40*/ },
    BaseIsaInfo { name: "RV32E" , desc: "Base Integer Instruction Set (embedded)", version: "2.0", status: IsaStatus::Ratified, size: IsaSize::Size32 , instructions: None /*instr_count: 40*/ },
    BaseIsaInfo { name: "RV64I" , desc: "Base Integer Instruction Set"           , version: "2.1", status: IsaStatus::Ratified, size: IsaSize::Size64 , instructions: None /*instr_count: 55*/ },
    BaseIsaInfo { name: "RV64E" , desc: "Base Integer Instruction Set (embedded)", version: "2.0", status: IsaStatus::Ratified, size: IsaSize::Size64 , instructions: None /*instr_count: 0 */ },
    BaseIsaInfo { name: "RV128I", desc: "Base Integer Instruction Set"           , version: "1.6", status: IsaStatus::Draft   , size: IsaSize::Size128, instructions: None /*instr_count: 15*/ },
];

pub const EXT_ISA_INFO: [ExtensionIsaInfo; ExtensionIsa::COUNT] = [
    ExtensionIsaInfo { name: "M"          , desc: "Standard extension for integer multiplication and division", version: "2.0"   , status: IsaStatus::Ratified, instructions: [None, None, None, None, None,] },
    ExtensionIsaInfo { name: "A"          , desc: "Standard extension for atomic instructions"                , version: "2.1"   , status: IsaStatus::Ratified, instructions: [None, None, None, None, None,] },
    ExtensionIsaInfo { name: "F"          , desc: "Standard extension for single-precision floating-point"    , version: "2.2"   , status: IsaStatus::Ratified, instructions: [None, None, None, None, None,] },
    ExtensionIsaInfo { name: "D"          , desc: "Standard extension for double-precision floating-point"    , version: "2.2"   , status: IsaStatus::Ratified, instructions: [None, None, None, None, None,] },
    ExtensionIsaInfo { name: "Zicsr"      , desc: "Control and Status Register (CSR) instructions"            , version: "2.0"   , status: IsaStatus::Ratified, instructions: [None, None, None, None, None,] },
    ExtensionIsaInfo { name: "Zifencei"   , desc: "Instuction-fetch fence"                                    , version: "2.0"   , status: IsaStatus::Ratified, instructions: [None, None, Some(&ZIFENCEI_INSTUCTION_INFO), Some(&ZIFENCEI_INSTUCTION_INFO), None,] },
    ExtensionIsaInfo { name: "G"          , desc: "Shorthand for the IMAFD_Zicrt_Zifencei base and extensions", version: "n/a"   , status: IsaStatus::Ratified, instructions: [None, None, None, None, None,] },
    ExtensionIsaInfo { name: "Q"          , desc: "Standard extension for quad-precision floating-point"      , version: "2.2"   , status: IsaStatus::Ratified, instructions: [None, None, None, None, None,] },
    ExtensionIsaInfo { name: "L"          , desc: "Standard extension for decimal floating-point"             , version: "0.0"   , status: IsaStatus::Draft   , instructions: [None, None, None, None, None,] },
    ExtensionIsaInfo { name: "C"          , desc: "Standard extension for compressed instructions"            , version: "2.0"   , status: IsaStatus::Ratified, instructions: [None, None, None, None, None,] },
    ExtensionIsaInfo { name: "B"          , desc: "Standard extension for bit manipulation"                   , version: "1.0"   , status: IsaStatus::Ratified, instructions: [None, None, None, None, None,] },
    ExtensionIsaInfo { name: "J"          , desc: "Standard extension for dynamically translated languages"   , version: "0.0"   , status: IsaStatus::Draft   , instructions: [None, None, None, None, None,] },
    ExtensionIsaInfo { name: "T"          , desc: "Standard extension for transactional memory"               , version: "0.0"   , status: IsaStatus::Draft   , instructions: [None, None, None, None, None,] },
    ExtensionIsaInfo { name: "P"          , desc: "Standard extension for packed SIMD instructions"           , version: "0.9.10", status: IsaStatus::Draft   , instructions: [None, None, None, None, None,] },
    ExtensionIsaInfo { name: "V"          , desc: "Standard extension for vector instructions"                , version: "1.0"   , status: IsaStatus::Ratified, instructions: [None, None, None, None, None,] },
    ExtensionIsaInfo { name: "Zk"         , desc: "Standard extension for scalar cryptography"                , version: "1.0.1" , status: IsaStatus::Ratified, instructions: [None, None, None, None, None,] },
    ExtensionIsaInfo { name: "H"          , desc: "Standard extension for hypervisor"                         , version: "1.0"   , status: IsaStatus::Ratified, instructions: [None, None, None, None, None,] },
    ExtensionIsaInfo { name: "S"          , desc: "Standard extension for supervisor-level instructions"      , version: "1.12"  , status: IsaStatus::Ratified, instructions: [None, None, None, None, None,] },
    ExtensionIsaInfo { name: "Zam"        , desc: "Misaligned atomics"                                        , version: "0.1"   , status: IsaStatus::Draft   , instructions: [None, None, None, None, None,] },
    ExtensionIsaInfo { name: "Zihintpause", desc: "Pause hint"                                                , version: "2.0"   , status: IsaStatus::Ratified, instructions: [None, None, None, None, None,] },
    ExtensionIsaInfo { name: "Zihintntl"  , desc: "Non-temporal locality hints"                               , version: "0.2"   , status: IsaStatus::Draft   , instructions: [None, None, None, None, None,] },
    ExtensionIsaInfo { name: "Zfa"        , desc: "Aditional floating-point instruction"                      , version: "0.1"   , status: IsaStatus::Draft   , instructions: [None, None, None, None, None,] },
    ExtensionIsaInfo { name: "Zfh"        , desc: "Half-precision floating-point"                             , version: "1.0"   , status: IsaStatus::Ratified, instructions: [None, None, None, None, None,] },
    ExtensionIsaInfo { name: "Zfhmin"     , desc: "Minimal half-precision floating-point"                     , version: "1.0"   , status: IsaStatus::Ratified, instructions: [None, None, None, None, None,] },
    ExtensionIsaInfo { name: "Zfinx"      , desc: "Single-precision floating-point in integer registers"      , version: "1.0"   , status: IsaStatus::Ratified, instructions: [None, None, None, None, None,] },
    ExtensionIsaInfo { name: "Zdinx"      , desc: "Double-precision floating-point in integer registers"      , version: "1.0"   , status: IsaStatus::Ratified, instructions: [None, None, None, None, None,] },
    ExtensionIsaInfo { name: "Zhinx"      , desc: "Half-precision floating-point in integer registers"        , version: "1.0"   , status: IsaStatus::Ratified, instructions: [None, None, None, None, None,] },
    ExtensionIsaInfo { name: "Zhinxmin"   , desc: "Minimum half-precision floating-point in integer registers", version: "1.0"   , status: IsaStatus::Ratified, instructions: [None, None, None, None, None,] },
    ExtensionIsaInfo { name: "Zmmul"      , desc: "Multiplication subset of the M extension"                  , version: "1.0"   , status: IsaStatus::Ratified, instructions: [None, None, None, None, None,] },
    ExtensionIsaInfo { name: "Zlso"       , desc: "Total store ordering"                                      , version: "1.0"   , status: IsaStatus::Ratified, instructions: [None, None, None, None, None,] },
];