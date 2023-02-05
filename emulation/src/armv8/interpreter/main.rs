use base::debug;
use libc::sysinfo;
use simple_soft_float::RoundingMode;
use crate::armv8::common::ArmExt;
use crate::armv8::interpreter::floating::fpcr_2_fpsr;
use crate::armv8::interpreter::mem::{MemAccessStr, MemData};
use crate::common::arm_fp_defs::{FPSR, Flags};
use crate::armv8::interpreter::vect_helper::VectorReg;
use crate::armv8::ume::defs::{arm64_translate_syscall, write_arm64_stat};
use crate::common::memory::{flat_mem, MemEndian};
use crate::elf::UserModeRuntime;
use crate::linux_usermode::defs::{GenericStat, write_sysinfo_generic64};
use crate::linux_usermode::main::{dispatch, SyscallIn, UsermodeCpu};
use crate::linux_usermode::signals::{GenericSigactionArg, GenericStackt, SigEntry, SigInfo, Sigmask};

pub struct Arm64Cpu {
    reg: [u64; 32],
    pub tpidr: [u64; 4],
    pub stack_reg: u64,
    pub pc: u64,
    pub flag_status: Flags,
    pub vreg: [VectorReg; 32],
    pub stop_exec: bool,
    pub want_pc: Option<u64>,
    pub memory_access: flat_mem,
    pub is_usermode: bool,
    pub user_struct: UserModeRuntime,
    pub want_syscall: bool,
    pub fpcr: u32,
    pub fpsr: u32,
    pub mdata: MemData,


}
impl Arm64Cpu {
    // forgetset remember that 31 is zero
    pub fn a64_illegal_instruction(&mut self) {
        panic!("Internal error")
    }
    pub fn is_feat_avail(&mut self, ext: ArmExt) -> bool {
        // true if available, false if not
        false
    }
    pub fn handle_syscall(&mut self) {
        let syscallnum = self.get_reg(8, false) as u32;
        let systype = if let Some(s) = arm64_translate_syscall(syscallnum) {
            debug!("Going to execute syscall {:?} (number {:})", s, syscallnum);
            s
        } else {
            debug!("Failed to execute syscall number {:}", syscallnum);
            panic!();
        };
        let arg1 = self.get_reg(0, false) as u64;
        let arg2 = self.get_reg(1, false) as u64;
        let arg3 = self.get_reg(2, false) as u64;
        let arg4 = self.get_reg(3, false) as u64;
        let arg5 = self.get_reg(4, false) as u64;
        let arg6 = self.get_reg(5, false) as u64;
        let sysin: SyscallIn = SyscallIn {
            syscall: systype,
            args: [arg1, arg2, arg3, arg4, arg5, arg6, 0]
        };
        let out = dispatch(self, sysin);
        self.set_reg(0, out.ret1, false);
        if let Some(xx) = out.ret2 {
            self.set_reg(1, xx, false);
        }
    }
    pub fn get_pc(&mut self) -> u64 {
        self.pc
    }
    pub fn set_n_flag(&mut self, newval: bool) {
        self.flag_status.n = newval;
    }
    pub fn set_z_flag(&mut self, newval: bool) {
        self.flag_status.z = newval;
    }
    pub fn set_c_flag(&mut self, newval: bool) {
        self.flag_status.c = newval;
    }
    pub fn set_v_flag(&mut self, newval: bool) {
        self.flag_status.v = newval;
    }
    pub fn set_flags(&mut self, flags: Flags) {
        self.flag_status = flags;
    }
    pub fn get_n_flag(&mut self) -> bool {
        self.flag_status.n
    }
    pub fn get_z_flag(&mut self) -> bool  {
        self.flag_status.z
    }
    pub fn get_c_flag(&mut self) -> bool {
        self.flag_status.c
    }
    pub fn get_v_flag(&mut self) -> bool {
        self.flag_status.v
    }
    pub fn get_el_level(&mut self) -> u8 {
        0 // for now
    }

    pub fn get_fpcr_dn_flag(&mut self) -> bool {

        todo!();

    }
    pub fn accumlate_fpsr_errors(&mut self, err: FPSR) {
        // difference whem fpcr.1 is enabe
        //todo!();
        fpcr_2_fpsr(self, err.ioc, 0);
        fpcr_2_fpsr(self, err.dzc, 1);
        fpcr_2_fpsr(self, err.ofc, 2);
        fpcr_2_fpsr(self, err.ufc, 3);
        fpcr_2_fpsr(self, err.ixc, 4);
        fpcr_2_fpsr(self, err.idc, 7);
    }
    pub fn get_fpscr_rounding_mode(&mut self) -> RoundingMode {
        match (self.fpcr >> 22) & 0b11 {
            0 => RoundingMode::TiesToEven,
            1 => RoundingMode::TowardPositive,
            2 => RoundingMode::TowardNegative,
            3 => RoundingMode::TowardZero,
            _ => unreachable!()
        }
    }
    pub fn set_reg(&mut self, rd: usize, result: u64, is_stack: bool) {
        if rd == 31 {
            if is_stack {
                self.stack_reg = result
            } else {
                // nop
                return;
                //panic!(); // for early debug
            }
        } else {
            self.reg[rd] = result;
        }

    }
    pub fn init_usermode(ume: UserModeRuntime) -> Arm64Cpu {
        Arm64Cpu {
            reg: [0; 32],
            tpidr: [0; 4],
            stack_reg: 0,
            flag_status: Default::default(),
            vreg: [VectorReg::default(); 32],
            stop_exec: false,
            want_pc: None,
            pc: 0,
            memory_access: flat_mem::new_usermode(),
            is_usermode: true,
            user_struct: ume,
            want_syscall: false,
            fpcr: 0,
            fpsr: 0,
            mdata: Default::default()
        }
    }
    pub fn get_reg(&mut self, rd: usize, is_stack: bool) -> u64 {
        if rd == 31 {
            return if is_stack {
                self.stack_reg
            } else {
                0
            };
        }
        self.reg[rd]

    }
    pub fn get_stack_reg(&mut self) -> u64 {
        self.stack_reg
    }
    pub fn run(&mut self) {
        loop {
            self.exec_one_by_one();
            if self.stop_exec {
                if let Some(f) = self.want_pc {
                    self.pc = f;
                    self.want_pc = None;
                }
                if self.want_syscall {
                    self.handle_syscall();
                    self.want_syscall = false;
                }
                self.stop_exec = false;
            }
        }
    }
    pub fn exec_one_by_one(&mut self) {
        loop {
            // todo: special mrmaccessstire for instr
            let instr = self.read32(self.pc, MemAccessStr::std_loadstore()).unwrap();
            if !crate::armv8::decode::decodestep1::root_decode(self, instr) {
                self.a64_illegal_instruction();
            }
            self.pc += 4;

            if self.stop_exec {
                return;
                // could be a trap for instr, request to jump, etc...
            }
        }
    }


}
fn nofunc() {

}
impl UsermodeCpu for Arm64Cpu {
    fn push_stack_natural(&mut self, val: u64) {
        todo!()
    }

    fn pop_stack_natural(&mut self) -> u64 {
        todo!()
    }

    fn get_stack_reg(&mut self) -> u64 {
        todo!()
    }

    fn get_ume(&mut self) -> &mut UserModeRuntime {
        &mut self.user_struct
    }

    fn write_stat_t(&mut self, addr: u64, stat_t: GenericStat) {
        write_arm64_stat(addr, MemEndian::Little, stat_t);
    }

    fn write_sysinfo_t(&mut self, addr: u64, si: sysinfo) {
        write_sysinfo_generic64(addr, MemEndian::Little, si);
    }

    fn get_sigaction(&mut self, addr: u64) -> GenericSigactionArg {
        todo!()
    }

    fn get_mask(&mut self, addr: u64) -> Sigmask {
        todo!()
    }

    fn set_old_sigaction(&mut self, addr: u64, se: SigEntry) {
        todo!()
    }

    fn set_altstack(&mut self, addr: u64, si: &SigInfo) {
        todo!()
    }

    fn get_altstack(&mut self, addr: u64) -> GenericStackt {
        todo!()
    }

    fn rt_frame_setup(&mut self, sig: i32, si: &mut SigInfo) {
        todo!()
    }
}