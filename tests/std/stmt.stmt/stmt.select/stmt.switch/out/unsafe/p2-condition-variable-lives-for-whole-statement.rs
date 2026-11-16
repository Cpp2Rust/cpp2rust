extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut matching_case_runs_0: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut other_case_runs_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub static mut dtorCount_2: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, CppFromUnsafe, DestructorUnsafe, Default)]
#[cpp_into(i32, to_i32)]
pub struct ConvertsToInt {
    pub value: i32,
}
impl ConvertsToInt {
    pub unsafe fn to_i32(&self) -> i32 {
        return self.value;
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_2)).prefix_inc();
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    'switch: {
        match { (unsafe { ConvertsToInt::to_i32(&selector) }) } {
            __v if __v == 2 => {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut matching_case_runs_0)).prefix_inc();
                assert!(((selector.value) == (2)));
                assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_2)) == (0)));
                break 'switch;
            }
            _ => {
                (*std::cell::LazyCell::force_mut(&mut *&raw mut other_case_runs_1)).prefix_inc();
                break 'switch;
            }
        }
    };
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut matching_case_runs_0)) == (1)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut other_case_runs_1)) == (0)));
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut dtorCount_2)) == (1)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const matching_case_runs_0);
    std::cell::LazyCell::force(&*&raw const other_case_runs_1);
    std::cell::LazyCell::force(&*&raw const dtorCount_2);
}
