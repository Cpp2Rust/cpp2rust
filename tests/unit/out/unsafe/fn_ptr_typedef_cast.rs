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
            call_with_ulong_0(Some(via_size_t_param_2) as Option<unsafe fn(u64) -> u64>)
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
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
