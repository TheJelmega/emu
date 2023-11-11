use std::fmt::Write;

use emu_cpu::{ CpuEmulator, BaseIsaInfo, ExtensionIsaInfo, EmulationSettings };
use isa::{BASE_ISA_INFO, BaseIsa, EXT_ISA_INFO, ExtensionIsa};
use registers::RegisterFile;

use crate::instructions::InstructionEncoding32;



mod isa;
mod registers;
mod instructions;



// TODO:
// Setting to allow misaligned load/stores, i.e. 32-bit load/store not aligned to 4-bytes, and 16-bit load/store not aligned to 2-bytes
pub struct RiscvEmulator {
    pub settings: EmulationSettings,
    pub base_isa: BaseIsa,
    pub extensions: ExtensionIsa,

    pub register_file: RegisterFile,

    pub machine_code: Vec<u8>,
    pub memory: Vec<u8>,
}

impl RiscvEmulator {
    pub fn new(settings: EmulationSettings) -> Self {
        RiscvEmulator { 
            settings,
            base_isa: BaseIsa::RVWMO,
            extensions: ExtensionIsa::None,
            register_file: RegisterFile::new(false),
            machine_code: Vec::new(),
            memory: Vec::new(),
        }
    }
}

impl CpuEmulator for RiscvEmulator {

    fn set_settings(&mut self, settings: EmulationSettings) {
        self.settings = settings;
    }

    fn set_base_isa(&mut self, isa: &str) -> bool {
        match isa {
            "RVWMO"  => { self.base_isa = BaseIsa::RVWMO;  self.register_file.set_32_bit(false); true },
            "RV32I"  => { self.base_isa = BaseIsa::RV32I;  self.register_file.set_32_bit(true ); true },
            "RV32E"  => { self.base_isa = BaseIsa::RV32E;  self.register_file.set_32_bit(true ); true },
            "RV64I"  => { self.base_isa = BaseIsa::RV64I;  self.register_file.set_32_bit(false); true },
            "RV64E"  => { self.base_isa = BaseIsa::RV64E;  self.register_file.set_32_bit(false); true },
            "RV128I" => { self.base_isa = BaseIsa::RV128I; self.register_file.set_32_bit(false); true },
            _ => false
        }
    }


    fn get_base_isas(&self) -> &[BaseIsaInfo] {
        &BASE_ISA_INFO
    }

    fn get_ext_isas(&self) -> &[ExtensionIsaInfo] {
        &EXT_ISA_INFO
    }

    fn format_register_file(&self, f: &mut dyn std::fmt::Write) -> std::fmt::Result {
        write!(f, "{}", self.register_file)
    }

    fn set_code(&mut self, code: Vec<u8>) {
        self.machine_code = code;
    }

    fn set_instruction_pointer(&mut self, pointer: usize) {
        assert!(pointer & 1 == 0);
        self.register_file.write_pc(pointer as u64);
    }

    fn tick(&mut self) {
        let pc = self.register_file.read_pc() as usize;
        assert!(pc + 3 < self.machine_code.len());

        let raw = unsafe { *(self.machine_code.as_ptr().add(pc) as *const u32) };
        let encoded = InstructionEncoding32(raw);
        let instr = encoded.decode().unwrap();

        if self.settings.print_instructions {
            let mut buf = String::new();
            instr.log(&mut buf);
            println!("{}", &buf);
        }

        instr.exec(&mut self.register_file, &mut self.memory);
    }

    fn execute(&mut self, num_instructions: Option<u32>) {
        match num_instructions {
            Some(count) => {
                for _ in 0..count {
                    self.tick();
                }
            },
            None => loop {
                self.tick();
            },
        }
    }
}