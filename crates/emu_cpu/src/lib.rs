
mod isa;
pub use isa::*;


pub struct EmulationSettings {
    /// Print the each instruction being execute
    pub print_instructions: bool,
}

pub trait CpuEmulator {
    /// Set the emulator settings
    fn set_settings(&mut self, settings: EmulationSettings);

    /// Set he base ISA, returns `true` if the given ISA was valid and set
    fn set_base_isa(&mut self, isa: &str) -> bool;

    /// Get all base ISAs supported by the CPU emulator
    fn get_base_isas(&self) -> &[BaseIsaInfo];

    /// Get all extension ISAs supporte by the CPU emulator
    fn get_ext_isas(&self) -> &[ExtensionIsaInfo];

    /// format the register file out to given formatter
    fn format_register_file(&self, f: &mut dyn std::fmt::Write) -> std::fmt::Result;

    /// Set the machine code to use
    fn set_code(&mut self, code: Vec<u8>);

    fn set_instruction_pointer(&mut self, pointer: usize);

    /// Tick a single instuction
    fn tick(&mut self);

    /// Execute the current machine code.
    /// 
    /// `num_instructions` allows the user to set the amount of instructions to execute, `None` means no limit
    fn execute(&mut self, num_instructions: Option<u32>);
}