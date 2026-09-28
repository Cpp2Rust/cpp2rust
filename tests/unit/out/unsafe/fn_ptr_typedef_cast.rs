extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn call_with_ulong_0(mut g: Option<unsafe fn(u64) -> u64>) -> u64 {
    return (unsafe { (g).unwrap()(3_u64) }).wrapping_add(1_u64);
}
pub unsafe fn same_type_1(mut a: u64) -> u64 {
    return a;
}
pub unsafe fn via_size_t_param_2(mut b: usize) -> u64 {
    return (b as u64);
}
pub unsafe fn via_size_t_return_3(mut b: u64) -> usize {
    return (b as usize);
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct pair {
    pub a: i32,
    pub b: i32,
}
pub unsafe fn pair_scaled_4(mut p: pair, mut n: usize) -> u64 {
    return (((p.a as usize).wrapping_mul(n)).wrapping_add((p.b as usize)) as u64);
}
pub unsafe fn make_pair_5(mut n: usize) -> pair {
    let mut p: pair = pair {
        a: (n as i32),
        b: ((n as i32) * (2)),
    };
    return p;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((((unsafe { call_with_ulong_0(Some(same_type_1),) }) == (4_u64)) as i32) != 0));
    assert!(
        ((((unsafe {
            call_with_ulong_0(std::mem::transmute::<
                Option<unsafe fn(usize) -> u64>,
                Option<unsafe fn(u64) -> u64>,
            >(Some(via_size_t_param_2)))
        }) == (4_u64)) as i32)
            != 0)
    );
    assert!(
        ((((unsafe {
            call_with_ulong_0(std::mem::transmute::<
                Option<unsafe fn(u64) -> usize>,
                Option<unsafe fn(u64) -> u64>,
            >(Some(via_size_t_return_3)))
        }) == (4_u64)) as i32)
            != 0)
    );
    let mut original: Option<unsafe fn(usize) -> u64> = Some(via_size_t_param_2);
    let mut adapted: Option<unsafe fn(u64) -> u64> = std::mem::transmute::<
        Option<unsafe fn(usize) -> u64>,
        Option<unsafe fn(u64) -> u64>,
    >(original);
    let mut back: Option<unsafe fn(usize) -> u64> = std::mem::transmute::<
        Option<unsafe fn(u64) -> u64>,
        Option<unsafe fn(usize) -> u64>,
    >(adapted);
    assert!(((((back) == (original)) as i32) != 0));
    assert!(((((unsafe { (back).unwrap()(5_usize,) }) == (5_u64)) as i32) != 0));
    let mut scaled: Option<unsafe fn(pair, u64) -> u64> = std::mem::transmute::<
        Option<unsafe fn(pair, usize) -> u64>,
        Option<unsafe fn(pair, u64) -> u64>,
    >(Some(pair_scaled_4));
    let mut p: pair = pair { a: 3, b: 4 };
    assert!(((((unsafe { (scaled).unwrap()(p, 10_u64,) }) == (34_u64)) as i32) != 0));
    let mut make: Option<unsafe fn(u64) -> pair> = std::mem::transmute::<
        Option<unsafe fn(usize) -> pair>,
        Option<unsafe fn(u64) -> pair>,
    >(Some(make_pair_5));
    let mut q: pair = (unsafe { (make).unwrap()(5_u64) });
    assert!(((((q.a) == (5)) as i32) != 0));
    assert!(((((q.b) == (10)) as i32) != 0));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
