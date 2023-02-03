
pub fn round_up(val: u64, align: u64) -> u64 {
    assert!(align.is_power_of_two());
    let mask = align - 1;
    (val + mask) & !mask
}
pub fn round_down(val: u64, align: u64) -> u64 {
    assert!(align.is_power_of_two());
    let mask = align - 1;
    (val) & !mask
}