use crate::armv8::common::ArmExt;
use crate::armv8::decodedefs::ArmInstr;
use crate::armv8::interpreter::helpers::{extend_value, ExtendType};
use crate::armv8::interpreter::main::{Arm64Cpu};
use crate::armv8::interpreter::mem::MemAccessStr;
use crate::armv8::interpreter::neon::cvt_size_to_vecinfo;
use crate::armv8::interpreter::vect_helper::VectorReg;
use crate::common::signext_arbpos;
use crate::common::vect::VectInfo;

#[derive(Copy, Clone, PartialEq)]
pub enum AddressMode {
    Imm,
    PreIdx,
    PostIdx
}
fn write_arbsize(ai: &mut Arm64Cpu, address: u64, data: u64, size: u64, mem: MemAccessStr) -> bool {
    match size {
        8 => {
            ai.write8(address, data as u8, mem)
        },
        16 => {
            ai.write16(address, data as u16, mem)
        }
        32 => {
            ai.write32(address, data as u32, mem)
        }
        64 => {
            ai.write64(address, data as u64, mem)
        },
        _ => unreachable!()
    }
}
fn read_arbsize(ai: &mut Arm64Cpu, address: u64, signed: bool,
                    size: u64, mem: MemAccessStr) -> Option<u64> {
    match size {
        8 => {
            let res = ai.read8(address, mem);
            if res == None {
                return None;
            }
            let realread = res.unwrap();
            Some(if signed {
                realread as i8 as i64 as u64
            } else {
                realread as u64
            })
        }
        16 => {
            let res = ai.read16(address, mem);
            if res == None {
                return None;
            }
            let realread = res.unwrap();
            if signed {
                Some(realread as i16 as i64 as u64)
            } else {
                Some(realread as u64)
            }
        }
        32 => {
            let res = ai.read32(address, mem);
            if res == None {
                return None;
            }
            let realread = res.unwrap();
            if signed {
                Some(realread as i32 as i64 as u64)
            } else {
                Some(realread as u64)
            }
        }
        64 => {
            let res = ai.read64(address, mem);
            if res == None {
                return None;
            }
            let realread = res.unwrap();
            Some(realread)
        }
        _ => panic!()
    }
}

fn load_store_helper(ai: &mut Arm64Cpu, args: &ArmInstr, offset: u64, postindex: bool,
                         mem: MemAccessStr, store: bool,
                         signed: bool, size: u32, wback: bool, cut_32: bool) {
    let srcdst = args.get_rt();
    let addr_reg = args.get_rn();
    let address = verify_addr_and_writeback(ai, addr_reg,
                                            offset, postindex).unwrap(); // for now
    if store {
        let val = ai.get_reg(srcdst, false);
        let status = write_arbsize(ai, address, val, size as u64, mem);
        if status {
            // error while doing write
            return;
        }

    } else {
        // load
        let mut val: u64 = read_arbsize(ai, address, signed, size as u64, mem).unwrap();
        if cut_32 {
            val &= 0xffffffff;
        }
        ai.set_reg(srcdst, val, false);
    }
    if wback {
        let mut writeaddr = address;
        if postindex {
            writeaddr = address.wrapping_add(offset);
        }
        ai.set_reg(addr_reg, writeaddr, true);
    }


}
fn verify_addr_and_writeback(ai: &mut Arm64Cpu, source: usize, offset: u64, postindex: bool) -> Option<u64> {
    let realaddr = ai.get_reg(source, true);
    if (source == 31) & (realaddr % 16 != 0) {
        // unalgined, todo: do properly
        panic!();
        return None;
    }
    if  postindex {
        Some(realaddr)
    } else {
        let val2 = realaddr.wrapping_add(offset);
        Some(val2)
    }
}
pub fn addr_writeback(ai: &mut Arm64Cpu, dest: usize, offset: u64, addr: AddressMode){
    if addr == AddressMode::PostIdx || addr == AddressMode::PreIdx {
        let realaddr = ai.get_reg(dest, true);
        ai.set_reg(dest, realaddr + offset, true);
    }
}
pub fn sturb(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let imm = args.get_loadstore_reg_imm9();
    load_store_helper(ai, args, imm, false,
                      MemAccessStr::std_loadstore(), true,
                      false, 8, false, false);
}
pub fn ldurb(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let imm = args.get_loadstore_reg_imm9();
    load_store_helper(ai, args, imm, false,
                      MemAccessStr::std_loadstore(), false,
                      false, 8, false, false);
}
pub fn ldursb(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let imm = args.get_loadstore_reg_imm9();
    let cut_32 = if (args.insn & (1 << 22)) != 0 { true } else { false };
    load_store_helper(ai, args, imm, false,
                      MemAccessStr::std_loadstore(), false,
                      true, 8, false, cut_32);
}
pub fn sturh(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let imm = args.get_loadstore_reg_imm9();
    load_store_helper(ai, args, imm, false,
                      MemAccessStr::std_loadstore(), true,
                      false, 16, false, false);
}
pub fn ldurh(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let imm = args.get_loadstore_reg_imm9();
    load_store_helper(ai, args, imm, false,
                      MemAccessStr::std_loadstore(), false,
                      false, 16, false, false);
}
pub fn ldursh(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let imm = args.get_loadstore_reg_imm9();
    let cut_32 = if (args.insn & (1 << 22)) != 0 { true } else { false };
    load_store_helper(ai, args, imm, false,
                      MemAccessStr::std_loadstore(), false,
                      true, 16, false, cut_32);
}
pub fn stur_gen(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let imm = args.get_loadstore_reg_imm9();
    let size = if (args.insn & (1 << 30)) != 0 { 64 } else { 32 };
    load_store_helper(ai, args, imm, false,
                      MemAccessStr::std_loadstore(), true,
                      false, size, false, false);
}
pub fn ldur_gen(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let imm = args.get_loadstore_reg_imm9();
    let size = if (args.insn & (1 << 30)) != 0 { 64 } else { 32 };
    load_store_helper(ai, args, imm, false,
                      MemAccessStr::std_loadstore(), false,
                      false, size, false, false);
}
pub fn ldursw(ai: &mut Arm64Cpu, args: &ArmInstr) {
    // by definition, 32-bit word loaded into 64 bit reg
    let imm = args.get_loadstore_reg_imm9();
    load_store_helper(ai, args, imm, false,
                      MemAccessStr::std_loadstore(), false,
                      true, 32, false, false);
}
pub fn strb_reg(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let exttype = ExtendType::num2type(((args.insn >> 13) & 0b111) as u8);
    let shift = 0; // for strb no matter what it will always be 0
    let rm = ai.get_reg(args.get_rm(), false);
    let offset = extend_value(rm, exttype,shift);
    load_store_helper(ai, args, offset, false,
                      MemAccessStr::std_loadstore(), true,
                      false, 8, false, false);

}
pub fn ldrb_reg(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let exttype = ExtendType::num2type(((args.insn >> 13) & 0b111) as u8);
    let shift = 0; // for strb no matter what it will always be 0
    let rm = ai.get_reg(args.get_rm(), false);
    let offset = extend_value(rm, exttype,shift);
    load_store_helper(ai, args, offset, false,
                      MemAccessStr::std_loadstore(), false,
                      false, 8, false, false);

}
pub fn ldrsb_reg(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let exttype = ExtendType::num2type(((args.insn >> 13) & 0b111) as u8);
    let shift = 0; // for strb no matter what it will always be 0
    let rm = ai.get_reg(args.get_rm(), false);
    let offset = extend_value(rm, exttype,shift);
    let cut_32 = if (args.insn & (1 << 22)) != 0 { true } else { false };
    load_store_helper(ai, args, offset, false,
                      MemAccessStr::std_loadstore(), false,
                      true, 8, false, cut_32);
}
pub fn strh_reg(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let exttype = ExtendType::num2type(((args.insn >> 13) & 0b111) as u8);
    let shift= if (args.insn & (1 << 12)) != 0 { 1 } else { 0 };
    let rm = ai.get_reg(args.get_rm(), false);
    let offset = extend_value(rm, exttype,shift);
    load_store_helper(ai, args, offset, false,
                      MemAccessStr::std_loadstore(), true,
                      false, 16, false, false);
}
pub fn ldrh_reg(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let exttype = ExtendType::num2type(((args.insn >> 13) & 0b111) as u8);
    let shift= if (args.insn & (1 << 12)) != 0 { 1 } else { 0 };
    let rm = ai.get_reg(args.get_rm(), false);
    let offset = extend_value(rm, exttype,shift);
    load_store_helper(ai, args, offset, false,
                      MemAccessStr::std_loadstore(), false,
                      false, 16, false, false);
}
pub fn ldrsh_reg(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let exttype = ExtendType::num2type(((args.insn >> 13) & 0b111) as u8);
    let cut_32 = if (args.insn & (1 << 22)) != 0 { true } else { false };
    let shift= if (args.insn & (1 << 12)) != 0 { 1 } else { 0 };
    let rm = ai.get_reg(args.get_rm(), false);
    let offset = extend_value(rm, exttype,shift);
    load_store_helper(ai, args, offset, false,
                      MemAccessStr::std_loadstore(), false,
                      true, 16, false, cut_32);
}
pub fn str_reg_gen(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let exttype = ExtendType::num2type(((args.insn >> 13) & 0b111) as u8);
    let scale = args.get_ls_size(); // 0b11 or 0b10
    let size = if scale == 0b11 { 64 } else if scale == 0b10 { 32 } else { panic!(); };
    let shift= if (args.insn & (1 << 12)) != 0 { scale } else { 0 };
    let rm = ai.get_reg(args.get_rm(), false);
    let offset = extend_value(rm, exttype, shift as usize);
    load_store_helper(ai, args, offset, false,
                      MemAccessStr::std_loadstore(), true,
                      false, size, false, false);
}
pub fn ldr_reg_gen(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let exttype = ExtendType::num2type(((args.insn >> 13) & 0b111) as u8);
    let scale = args.get_ls_size(); // 0b11 or 0b10
    let size = if scale == 0b11 { 64 } else if scale == 0b10 { 32 } else { panic!(); };
    let shift= if (args.insn & (1 << 12)) != 0 { scale } else { 0 };
    let rm = ai.get_reg(args.get_rm(), false);
    let offset = extend_value(rm, exttype, shift as usize);
    load_store_helper(ai, args, offset, false,
                      MemAccessStr::std_loadstore(), false,
                      false, size, false, false);
}
pub fn ldrsw_reg(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let exttype = ExtendType::num2type(((args.insn >> 13) & 0b111) as u8);
    let shift= if (args.insn & (1 << 12)) != 0 { 2 } else { 0 };
    let rm = ai.get_reg(args.get_rm(), false);
    let offset = extend_value(rm, exttype, shift as usize);
    load_store_helper(ai, args, offset, false,
                      MemAccessStr::std_loadstore(), false,
                      true, 32, false, false);

}
pub fn strb_imm(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let mut wback = false;
    let mut postindex = false;
    let mut offset = 0;
    let scale = args.get_ls_size();
    if (args.insn & (1 << 24)) != 0 {
        // unsigned offset
        offset = args.get_loadstore_reg_imm12();
    } else {
        if (args.insn & (1 << 11)) != 0 {
            // pre index
            offset = args.get_loadstore_reg_imm9();
            wback = true;
        } else {
            // post index
            offset = args.get_loadstore_reg_imm9();
            wback = true;
            postindex = true;
        }
    }
    load_store_helper(ai, args, offset, postindex,
                      MemAccessStr::std_loadstore(), true,
                      false, 8, wback, false);

}
pub fn ldrb_imm(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let mut wback = false;
    let mut postindex = false;
    let mut offset = 0;
    let scale = args.get_ls_size();
    if (args.insn & (1 << 24)) != 0 {
        // unsigned offset
        offset = args.get_loadstore_reg_imm12();
    } else {
        if (args.insn & (1 << 11)) != 0 {
            // pre index
            offset = args.get_loadstore_reg_imm9();
            wback = true;
        } else {
            // post index
            offset = args.get_loadstore_reg_imm9();
            wback = true;
            postindex = true;
        }
    }
    load_store_helper(ai, args, offset, postindex,
                      MemAccessStr::std_loadstore(), false,
                      false, 8, wback, false);

}
pub fn ldrsb_imm(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let mut wback = false;
    let mut postindex = false;
    let mut offset = 0;
    let scale = args.get_ls_size();
    if (args.insn & (1 << 24)) != 0 {
        // unsigned offset
        offset = args.get_loadstore_reg_imm12();
    } else {
        if (args.insn & (1 << 11)) != 0 {
            // pre index
            offset = args.get_loadstore_reg_imm9();
            wback = true;
        } else {
            // post index
            offset = args.get_loadstore_reg_imm9();
            wback = true;
            postindex = true;
        }
    }
    let cut_32 = if (args.insn & (1 << 22)) != 0 { true } else { false };

    load_store_helper(ai, args, offset, postindex,
                      MemAccessStr::std_loadstore(), false,
                      true, 8, wback, cut_32);

}
pub fn strh_imm(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let mut wback = false;
    let mut postindex = false;
    let mut offset;
    let scale = args.get_ls_size();
    if (args.insn & (1 << 24)) != 0 {
        // unsigned offset
        offset = args.get_loadstore_reg_imm12() << 1;
    } else {
        if (args.insn & (1 << 11)) != 0 {
            // pre index
            offset = args.get_loadstore_reg_imm9();
            wback = true;
        } else {
            // post index
            offset = args.get_loadstore_reg_imm9();
            wback = true;
            postindex = true;
        }
    }
    load_store_helper(ai, args, offset, postindex,
                      MemAccessStr::std_loadstore(), true,
                      false, 16, wback, false);

}
pub fn ldrh_imm(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let mut wback = false;
    let mut postindex = false;
    let mut offset = 0;
    let scale = args.get_ls_size();
    if (args.insn & (1 << 24)) != 0 {
        // unsigned offset
        offset = args.get_loadstore_reg_imm12() << 1;
    } else {
        if (args.insn & (1 << 11)) != 0 {
            // pre index
            offset = args.get_loadstore_reg_imm9();
            wback = true;
        } else {
            // post index
            offset = args.get_loadstore_reg_imm9();
            wback = true;
            postindex = true;
        }
    }
    load_store_helper(ai, args, offset, postindex,
                      MemAccessStr::std_loadstore(), false,
                      false, 16, wback, false);

}
pub fn ldrsh_imm(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let mut wback = false;
    let mut postindex = false;
    let mut offset = 0;
    let scale = args.get_ls_size();
    if (args.insn & (1 << 24)) != 0 {
        // unsigned offset
        offset = args.get_loadstore_reg_imm12() << 1;
    } else {
        if (args.insn & (1 << 11)) != 0 {
            // pre index
            offset = args.get_loadstore_reg_imm9();
            wback = true;
        } else {
            // post index
            offset = args.get_loadstore_reg_imm9();
            wback = true;
            postindex = true;
        }
    }
    let cut_32 = if (args.insn & (1 << 22)) != 0 { true } else { false };

    load_store_helper(ai, args, offset, postindex,
                      MemAccessStr::std_loadstore(), false,
                      true, 16, wback, cut_32);

}
pub fn ldr_imm_gen(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let mut wback = false;
    let mut postindex = false;
    let mut offset = 0;
    let scale = args.get_ls_size();
    let size = if scale == 0b11 { 64 } else if scale == 0b10 { 32 } else { panic!(); };
    if (args.insn & (1 << 24)) != 0 {
        // unsigned offset
        offset = args.get_loadstore_reg_imm12() << scale;
    } else {
        if (args.insn & (1 << 11)) != 0 {
            // pre index
            offset = args.get_loadstore_reg_imm9();
            wback = true;
        } else {
            // post index
            offset = args.get_loadstore_reg_imm9();
            wback = true;
            postindex = true;
        }
    }
    load_store_helper(ai, args, offset, postindex,
                      MemAccessStr::std_loadstore(), false,
                      false, size, wback, false);

}
pub fn str_imm_gen(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let mut wback = false;
    let mut postindex = false;
    let mut offset = 0;
    let scale = args.get_ls_size();
    let size = if scale == 0b11 { 64 } else if scale == 0b10 { 32 } else { panic!(); };
    if (args.insn & (1 << 24)) != 0 {
        // unsigned offset
        offset = args.get_loadstore_reg_imm12() << scale;
    } else {
        if (args.insn & (1 << 11)) != 0 {
            // pre index
            offset = args.get_loadstore_reg_imm9();
            wback = true;
        } else {
            // post index
            offset = args.get_loadstore_reg_imm9();
            wback = true;
            postindex = true;
        }
    }
    load_store_helper(ai, args, offset, postindex,
                      MemAccessStr::std_loadstore(), true,
                      false, size, wback, false);

}
pub fn ldrsw_imm(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let mut wback = false;
    let mut postindex = false;
    let mut offset = 0;
    let scale = args.get_ls_size();
    if (args.insn & (1 << 24)) != 0 {
        // unsigned offset
        offset = args.get_loadstore_reg_imm12() << 2;
    } else {
        if (args.insn & (1 << 11)) != 0 {
            // pre index
            offset = args.get_loadstore_reg_imm9();
            wback = true;
        } else {
            // post index
            offset = args.get_loadstore_reg_imm9();
            wback = true;
            postindex = true;
        }
    }
    load_store_helper(ai, args, offset, postindex,
                      MemAccessStr::std_loadstore(), false,
                      true, 32, wback, false);

}
pub fn load_store_pair_helper(ai: &mut Arm64Cpu, args: &ArmInstr,
                         store: bool,
                         signed: bool) {

    let mut wback = false;
    let mut postindex = false;
    let opc = (args.insn >> 23) & 0b11;
    if opc == 0b01 {
        // post index
        wback = true;
        postindex = true;
    } else if opc == 0b10 {
        // signed offset
    } else if opc == 0b11 {
        // pre index
        wback = true;
    } else {
        unimplemented!();
    }

    let mem = MemAccessStr::std_loadstore();
    let t = args.get_rt();
    let t2 = args.get_rt2();
    let size = if (args.insn & (1 << 31)) != 0 { 64 } else { 32 };
    let nextbyte = if size == 64 { 8 } else if size == 32 { 4 } else { unreachable!() };
    let scale = if size == 64 { 3 } else { 2 };
    let addr_reg = args.get_rn();
    let offset = args.get_loadstore_pair_imm7() << scale;
    let address = verify_addr_and_writeback(ai, addr_reg,
                                            offset, postindex).unwrap(); // for now. addresses always 64 bit
    let address2 = address + nextbyte;
    if store {
        let data1 = ai.get_reg(t, false);
        let data2 = ai.get_reg(t2, false);
        if ai.is_feat_avail(ArmExt::Lse2) {
            unimplemented!();
        } else {
            let status1 = write_arbsize(ai, address, data1, size, mem);
            if status1 {
                // error while doing write
                return;
            }
            let status2 = write_arbsize(ai, address2, data2, size, mem);
            if status2 {
                // error while doing write
                return;
            }
        }


    } else {
        // load
        let (data1, data2) = if ai.is_feat_avail(ArmExt::Lse2) && !signed {
            unimplemented!();
        } else {
            (
                read_arbsize(ai, address, signed, size, mem).unwrap(),
                read_arbsize(ai, address2, signed, size, mem).unwrap(),

            )

        };
        // if needed, already signext via read_arbsize()
        ai.set_reg(t, data1, false);
        ai.set_reg(t2, data2, false);
    }
    if wback {
        let mut writeaddr = address;
        if postindex {
            writeaddr = address + offset;
        }
        ai.set_reg(addr_reg, writeaddr, true);
    }


}
pub fn ldp_gen(ai: &mut Arm64Cpu, args: &ArmInstr) {
    load_store_pair_helper(ai, args, false, false);

}
pub fn stp_gen(ai: &mut Arm64Cpu, args: &ArmInstr) {
    load_store_pair_helper(ai, args, true, false);
}
pub fn ldpsw(ai: &mut Arm64Cpu, args: &ArmInstr) {
    load_store_pair_helper(ai, args, false, true);
}
fn load_store_simd_helper(ai: &mut Arm64Cpu, args: &ArmInstr, offset: u64, postindex: bool,
                     mem: MemAccessStr, store: bool,
                     size: u32, wback: bool) {
    let srcdst = args.get_rt();
    let addr_reg = args.get_rn();
    let address = verify_addr_and_writeback(ai, addr_reg,
                                            offset, postindex).unwrap(); // for now
    if store {
        let val = ai.vreg[srcdst].vect;
        let status = if size != 128 {
            write_arbsize(ai, address, val as u64, size as u64, mem)
        } else if size == 128 {
            ai.write64(address, val as u64, mem); // todo: exit if fault here
            ai.write64(address + 8, (val >> 64) as u64, mem)
        } else {
            unreachable!();
        };
        if status {
            // error while doing write
            return;
        }

    } else {
        // load
        let mut val: u128  = if size != 128 {
            read_arbsize(ai, address, false, size as u64, mem).unwrap() as u128
        } else if size == 128 {
            let res1 = ai.read64(address, mem);
            if res1 == None {
                return;
            }
            let res2 = ai.read64(address + 8, mem);
            if res2 == None {
                return;
            }
            let rres1 = res1.unwrap();
            let rres2 = res2.unwrap();
            let fres = (rres1 as u128) | ((rres2 as u128) << 64);
            fres
        } else {
            unreachable!();
        };
        ai.vreg[srcdst].vect = val;
    }
    if wback {
        let mut writeaddr = address;
        if postindex {
            writeaddr = address + offset;
        }
        ai.set_reg(addr_reg, writeaddr, true);
    }


}
pub fn ldr_reg_fpsimd(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let exttype = ExtendType::num2type(((args.insn >> 13) & 0b111) as u8);
    let scale_exbit = ((args.insn >> 23) & 1 )as u64;
    let scale = args.get_ls_size() | scale_exbit << 2;
    let size = if scale == 0b011 {
        64
    }
    else if scale == 0b010 {
        32
    } else if scale == 0b001 {
        16
    } else if scale == 0b100 {
        128
    } else if scale == 0 {
        8
    } else {
        panic!();
    };
    let shift= if (args.insn & (1 << 12)) != 0 { scale } else { 0 };
    let rm = ai.get_reg(args.get_rm(), false);
    let offset = extend_value(rm, exttype, shift as usize);
    load_store_simd_helper(ai, args, offset, false,
                      MemAccessStr::std_loadstore(), false,
                       size, false);
}
pub fn str_reg_fpsimd(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let exttype = ExtendType::num2type(((args.insn >> 13) & 0b111) as u8);
    let scale_exbit = ((args.insn >> 23) & 1) as u64;
    let scale = args.get_ls_size() | scale_exbit << 2;
    let size = if scale == 0b011 {
        64
    }
    else if scale == 0b010 {
        32
    } else if scale == 0b001 {
        16
    } else if scale == 0b100 {
        128
    } else if scale == 0 {
        8
    } else {
        panic!();
    };
    let shift= if (args.insn & (1 << 12)) != 0 { scale } else { 0 };
    let rm = ai.get_reg(args.get_rm(), false);
    let offset = extend_value(rm, exttype, shift as usize);
    load_store_simd_helper(ai, args, offset, false,
                           MemAccessStr::std_loadstore(), true,
                            size, false);
}
pub fn stur_fpsimd(ai: &mut Arm64Cpu, arg: &ArmInstr) {
    let scale_exbit = ((arg.insn >> 23) & 1) as u64;
    let scale = arg.get_ls_size() | scale_exbit << 2;
    let size = if scale == 0b011 {
        64
    } else if scale == 0b010 {
        32
    } else if scale == 0b001 {
        16
    } else if scale == 0b100 {
        128
    } else if scale == 0 {
        8
    } else {
        panic!();
    };
    let imm9 = (arg.insn >> 12) & 0b111_111_111;
    let offset = signext_arbpos(imm9 as u64, 9);
    load_store_simd_helper(ai, arg, offset, false,
                           MemAccessStr::std_loadstore(), true,
                           size, false);
}
pub fn str_imm_fpsimd(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let mut wback = false;
    let mut postindex = false;
    let mut offset = 0;
    let scale_exbit = ((args.insn >> 23) & 1) as u64;
    let scale = args.get_ls_size() | scale_exbit << 2;
    let size = if scale == 0b011 {
        64
    }
    else if scale == 0b010 {
        32
    } else if scale == 0b001 {
        16
    } else if scale == 0b100 {
        128
    } else if scale == 0 {
        8
    } else {
        panic!();
    };

    if (args.insn & (1 << 24)) != 0 {
        // unsigned offset
        offset = args.get_loadstore_reg_imm12() << scale;
    } else {
        if (args.insn & (1 << 11)) != 0 {
            // pre index
            offset = args.get_loadstore_reg_imm9();
            wback = true;
        } else {
            // post index
            offset = args.get_loadstore_reg_imm9();
            wback = true;
            postindex = true;
        }
    }
    load_store_simd_helper(ai, args, offset, postindex,
                           MemAccessStr::std_loadstore(), true,
                            size, wback);
}
pub fn ldr_imm_fpsimd(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let mut wback = false;
    let mut postindex = false;
    let mut offset = 0;
    let scale_exbit = ((args.insn >> 23) & 1) as u64;
    let scale = args.get_ls_size() | scale_exbit << 2;
    let size = if scale == 0b011 {
        64
    }
    else if scale == 0b010 {
        32
    } else if scale == 0b001 {
        16
    } else if scale == 0b100 {
        128
    } else if scale == 0 {
        8
    } else {
        panic!();
    };

    if (args.insn & (1 << 24)) != 0 {
        // unsigned offset
        offset = args.get_loadstore_reg_imm12() << scale;
    } else {
        if (args.insn & (1 << 11)) != 0 {
            // pre index
            offset = args.get_loadstore_reg_imm9();
            wback = true;
        } else {
            // post index
            offset = args.get_loadstore_reg_imm9();
            wback = true;
            postindex = true;
        }
    }
    load_store_simd_helper(ai, args, offset, postindex,
                           MemAccessStr::std_loadstore(), false,
                            size, wback);
}
pub fn load_store_pair_simd_helper(ai: &mut Arm64Cpu, args: &ArmInstr,
                              store: bool) {

    let mut wback = false;
    let mut postindex = false;
    let opn = (args.insn >> 23) & 0b11;
    if opn == 0b01 {
        // post index
        wback = true;
        postindex = true;
    } else if opn == 0b10 {
        // signed offset
    } else if opn == 0b11 {
        // pre index
        wback = true;
    } else {
        unimplemented!();
    }

    let mem = MemAccessStr::std_loadstore();
    let t = args.get_rt();
    let t2 = args.get_rt2();
    let opc = (args.insn >> 30) & 0b11;
    let size = match opc {
        0 => {
            32
        }
        1 => {
            64
        }
        2 => {
            128
        }
        _ => unreachable!()
    };
    let nextbyte = size / 8;
    let scale = 2 + opc;
    let addr_reg = args.get_rn();
    let offset = args.get_loadstore_pair_imm7() << scale;
    let address = verify_addr_and_writeback(ai, addr_reg,
                                            offset, postindex).unwrap(); // for now. addresses always 64 bit
    let address2 = address + nextbyte;
    if store {
        let data1 = ai.vreg[t as usize].vect;
        let data2 = ai.vreg[t2 as usize].vect;
        if ai.is_feat_avail(ArmExt::Lse2) {
            unimplemented!();
        } else {
            let status1 = if size != 128 {
                write_arbsize(ai, address, data1 as u64, size as u64, mem)
            } else if size == 128 {
                ai.write64(address, data1 as u64, mem); // todo: exit if fault here
                ai.write64(address + 8, (data1 >> 64) as u64, mem)
            } else {
                unreachable!()
            };
            if status1 {
                // error while doing write
                return;
            }
            let status2 = if size != 128 {
                write_arbsize(ai, address2, data2 as u64, size as u64, mem)
            } else if size == 128 {
                ai.write64(address2, data2 as u64, mem); // todo: exit if fault here
                ai.write64(address2 + 8, (data2 >> 64) as u64, mem)
            } else {
                unreachable!()
            };
            if status2 {
                // error while doing write
                return;
            }
        }
    } else {
        // load
        let data1 = if size != 128 {
            read_arbsize(ai, address, false, size as u64, mem).unwrap() as u128
        } else if size == 128 {
            let res1 = ai.read64(address, mem);
            if res1 == None {
                return;
            }
            let res2 = ai.read64(address + 8, mem);
            if res2 == None {
                return;
            }
            let rres1 = res1.unwrap();
            let rres2 = res2.unwrap();
            let fres = (rres1 as u128) | ((rres2 as u128) << 64);
            fres
        } else {
            unreachable!()
        };
        let data2 = if size != 128 {
            read_arbsize(ai, address2, false, size as u64, mem).unwrap() as u128
        } else if size == 128 {
            let res1 = ai.read64(address2, mem);
            if res1 == None {
                return;
            }
            let res2 = ai.read64(address2 + 8, mem);
            if res2 == None {
                return;
            }
            let rres1 = res1.unwrap();
            let rres2 = res2.unwrap();
            let fres = (rres1 as u128) | ((rres2 as u128) << 64);
            fres
        } else {
            unreachable!()
        };
        ai.vreg[t].vect = data1;
        ai.vreg[t2].vect = data2;
    }
    if wback {
        let mut writeaddr = address;
        if postindex {
            writeaddr = address + offset;
        }
        ai.set_reg(addr_reg, writeaddr, true);
    }


}
pub fn ldp_fpsimd(ai: &mut Arm64Cpu, args: &ArmInstr) {
    load_store_pair_simd_helper(ai, args, false);

}
pub fn stp_fpsimd(ai: &mut Arm64Cpu, args: &ArmInstr) {
    load_store_pair_simd_helper(ai, args, true);
}
fn load_lane(ai: &mut Arm64Cpu, dst: &mut VectorReg, vinfo: VectInfo, idx: usize, addr: u64) -> bool {
    let data = read_arbsize(ai, addr, false,
                            vinfo.elem_size as u64, MemAccessStr::std_loadstore()).unwrap();
    dst.set_elem_fixed(data, idx, vinfo);
    false

}
fn store_lane(ai: &mut Arm64Cpu, dst: &mut VectorReg, vinfo: VectInfo, idx: usize, addr: u64) -> bool {
    let storeval = dst.get_elem_fixed(idx, vinfo);
    write_arbsize(ai, addr, storeval, vinfo.elem_size as u64,
                  MemAccessStr::std_loadstore())

}
fn ld1(ai: &mut Arm64Cpu, dst: &mut VectorReg, vinfo: VectInfo, addr: u64) {
    dst.clear_vect();
    let mut realaddr = addr;
    let elemsizebytes= vinfo.elem_size / 8;
    for idx in 0..vinfo.lane_count {
        load_lane(ai, dst, vinfo, idx, realaddr);
        realaddr += (elemsizebytes as u64);
    }
}
fn ld2(ai: &mut Arm64Cpu, dst1: &mut VectorReg, dst2: &mut VectorReg,  vinfo: VectInfo, addr: u64) {
    dst1.clear_vect();
    dst2.clear_vect();
    let elemsizebytes= (vinfo.elem_size / 8) as u64;
    let mut addr1 = addr;
    let mut addr2 = addr + elemsizebytes;
    for idx in 0..vinfo.lane_count {
        load_lane(ai, dst1, vinfo, idx, addr1);
        load_lane(ai, dst2, vinfo, idx, addr2);
        addr1 += 2 * elemsizebytes;
        addr2 += 2 * elemsizebytes;
    }
}
fn st1(ai: &mut Arm64Cpu, src: &mut VectorReg, vinfo: VectInfo, addr: u64) {
    let mut realaddr = addr;
    let elemsizebytes= vinfo.elem_size / 8;
    for idx in 0..vinfo.lane_count {
        store_lane(ai, src, vinfo, idx, realaddr);
        realaddr += (elemsizebytes as u64);
    }
}
#[derive(Copy, Clone)]
pub enum LoadStoreSIMDMultiTypes {
    Ld1(u32),
    St1(u32),
    Ld2,
}
fn load_store_multi_gen(ai: &mut Arm64Cpu, arg: &ArmInstr, typ: LoadStoreSIMDMultiTypes, postidx: bool) {
    let addrbase = ai.get_reg(arg.get_rn(), true);
    let q = (arg.insn >> 30) & 1;
    let size = (arg.insn >> 10) & 0b11;
    let vinfo = cvt_size_to_vecinfo(size as u8, q as u8);
    let reg_size_bytes = ((vinfo.elem_size * vinfo.lane_count) / 8) as u64;
    let mut reg: [usize; 4] = [0; 4];
    let mut addr: [u64; 4] = [0; 4];
    let rt = arg.get_rt();
    for i in 0..4 {
        reg[i]  = (rt + i) % 32;
        addr[i] = addrbase + ((i as u64) * reg_size_bytes);
    }
    let mut regcount = 0;
    match typ {
        LoadStoreSIMDMultiTypes::Ld1(k) => {
            for i in (0..k) {
                let regidx = reg[i as usize];
                let mut dst = ai.vreg[regidx as usize];
                ld1(ai, &mut dst, vinfo, addr[i as usize]);
                ai.vreg[regidx as usize] = dst;
                regcount += 1;
            }
        }
        LoadStoreSIMDMultiTypes::St1(k) => {
            for i in (0..k) {
                let regidx = reg[i as usize];
                let mut src = ai.vreg[regidx as usize];
                st1(ai, &mut src, vinfo, addr[i as usize]);
                regcount += 1;
            }
        },
        LoadStoreSIMDMultiTypes::Ld2 => {
            let mut dst1 = ai.vreg[0];
            let mut dst2 = ai.vreg[1];
            let addr = addr[0];
            ld2(ai, &mut dst1, &mut dst2, vinfo, addr);
            ai.vreg[0] = dst1;
            ai.vreg[1] = dst2;
        }
    }
    if postidx {
        let rm = arg.get_rm();
        let mut newaddrbase = if rm == 31 {
            addrbase + (reg_size_bytes * regcount)
        } else {
            let rmval = ai.get_reg(rm, false);
            addrbase + rmval
        };
        ai.set_reg(arg.get_rn(), newaddrbase, true);
    }

}
pub fn ldaxr(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let size = args.get_ls_size();
    let addr = ai.get_reg(args.get_rn(), false);
    let val = if size == 0b10 {
        ai.set_exclusive_monitors(addr, 4);
        ai.read32(addr, MemAccessStr::atomic_load()).unwrap() as u64
    } else if size == 0b11 {
        ai.set_exclusive_monitors(addr, 8);
        ai.read64(addr, MemAccessStr::atomic_load()).unwrap() as u64
    } else {
        panic!();
    };
    ai.set_reg(args.get_rt(), val, false);
}
pub fn ldxr(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let size = args.get_ls_size();
    let addr = ai.get_reg(args.get_rn(), false);
    let val = if size == 0b10 {
        ai.set_exclusive_monitors(addr, 4);
        ai.read32(addr, MemAccessStr::std_loadstore()).unwrap() as u64
    } else if size == 0b11 {
        ai.set_exclusive_monitors(addr, 8);
        ai.read64(addr, MemAccessStr::std_loadstore()).unwrap() as u64
    } else {
        panic!();
    };
    ai.set_reg(args.get_rt(), val, false);
}
pub fn stlr(ai: &mut Arm64Cpu, args: &ArmInstr) {
    if (args.insn >> 28) & 1 != 0 {
        panic!(); // lrcpc3
    }
    let size = args.get_ls_size();
    let addr = ai.get_reg(args.get_rn(), true);
    let val = ai.get_reg(args.get_rt(), false);
    if size == 0b10 {
        ai.write32(addr, val as u32, MemAccessStr::atomic_load()) as u64
    } else if size == 0b11 {
        ai.write64(addr, val as u64, MemAccessStr::atomic_load()) as u64
    } else {
        panic!();
    };
}
pub fn ldar(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let size = args.get_ls_size();
    let addr = ai.get_reg(args.get_rn(), true);
    let val = if size == 0b10 {
        ai.read32(addr, MemAccessStr::atomic_load()).unwrap() as u64
    } else if size == 0b11 {
        ai.read64(addr, MemAccessStr::atomic_load()).unwrap() as u64
    } else {
        panic!();
    };
    ai.set_reg(args.get_rt(), val, false);
}

pub fn stxr(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let addr = ai.get_reg(args.get_rn(), true);
    let val = ai.get_reg(args.get_rt(), false);
    let size = args.get_ls_size();
    // rs uses same fields as rm
    let rs = args.get_rm();
    let bits = if size == 0b10 {
        32
    } else {
        64
    };
    if ai.check_exclusive_monitors(addr, bits / 8) {
        if bits == 32 {
            ai.write32(addr, val as u32, MemAccessStr::std_loadstore()) as u64
        } else if bits == 64 {
            ai.write64(addr, val as u64, MemAccessStr::std_loadstore()) as u64
        } else {
            panic!();
        };
        ai.clear_exclusive_monitor();
        ai.set_reg(rs, 0, false);
    } else {
        ai.set_reg(rs, 1, false);
    }
}
pub fn stlxr(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let addr = ai.get_reg(args.get_rn(), true);
    let val = ai.get_reg(args.get_rt(), false);
    let size = args.get_ls_size();
    // rs uses same fields as rm
    let rs = args.get_rm();
    let bits = if size == 0b10 {
        32
    } else {
        64
    };
    if ai.check_exclusive_monitors(addr, bits / 8) {
        if bits == 32 {
            ai.write32(addr, val as u32, MemAccessStr::atomic_load()) as u64
        } else if bits == 64 {
            ai.write64(addr, val as u64, MemAccessStr::atomic_load()) as u64
        } else {
            panic!();
        };
        ai.clear_exclusive_monitor();
        ai.set_reg(rs, 0, false);
    } else {
        ai.set_reg(rs, 1, false);
    }
}
pub fn ld1_advsimd_mult(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let postidx = ((args.insn >> 23) & 1) != 0;
    let opcode = (args.insn >> 12) & 0b1111;
    let size = if opcode == 0b0111 {
        1
    } else if opcode == 0b1010 {
        2
    } else if opcode == 0b0110 {
        3
    } else if opcode == 0b0010 {
        4
    } else {
        panic!()
    };
    load_store_multi_gen(ai, args, LoadStoreSIMDMultiTypes::Ld1(size), postidx);
}
pub fn st1_advsimd_mult(ai: &mut Arm64Cpu, args: &ArmInstr) {
    let postidx = ((args.insn >> 23) & 1) != 0;
    let opcode = (args.insn >> 12) & 0b1111;
    let size = if opcode == 0b0111 {
        1
    } else if opcode == 0b1010 {
        2
    } else if opcode == 0b0110 {
        3
    } else if opcode == 0b0010 {
        4
    } else {
        panic!()
    };
    load_store_multi_gen(ai, args, LoadStoreSIMDMultiTypes::St1(size), postidx);
}