use std::cmp::Ordering;
use simple_soft_float::{F32, F64, Float, FloatBitsType, FloatClass, FloatTraits, FPState, RoundingMode, StatusFlags};
use crate::riscv::interpreter::consts::{CSR_FCSR_ADDRESS, EXT_F, EXT_ZFINX};
use crate::riscv::interpreter::main::{ExtensionSearchMode, RiscvInt};
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum FLen {
    F32,
    F64,
    F128,
}
pub type FRegT = u64;
impl FLen {
    pub fn len(&self) -> usize {
        match self {
            FLen::F32 => 32,
            FLen::F64 => 64,
            FLen::F128 => 128,
        }
    }
    pub fn size(&self) -> usize {
        self.len() >> 3
    }
    pub fn mask(&self) -> FRegT {
        match self {
            FLen::F32 => ((1 as FRegT) << (self.len() as FRegT)) - 1,
            FLen::F64 => u64::max_value() as FRegT,
            FLen::F128 => panic!() // not supported yet.
        }
    }
    pub fn padding(&self, v: u64, flen: FLen) -> FRegT {
        self.mask()
            & if flen.len() < self.len() {
            v & flen.mask() | self.mask() & !flen.mask()
        } else {
            v
        }
    }
    pub fn boxed(&self, v: FRegT, flen: FLen) -> FRegT {
        flen.mask()
            & if flen.len() < self.len() {
            if ((v | (-1i128 as FRegT) & !self.mask()) | flen.mask()) == -1i128 as FRegT {
                // if the distance from the actual lessened value to the end of the allocated variable si all 1s,
                // everything is fine
                v
            } else {
                match flen {
                    FLen::F32 => *F32::quiet_nan().bits() as FRegT,
                    FLen::F64 => *F64::quiet_nan().bits() as FRegT,
                    _ => unreachable!(),
                }
            }
        } else {
            v
        }
    }
}

fn get_system_flen(ri: &RiscvInt) -> FLen {
    FLen::F64 // for now
}
pub fn fps_2_fflags(ri: &mut RiscvInt, fp: FPState) {
    let mut statusbits: u64 = 0;
    // nx - 0
    // uf - 1
    // of - 2
    // dz - 3
    // nv - 4
    if fp.status_flags.contains(StatusFlags::INEXACT) {
        statusbits |= 1;
    }
    if fp.status_flags.contains(StatusFlags::UNDERFLOW) {
        statusbits |= 0b10;
    }
    if fp.status_flags.contains(StatusFlags::OVERFLOW) {
        statusbits |= 0b100;
    }
    if fp.status_flags.contains(StatusFlags::DIVISION_BY_ZERO) {
        statusbits |= 0b1000;
    }
    if fp.status_flags.contains(StatusFlags::INVALID_OPERATION) {
        statusbits |= 0b10000;
    }
    let exist = ri.csr[CSR_FCSR_ADDRESS as usize] & !0x1f;
    ri.csr[CSR_FCSR_ADDRESS as usize] =  exist | statusbits;
}
pub fn insn_2_rm_with_csr(ri: &RiscvInt, bits: u32) -> Option<RoundingMode> {
    match bits {
        7 => {
            let rm = (ri.csr[CSR_FCSR_ADDRESS as usize]) >> 5 & 0b111;
            insn_2_rm(rm as u32)
        },
        _ => insn_2_rm(bits)
    }
}
pub fn insn_2_rm(bits: u32) -> Option<RoundingMode> {
    match bits {
        0 => Some(RoundingMode::TiesToEven),
        1 => Some(RoundingMode::TowardZero),
        2 => Some(RoundingMode::TowardNegative),
        3 => Some(RoundingMode::TowardPositive),
        4 => Some(RoundingMode::TiesToAway),
        7 => panic!("wants csr val"),
        _ => None, // for now. Todo: get state if requested
    }
}
pub fn read_float32_raw(ri: &mut RiscvInt, idx: usize)  -> u32 {
    ri.fregs[idx] as u32

}
pub fn read_float64_raw(ri: &mut RiscvInt, idx: usize)  -> u64 {
    ri.fregs[idx] as u64

}
pub fn read_float32(ri: &mut RiscvInt, idx: usize)  -> u32 {
    get_system_flen(ri).boxed(ri.fregs[idx], FLen::F32) as u32
    // F32::from_bits(rawu32)

}
pub fn read_float64(ri: &mut RiscvInt, idx: usize)  -> u64 {
    get_system_flen(ri).boxed(ri.fregs[idx], FLen::F64) as u64


    // F64::from_bits(raw)

}
pub fn write_float32(ri: &mut RiscvInt, value: u32, idx: usize)  {
    // to bits before
    let write = value;
    ri.fregs[idx] = get_system_flen(ri).padding(write as u64, FLen::F32);

}
pub fn write_float64(ri: &mut RiscvInt, value: u64, idx: usize)  {
    // let write = value.into_bits();
    let write = value;
    ri.fregs[idx] = get_system_flen(ri).padding(write as u64, FLen::F64);


}
pub fn class_f32(rs1: F32) -> u64 {
    1 << match rs1.class() {
        FloatClass::NegativeInfinity => 0,
        FloatClass::NegativeNormal => 1,
        FloatClass::NegativeSubnormal => 2,
        FloatClass::NegativeZero => 3,
        FloatClass::PositiveZero => 4,
        FloatClass::PositiveSubnormal => 5,
        FloatClass::PositiveNormal => 6,
        FloatClass::PositiveInfinity => 7,
        FloatClass::SignalingNaN => 8,
        FloatClass::QuietNaN => 9,
    }
}
