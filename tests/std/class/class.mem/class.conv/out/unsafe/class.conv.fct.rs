extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_into(i64, to_i64, mut_ptr)]
pub struct p2_non_const_conversion_function_Gauge {
    pub value: i64,
}
impl p2_non_const_conversion_function_Gauge {
    pub unsafe fn to_i64(&mut self) -> i64 {
        return (*self.value.prefix_inc());
    }
}
pub unsafe fn test_0() {
    let mut gauge: p2_non_const_conversion_function_Gauge =
        p2_non_const_conversion_function_Gauge { value: 20_i64 };
    let mut converted: i64 =
        (unsafe { p2_non_const_conversion_function_Gauge::to_i64(&mut gauge) });
    assert!(((converted) == (21_i64)));
    assert!(((gauge.value) == (21_i64)));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test_0() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
