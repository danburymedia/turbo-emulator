
#[derive(Debug,Copy, Clone,Eq, PartialEq)]
pub enum Xlen {
    // registers have variable lengths
    X32 = 1,
    X64 = 2,
}
pub const RISCV_RETURNADDR_REG: usize = 1;
pub const RISCV_STACKPOINTER_REG: usize = 2;
pub const RISCV_RETURNVALUE_REG: usize = 10;
pub const DRAM_BASE: u64 = 0x8000_0000;

pub fn xlen2bits(xl: Xlen) -> u64 {
    match xl {
        Xlen::X32 => 32,
        Xlen::X64 => 64
    }
}
pub fn xlen2misa(xl: Xlen) -> u64 {
    match xl {
        Xlen::X32 => 1,
        Xlen::X64 => 2
    }
}
#[derive(Debug,Copy, Clone,Eq, PartialEq)]
pub enum Exception {
    InstructionAddressMisaligned,
    InstructionAccessFault,
    IllegalInstruction,
    Breakpoint,
    LoadAddressMisaligned,
    LoadAccessFault,
    StoreAddressMisaligned,
    StoreAccessFault,
    EnvironmentCallFromUMode,
    EnvironmentCallFromSMode,
    EnvironmentCallFromMMode,
    InstructionPageFault,
    LoadPageFault,
    StorePageFault,
    UserSoftwareInterrupt,
    SupervisorSoftwareInterrupt,
    MachineSoftwareInterrupt,
    UserTimerInterrupt,
    SupervisorTimerInterrupt,
    MachineTimerInterrupt,
    UserExternalInterrupt,
    SupervisorExternalInterrupt,
    MachineExternalInterrupt
}
#[derive(Debug,Copy, Clone,Eq, PartialEq)]
pub struct Trap {
    pub ttype: Exception,
    pub val: u64
}
#[derive(Debug,Copy, Clone,Eq, PartialEq)]
pub enum RiscvMemError {
    /// failed for some reason at this addr (we don't necessarily know why)
    GenError(u64),
    /// error while page walking
    PageError(u64),

}
pub const RISCV_PAGE_SIZE: usize = 4096;
#[repr(u64)]
#[derive(Debug,Copy, Clone,Eq, PartialEq)]
pub enum Priv {
    UserApp = 0,
    Supervisor = 1,
    Reserved = 2, // maybe good for something?
    Machine = 3
}

pub fn get_privilege_encoding(mode: Priv) -> u64 {
    match mode {
        Priv::UserApp => 0,
        Priv::Supervisor => 1,
        Priv::Reserved => panic!(),
        Priv::Machine => 3
    }
}
pub fn get_privilege_mode(encoding: u64) -> Priv {
    match encoding {
        0 => Priv::UserApp,
        1 => Priv::Supervisor,
        3 => Priv::Machine,
        _ => panic!("Unknown privilege uncoding")
    }
}
pub fn get_trap_cause(trap: Trap, xlen: Xlen) -> u64 {
    let interrupt_bit = match xlen {
        Xlen::X32 => 0x80000000 as u64,
        Xlen::X64 => 0x8000000000000000 as u64,
    };
    match trap.ttype {
        Exception::InstructionAddressMisaligned => 0,
        Exception::InstructionAccessFault => 1,
        Exception::IllegalInstruction => 2,
        Exception::Breakpoint => 3,
        Exception::LoadAddressMisaligned => 4,
        Exception::LoadAccessFault => 5,
        Exception::StoreAddressMisaligned => 6,
        Exception::StoreAccessFault => 7,
        Exception::EnvironmentCallFromUMode => 8,
        Exception::EnvironmentCallFromSMode => 9,
        Exception::EnvironmentCallFromMMode => 11,
        Exception::InstructionPageFault => 12,
        Exception::LoadPageFault => 13,
        Exception::StorePageFault => 15,
        Exception::UserSoftwareInterrupt => interrupt_bit,
        Exception::SupervisorSoftwareInterrupt => interrupt_bit + 1,
        Exception::MachineSoftwareInterrupt => interrupt_bit + 3,
        Exception::UserTimerInterrupt => interrupt_bit + 4,
        Exception::SupervisorTimerInterrupt => interrupt_bit + 5,
        Exception::MachineTimerInterrupt => interrupt_bit + 7,
        Exception::UserExternalInterrupt => interrupt_bit + 8,
        Exception::SupervisorExternalInterrupt => interrupt_bit + 9,
        Exception::MachineExternalInterrupt => interrupt_bit + 11
    }
}
#[derive (Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RiscvArgs {
    pub rd: u32,
    pub rl: u32,
    pub aq: u32,
    pub csr: u32,
    pub pred: u32,
    pub succ: u32,
    pub zimm: u32,
    pub rm: u32,
    pub rs1: u32,
    pub rs2: u32,
    pub rs3: u32,
    pub shamt: u32,
    pub imm: u32, // can also be zimm, never use imm and zimm at same time
    pub nf: u32,
    pub vm: u32,
   // pub zimm: u32

}

pub fn rvc_reg(reg: u32) -> u32 {
    reg + 8
}
pub fn comp_shift(imm: u32) -> u32 {
    if imm == 0 {
        64
    } else {
        imm
    }
}
