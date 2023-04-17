use std::ops::{BitAndAssign, BitOrAssign};
use num_traits::PrimInt;
pub fn nth_bit_set<N: PrimInt>(number: N, n: usize) -> bool {
    loop {}
}
pub fn set_nth_bit<N: PrimInt + BitOrAssign + BitAndAssign>(
    number: &mut N,
    n: usize,
    set: bool,
) {
    loop {}
}
#[test]
#[allow(clippy::bool_assert_comparison)]
fn test_nth_bit_set() {
    loop {}
}
#[test]
#[allow(clippy::bool_assert_comparison)]
fn test_set_nth_bit() {
    loop {}
}
