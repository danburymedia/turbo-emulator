use crate::riscv::common::{Exception, get_privilege_encoding, get_privilege_mode, Priv, RiscvArgs, Trap};
use crate::riscv::interpreter::main::RiscvInt;
use crate::riscv::interpreter::consts::*;

fn has_csr_access_privilege(ri: &RiscvInt, address: u16) -> bool {
    let privilege = (address >> 8) & 0x3; // the lowest privilege level that can access the CSR
    privilege as u8 <= get_privilege_encoding(ri.prvmode) as u8
}
fn read_csr(ri: &mut RiscvInt, address: u16) -> Result<u64, ()> {
    match has_csr_access_privilege(ri, address) {
        true => Ok(read_csr_check(ri, address as usize)),
        false => {
            let val  = ri.get_pc_of_current_instr();
            ri.set_trap(Trap {
                ttype: Exception::IllegalInstruction,
                val
            });
            Err(())
        }
    }
}
static PASSTHROUGHS: &[u64] = &[0u64];
fn read_csr_check(ri: &mut RiscvInt, addr: usize) -> u64 {
    match addr {
        CSR_FFLAGS_ADDRESS => ri.csr[CSR_FCSR_ADDRESS as usize] & 0x1f,
        CSR_FRM_ADDRESS => (ri.csr[CSR_FCSR_ADDRESS as usize] >> 5) & 0x7,
        CSR_SSTATUS_ADDRESS => ri.csr[CSR_MSTATUS_ADDRESS as usize] & 0x80000003000de162,
        CSR_SIE_ADDRESS => ri.csr[CSR_MIE_ADDRESS as usize] & 0x222,
        CSR_SIP_ADDRESS => ri.csr[CSR_MIP_ADDRESS as usize] & 0x222,
        CSR_MHARTID_ADDRESS => { 0 } // for now.
        CSR_MTVEC_ADDRESS | CSR_SATP_ADDRESS |
        CSR_PMPADDR0_ADDRESS | CSR_PMPCFG0_ADDRESS
        | CSR_MEDELEG_ADDRESS | CSR_MIDELEG_ADDRESS
        | CSR_MIE_ADDRESS | CSR_STVEC_ADDRESS
        | CSR_MEPC_ADDRESS | CSR_MSTATUS_ADDRESS | CSR_MCAUSE_ADDRESS => {
            ri.csr[addr]
        },
        _ => panic!()
    }
}
fn write_csr_check(ri: &mut RiscvInt, addr: usize, value: u64) {
    match addr {
        CSR_FFLAGS_ADDRESS => {
            ri.csr[CSR_FCSR_ADDRESS as usize] &= !0x1f;
            ri.csr[CSR_FCSR_ADDRESS as usize] |= value & 0x1f;
        },
        CSR_FRM_ADDRESS => {
            ri.csr[CSR_FCSR_ADDRESS as usize] &= !0xe0;
            ri.csr[CSR_FCSR_ADDRESS as usize] |= (value << 5) & 0xe0;
        },
        CSR_SIE_ADDRESS => {
            // see page 32 of priv. doc
            ri.csr[CSR_MIE_ADDRESS as usize] &= !0x222;
            ri.csr[CSR_MIE_ADDRESS as usize] |= value & 0x222;
        },
        CSR_SIP_ADDRESS => {
            ri.csr[CSR_MIP_ADDRESS as usize] &= !0x222;
            ri.csr[CSR_MIP_ADDRESS as usize] |= value & 0x222;
        },
        CSR_MIDELEG_ADDRESS => {
            ri.csr[CSR_MIDELEG_ADDRESS as usize] = 0; // for now
        },
        CSR_SSTATUS_ADDRESS => {
            ri.csr[CSR_MSTATUS_ADDRESS as usize] &= !0x80000003000de162;
            ri.csr[CSR_MSTATUS_ADDRESS as usize] |= value & 0x80000003000de162;
            ri.flush_mstatus();
        },
        CSR_MSTATUS_ADDRESS => {
            ri.csr[CSR_MSTATUS_ADDRESS as usize] = value;
            ri.flush_mstatus();

        },
        CSR_SATP_ADDRESS => {
            ri.csr[addr] = value;
            ri.memsource.satp_flush(value);
        }
        CSR_MTVEC_ADDRESS | CSR_PMPADDR0_ADDRESS |
        CSR_PMPCFG0_ADDRESS | CSR_MEDELEG_ADDRESS |
         CSR_MIE_ADDRESS | CSR_FCSR_ADDRESS
        | CSR_STVEC_ADDRESS | CSR_MEPC_ADDRESS | CSR_MCAUSE_ADDRESS => {
            // passthroughs
            ri.csr[addr] = value;

        }
        _ => {
            panic!();
        }
    }
}
pub fn csrrc(ri: &mut RiscvInt, args: &RiscvArgs) {
    let data = match read_csr(ri, args.csr as u16) {
        Ok(z) => z,
        Err(_) => return
    };
    let tmp = ri.regs[args.rs1 as usize];
    ri.regs[args.rd as usize] = ri.sign_ext(data);
    if args.rs1 != 0 {
        write_csr_check(ri, args.csr as usize, ri.regs[args.rd as usize] & !tmp)

    }
}
pub fn csrrci(ri: &mut RiscvInt, args: &RiscvArgs) {
    let data = match read_csr(ri, args.csr as u16) {
        Ok(z) => z,
        Err(_) => return
    };
    ri.regs[args.rd as usize] = ri.sign_ext(data);
    if args.rs1 != 0 {
        write_csr_check(ri, args.csr as usize, ri.regs[args.rd as usize] & !(args.rs1 as u64))
    }
}
pub fn csrrs(ri: &mut RiscvInt, args: &RiscvArgs) {
    let data = match read_csr(ri, args.csr as u16) {
        Ok(z) => z,
        Err(_) => return
    };
    let tmp = ri.regs[args.rs1 as usize];
    ri.regs[args.rd as usize] = ri.sign_ext(data);
    if args.rs1 != 0 {
        write_csr_check(ri, args.csr as usize, ri.regs[args.rd as usize] | tmp)
    }
}
pub fn csrrsi(ri: &mut RiscvInt, args: &RiscvArgs) {
    let data = match read_csr(ri, args.csr as u16) {
        Ok(z) => z,
        Err(_) => return
    };
    let tmp = ri.regs[args.rs1 as usize];
    ri.regs[args.rd as usize] = ri.sign_ext(data);
    if args.rs1 != 0 {
        write_csr_check(ri, args.csr as usize, ri.regs[args.rd as usize] | (args.rs1 as u64))
    }
}
pub fn csrrw(ri: &mut RiscvInt, args: &RiscvArgs) {
    let data = match read_csr(ri, args.csr as u16) {
        Ok(z) => z,
        Err(_) => return
    };
    let tmp = ri.regs[args.rs1 as usize];
    ri.regs[args.rd as usize] = ri.sign_ext(data);
    write_csr_check(ri, args.csr as usize, ri.cull_reg(tmp))
}
pub fn ecall(ri: &mut RiscvInt, args: &RiscvArgs) {
    let exception_type = match ri.prvmode {
        Priv::UserApp => Exception::EnvironmentCallFromUMode,
        Priv::Supervisor => Exception::EnvironmentCallFromSMode,
        Priv::Machine => Exception::EnvironmentCallFromMMode,
        Priv::Reserved => panic!("Reserved priv mode (how are we even here)")
    };
    let val = ri.get_pc_of_current_instr();
    ri.set_trap(Trap {
        ttype: exception_type,
        val
    })

}
pub fn fence(ri: &mut RiscvInt, args: &RiscvArgs) {
}
pub fn mret(ri: &mut RiscvInt, args: &RiscvArgs) {
    ri.stop_exec = true;
    ri.want_pc = match read_csr(ri, CSR_MEPC_ADDRESS as u16) {
        Ok(z) => Some(z),
        Err(_) => return // trap
    };
    let status = read_csr_check(ri, CSR_MSTATUS_ADDRESS);
    let mpie = (status >> 7) & 1;
    let mpp = (status >> 11) & 0x3;
    let mprv = match get_privilege_mode(mpp) {
        Priv::Machine => (status >> 17) & 1,
        _ => 0
    };
    let new_status = (status & !0x21888) | (mprv << 17) | (mpie << 3) | (1 << 7);
    write_csr_check(ri, CSR_MSTATUS_ADDRESS, new_status);
    let privs = match mpp {
        0 => Priv::UserApp,
        1 => Priv::Supervisor,
        3 => Priv::Machine,
        _ => panic!() // Shouldn't happen
    };
    ri.change_priv(privs);
}
pub fn csrrwi(ri: &mut RiscvInt, args: &RiscvArgs) {
    if args.rd != 0 {
        // even though we reset zero reg, read_csr can cause trap so follow manual
        let data = match read_csr(ri, args.csr as u16) {
            Ok(z) => z,
            Err(_) => return
        };
        ri.regs[args.rd as usize] = ri.sign_ext(data);
    }
    write_csr_check(ri, args.csr as usize, args.rs1 as u64)
}