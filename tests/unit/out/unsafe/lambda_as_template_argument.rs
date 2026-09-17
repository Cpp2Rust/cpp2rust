extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn apply_0(mut fn_: lambda_1, mut x: i32) -> i32 {
    return (unsafe { lambda_1::operator_call(&fn_, x) });
}
pub unsafe fn apply_2(mut fn_: lambda_3, mut x: i32) -> i32 {
    return (unsafe { lambda_3::operator_call(&fn_, x) });
}
pub unsafe fn apply_4(mut fn_: lambda_5, mut x: i32) -> i32 {
    return (unsafe { lambda_5::operator_call(x) });
}
pub unsafe fn apply_twice_6(mut fn_: lambda_3, mut x: i32) -> i32 {
    return (unsafe {
        let _x: i32 = (unsafe { lambda_3::operator_call(&fn_, x) });
        lambda_3::operator_call(&fn_, _x)
    });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut base: i32 = 10;
    let mut add_base: lambda_1 = (lambda_1 { base: &mut base });
    assert!(((unsafe { apply_0(add_base, 5,) }) == (15)));
    base = 100;
    assert!(((unsafe { apply_0(add_base, 5,) }) == (105)));
    let mut factor: i32 = 3;
    let mut scale: lambda_3 = (lambda_3 { factor: factor });
    assert!(((unsafe { apply_2(scale, 4,) }) == (12)));
    assert!(((unsafe { apply_twice_6(scale, 4,) }) == (36)));
    assert!(((unsafe { apply_4((lambda_5 {}), 9,) }) == (-9_i32)));
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_1 {
    base: *mut i32,
}
impl lambda_1 {
    pub unsafe fn operator_call(&self, mut x: i32) -> i32 {
        return ((x) + (*self.base));
    }
}
impl Callable1<i32, i32> for lambda_1 {
    fn call(&self, a1: i32) -> i32 {
        unsafe { lambda_1::operator_call(self, a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_3 {
    factor: i32,
}
impl lambda_3 {
    pub unsafe fn operator_call(&self, mut x: i32) -> i32 {
        return ((x) * (self.factor));
    }
}
impl Callable1<i32, i32> for lambda_3 {
    fn call(&self, a1: i32) -> i32 {
        unsafe { lambda_3::operator_call(self, a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_5 {}
impl lambda_5 {
    pub unsafe fn operator_call(mut x: i32) -> i32 {
        return -x;
    }
}
impl Callable1<i32, i32> for lambda_5 {
    fn call(&self, a1: i32) -> i32 {
        unsafe { lambda_5::operator_call(a1) }
    }
}
impl lambda_5 {
    pub fn to_free_function(&self) -> Option<unsafe fn(i32) -> i32> {
        Some(lambda_5::operator_call)
    }
}
pub unsafe fn __cpp2rust_init_globals() {}
