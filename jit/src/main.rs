use thiserror::Error;
#[derive(Debug,Copy, Clone,Eq, PartialEq)]
pub enum JitEndian {
    Little,
    Big
}
#[derive(Error,Debug)]
pub enum JitError {
    #[error("Invalid argument to op")]
    InvalidArgument,
    #[error("Invalid argument to op (internal)")]
    InvalidArgumentInternal,
    #[error("reached a place we shouldn't")]
    InvalidExecution,

}
#[derive(Debug,Copy, Clone,Eq, PartialEq)]
// all these are 64 bit
pub enum DataDescType {
    Imm,
    TempReg, // temporaries
    PermReg, // these are emulated registers (ex: riscv, r0, r1, r2....)
}
#[derive(Debug,Copy, Clone,Eq, PartialEq)]
pub struct DataDesc {
    pub num: u64,
    pub dttype: DataDescType
}
impl DataDesc {
    pub fn new_imm(imm: u64) -> DataDesc {
        DataDesc {
            num: imm,
            dttype: DataDescType::Imm
        }
    }
    pub fn new_tempreg(num: u64) -> DataDesc {
        DataDesc {
            num,
            dttype: DataDescType::TempReg
        }
    }
    pub fn new_permreg(num: u64) -> DataDesc {
        DataDesc {
            num,
            dttype: DataDescType::PermReg
        }
    }
    pub fn is_writeable(&self) -> bool {
        if self.dttype == DataDescType::Imm {
            false
        } else {
            true
        }
    }
}
#[derive(Debug, Clone,Eq, PartialEq)]
pub struct ExtVarDesc {
    pub name: String,
}
impl ExtVarDesc {
    pub fn create_new(str: String) -> ExtVarDesc {
        ExtVarDesc {
            name: str
        }
    }
}
pub trait JitBackend {
    fn new_block(&mut self, addr: u64) -> Result<(), JitError>;
    fn end_block(&mut self) -> Result<(), JitError>;
    fn drop_specific_block_page(&mut self, addr: u64, pagesize: u64) -> Result<(), JitError>;
    fn exec_block(&mut self, addr: u64) -> Result<(), JitError>;
    fn init_guest_registers(&mut self, regs: Vec<ExtVarDesc>) -> Result<(), JitError>;
    fn get_guest_reg_desc(&mut self, idx: u64) -> Result<DataDesc, JitError>;
    fn get_guest_reg(&mut self, idx: u64) -> Result<u64, JitError>;
}
pub trait JitOps: JitBackend {
    fn mov(&mut self, dst: DataDesc, src: DataDesc);
    fn add(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn sub(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn extsw(&mut self, dst: DataDesc, op1: DataDesc);
    fn extuw(&mut self, dst: DataDesc, op1: DataDesc);
    fn extsh(&mut self, dst: DataDesc, op1: DataDesc);
    fn extuh(&mut self, dst: DataDesc, op1: DataDesc);
    fn extsb(&mut self, dst: DataDesc, op1: DataDesc);
    fn extub(&mut self, dst: DataDesc, op1: DataDesc);
    fn ceq(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn cne(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn cslel(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn cslew(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn csltl(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn csltw(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn csgel(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn csgew(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn csgtl(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn csgtw(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn culel(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn culew(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn cultl(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn cultw(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn cugel(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn cugew(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn cugtl(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn cugtw(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn udiv64(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn udiv32(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn sdiv64(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn sdiv32(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn mul64l(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn umul64h(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn smul64h(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn mul32l(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn umul32h(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn smul32h(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn neg32(&mut self, dst: DataDesc, op1: DataDesc);
    fn neg64(&mut self, dst: DataDesc, op1: DataDesc);
    fn urem32(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn srem64(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn srem32(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn urem64(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn or(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn and(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn xor(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn sar(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn shr(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    fn shl(&mut self, dst: DataDesc, op1: DataDesc, op2: DataDesc);
    // may do something, may not depening on impl
    fn free_temp(&mut self, var: DataDesc);
    fn create_temp(&mut self) -> Result<DataDesc, JitError>;

}
