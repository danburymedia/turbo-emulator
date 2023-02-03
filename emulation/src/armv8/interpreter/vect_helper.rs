use std::mem;
use crate::common::vect::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SaturationState {
    Pos,
    Neg
}
#[derive(Copy, Clone, Default, Debug)]
pub struct ElemInfo {
    unsigned_sat: Option<SaturationState>,
    signed_sat: Option<SaturationState>,
    rounding: bool

}
#[derive(Copy, Clone, Default, Debug)]
pub struct VectorReg {
    pub vect: u128,
    pub einfo: [ElemInfo; 16]
}
pub fn get_elem_vect<T: num::PrimInt>(vecval: u128, idx: usize) -> T {

    let size = mem::size_of::<T>();
    if !size.is_power_of_two() {
        panic!(); // has to eb power of two
    }
    let sizebits = size * 8;
    let shr_amt = sizebits * idx;
    let mask = (1 << sizebits) - 1;
    let newval = (vecval >> shr_amt) & mask;
    T::from(newval).unwrap()
}
pub fn set_elem_vect<T: num::PrimInt>(vecval: u128, val: T, idx: usize) -> u128 {

    let size = mem::size_of::<T>();
    let mut newvecval = vecval;
    if !size.is_power_of_two() {
        panic!(); // has to be power of two
    }
    let sizebits = size * 8;
    let shr_amt = sizebits * idx;
    let mask = (1 << sizebits) - 1;
    newvecval &= !(mask << shr_amt);
    newvecval |= ((val.to_u128().unwrap()) << shr_amt);
    newvecval
}
impl VectorReg {
    pub fn set_rounding(&mut self, idx: usize, val: bool) {
        self.einfo[idx].rounding = val;
    }
    pub fn set_unsigned_sat(&mut self, idx: usize, val: bool) {
        self.einfo[idx].unsigned_sat = if val {
            Some(SaturationState::Pos)
        } else {
            Some(SaturationState::Neg)
        };
    }
    pub fn set_signed_sat(&mut self, idx: usize, val: bool) {
        self.einfo[idx].signed_sat = if val {
            Some(SaturationState::Pos)
        } else {
            Some(SaturationState::Neg)
        };
    }
    pub fn clear_vect(&mut self) {
        self.vect = 0;
        self.einfo = [ElemInfo::default(); 16];
    }
    pub fn clear_unused(&mut self, vinfo: VectInfo) {
        let bits = vinfo.elem_size * vinfo.lane_count;
        assert_eq!(bits % 8, 0);
        let byteval = bits / 8;
        let byteinfo = VectInfo {
            lane_count: 16,
            elem_size: 8
        };
        for i in byteval..16 {
            self.set_elem_fixed(0, i, byteinfo);
        }
    }
    pub fn clear_einfo(&mut self) {
        self.einfo = [ElemInfo::default(); 16];
    }
    pub fn set_elem_fixed(&mut self, val: u64, idx: usize, vinfo: VectInfo) {
        self.vect = match vinfo.elem_size {
            8 => set_elem_vect(self.vect , val as u8, idx),
            16 => set_elem_vect(self.vect , val as u16, idx),
            32 => set_elem_vect(self.vect , val as u32, idx),
            64 => set_elem_vect(self.vect , val, idx),
            _ => panic!()
        }
    }
    pub fn set_elem_signed_fixed(&mut self, val: i64, idx: usize, vinfo: VectInfo) {
        self.vect = match vinfo.elem_size {
            8 => set_elem_vect(self.vect , val as i8, idx),
            16 => set_elem_vect(self.vect , val as i16, idx),
            32 => set_elem_vect(self.vect , val as i32, idx),
            64 => set_elem_vect(self.vect , val, idx),
            _ => panic!()
        }
    }
    pub fn get_elem_fixed(&self, idx: usize, vinfo: VectInfo) -> u64 {
        match vinfo.elem_size {
            8 => get_elem_vect::<u8>(self.vect, idx) as u64,
            16 => get_elem_vect::<u16>(self.vect, idx) as u64,
            32 => get_elem_vect::<u32>(self.vect, idx) as u64,
            64 => get_elem_vect::<u64>(self.vect, idx) as u64,
            _ => panic!()
        }
    }
    pub fn get_elem_fixed_justified_left(&self, idx: usize, vinfo: VectInfo) -> u64 {
        // to check sat calc
        self.get_elem_fixed(idx, vinfo) << ((64 - vinfo.elem_size) as u64)
    }
    pub fn get_leading_sign(&self, idx: usize, vinfo: VectInfo) -> i64 {
        let elem = match vinfo.elem_size {
            8 => get_elem_vect::<i8>(self.vect, idx) as i64,
            16 => get_elem_vect::<i16>(self.vect, idx) as i64,
            32 => get_elem_vect::<i32>(self.vect, idx) as i64,
            64 => get_elem_vect::<i64>(self.vect, idx) as i64,
            _ => panic!()
        };
        if elem < 0 {
            return match vinfo.elem_size {
                8 => (elem as i8).leading_ones(),
                16 => (elem as i16).leading_ones(),
                32 => (elem as i32).leading_ones(),
                64 => (elem as i64).leading_ones(),
                _ => panic!()
            } as i64;
        } else {
            return match vinfo.elem_size {
                8 => (elem as i8).leading_zeros(),
                16 => (elem as i16).leading_zeros(),
                32 => (elem as i32).leading_zeros(),
                64 => (elem as i64).leading_zeros(),
                _ => panic!()
            } as i64;
        }
    }
    pub fn get_leading_zeros(&self, idx: usize, vinfo: VectInfo) -> u32 {
        match vinfo.elem_size {
            8 => get_elem_vect::<i8>(self.vect, idx).leading_zeros(),
            16 => get_elem_vect::<i16>(self.vect, idx).leading_zeros(),
            32 => get_elem_vect::<i32>(self.vect, idx).leading_zeros(),
            64 => get_elem_vect::<i64>(self.vect, idx).leading_zeros(),
            _ => panic!()
        }
    }
    pub fn get_elem_signed_fixed(&self, idx: usize, vinfo: VectInfo) -> i64 {
        match vinfo.elem_size {
            8 => get_elem_vect::<i8>(self.vect, idx) as i64,
            16 => get_elem_vect::<i16>(self.vect, idx) as i64,
            32 => get_elem_vect::<i32>(self.vect, idx) as i64,
            64 => get_elem_vect::<i64>(self.vect, idx) as i64,
            _ => panic!()
        }
    }
    pub fn set_from_array(&mut self, arr: &[u64], vinfo: VectInfo) {
        self.vect = 0;
        self.einfo = [Default::default(); 16];
        for i in 0..arr.len() {
            self.set_elem_fixed(arr[i], i, vinfo);
        }
    }
    pub fn signed_saturate(&mut self, vinfo: VectInfo) {
        for i in 0..vinfo.lane_count {
            if Some(SaturationState::Pos) == self.einfo[i].signed_sat {
                self.set_elem_signed_fixed(vinfo.get_max_signed(), i, vinfo);
            } else if Some(SaturationState::Neg) == self.einfo[i].signed_sat {
                self.set_elem_signed_fixed(vinfo.get_min_signed(), i, vinfo);
            }
        }
    }
    pub fn unsigned_saturate(&mut self, vinfo: VectInfo) {
        for i in 0..vinfo.lane_count {
            if Some(SaturationState::Pos) == self.einfo[i].unsigned_sat {
                self.set_elem_fixed(vinfo.get_max(), i, vinfo);
            } else if Some(SaturationState::Neg) == self.einfo[i].unsigned_sat {
                self.set_elem_fixed(0, i, vinfo);
            }
        }
    }
}
