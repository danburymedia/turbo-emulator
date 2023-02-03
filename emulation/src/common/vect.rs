use std::mem;

#[derive(Copy, Clone)]
pub struct VectInfo {
    pub lane_count: usize, // amt of values
    pub elem_size: usize, // size of elements (in bits)
}
impl VectInfo {
    pub fn new_64bits(esize: usize) -> VectInfo {
        VectInfo {
            lane_count: 64 / esize,
            elem_size: esize
        }
    }
    pub fn new_128bits(esize: usize) -> VectInfo {
        VectInfo {
            lane_count: 128 / esize,
            elem_size: esize
        }
    }
    pub fn mask(&self) -> u64 {
        // mask of the bits elem can handle
        ((1u128 << (self.elem_size as u64)) - 1) as u64
    }
    pub fn half_width(&self) -> VectInfo {
        VectInfo {
            lane_count: self.lane_count,
            elem_size: self.elem_size / 2
        }
    }
    /// double the width, but half lane count, so total bits is same
    pub fn double_width_same_total(&self) -> VectInfo {
        VectInfo {
            lane_count: self.lane_count / 2,
            elem_size: self.elem_size * 2
        }
    }
    pub fn elem_use_whole_reg(&self) -> VectInfo {
        let mut nvi = self.clone();
        if nvi.elem_size == 8 {
            nvi.lane_count = 16;
        } else if nvi.elem_size == 16 {
            nvi.lane_count = 8;
        } else if nvi.elem_size == 32 {
            nvi.lane_count = 4;
        } else if nvi.elem_size == 64 {
            nvi.lane_count = 2;
        } else {
            panic!();
        }
        nvi
    }
    pub fn get_max_signed(&self) -> i64 {
        match self.elem_size {
            8 => i8::MAX as i64,
            16 => i16::MAX as i64,
            32 => i32::MAX as i64,
            64 => i64::MAX as i64,
            _ => panic!()
        }
    }
    pub fn get_max(&self) -> u64 {
        match self.elem_size {
            8 => u8::MAX as u64,
            16 => u16::MAX as u64,
            32 => u32::MAX as u64,
            64 => u64::MAX as u64,
            _ => panic!()
        }
    }
    pub fn get_min(&self) -> u64 {
        match self.elem_size {
            8 => u8::MIN as u64,
            16 => u16::MIN as u64,
            32 => u32::MIN as u64,
            64 => u64::MIN as u64,
            _ => panic!()
        }
    }
    pub fn get_min_signed(&self) -> i64 {
        match self.elem_size {
            8 => i8::MIN as i64,
            16 => i16::MIN as i64,
            32 => i32::MIN as i64,
            64 => i64::MIN as i64,
            _ => panic!()
        }
    }
}
pub fn set_elem_vect_fixed(vecval: u128, val: u64, idx: usize, vinfo: VectInfo) -> u128 {
    match vinfo.elem_size {
        8 => set_elem_vect(vecval, val as u8, idx),
        16 => set_elem_vect(vecval, val as u16, idx),
        32 => set_elem_vect(vecval, val as u32, idx),
        64 => set_elem_vect(vecval, val, idx),
        _ => panic!()
    }
}
pub fn get_elem_vect_fixed(vecval: u128, idx: usize, vinfo: VectInfo) -> u64 {
    match vinfo.elem_size {
        8 => get_elem_vect::<u8>(vecval, idx) as u64,
        16 => get_elem_vect::<u16>(vecval, idx) as u64,
        32 => get_elem_vect::<u32>(vecval, idx) as u64,
        64 => get_elem_vect::<u64>(vecval, idx) as u64,
        _ => panic!()
    }
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