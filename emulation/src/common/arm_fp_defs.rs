use simple_soft_float::{FPState, StatusFlags};

#[derive(Copy, Clone, Default)]
pub struct FPSR {
    pub ioc: bool, // invalid
    pub dzc: bool, // divide by 0
    pub ofc: bool, // overflow
    pub ufc: bool, // underflow
    pub ixc: bool, // inexact
    pub idc: bool, // input denorm
}
#[derive(Copy, Clone, Default)]
pub struct Flags {
    pub n: bool,
    pub z: bool,
    pub c: bool,
    pub v: bool,

}
pub fn cond_holds(cond: u8, flags: Flags) -> bool {
    let higher3 = cond >> 1;
    let cmet = match higher3 {
        0 => flags.z,
        1 => flags.c,
        2 => flags.n,
        3 => flags.v,
        4 => flags.c && !flags.z,
        5 => flags.n == flags.v,
        6 => (flags.n == flags.v) && !flags.z,
        7 => { return true; }, // shortcut
        _ => unreachable!()
    };
    if (cond & 1) != 0 {
        !cmet
    } else {
        cmet
    }
}
pub fn apply_fpstate(state: &mut FPSR, val: &FPState) {
    state.dzc = val.status_flags.contains(StatusFlags::DIVISION_BY_ZERO);
    state.ioc = val.status_flags.contains(StatusFlags::INVALID_OPERATION);
    state.ixc = val.status_flags.contains(StatusFlags::INEXACT);
    state.ufc = val.status_flags.contains(StatusFlags::UNDERFLOW);
    state.ofc = val.status_flags.contains(StatusFlags::OVERFLOW);

}