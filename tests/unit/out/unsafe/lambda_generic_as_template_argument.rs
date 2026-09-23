extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn apply_int_0(mut fn_: lambda_1, mut x: i32) -> i32 {
    return (unsafe { lambda_1::operator_call_i32__int_const(&fn_, x) });
}
pub unsafe fn apply_int_2(mut fn_: lambda_3, mut x: i32) -> i32 {
    return (unsafe { lambda_3::operator_call_i32__int_const(x) });
}
pub unsafe fn apply_int_4(mut fn_: lambda_5, mut x: i32) -> i32 {
    return (unsafe { lambda_5::operator_call_i32__int_const(&fn_, x) });
}
pub unsafe fn apply_double_6(mut fn_: lambda_1, mut x: f64) -> f64 {
    return (unsafe { lambda_1::operator_call_f64__double_const(&fn_, x) });
}
pub unsafe fn apply_double_7(mut fn_: lambda_5, mut x: f64) -> f64 {
    return (unsafe { lambda_5::operator_call_f64__double_const(&fn_, x) });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut factor: i32 = 3;
    let mut scale: lambda_1 = (lambda_1 { factor: factor });
    assert!(((unsafe { apply_int_0(scale, 4,) }) == (12)));
    assert!(((unsafe { apply_double_6(scale, 1.5E+0,) }) == (4.5E+0)));
    assert!(((unsafe { apply_int_2((lambda_3 {}), 9,) }) == (-9_i32)));
    let mut offset: lambda_5 = (lambda_5 { factor: factor });
    assert!(((unsafe { apply_int_4(offset, 4,) }) == (7)));
    assert!(((unsafe { apply_double_7(offset, 1.5E+0,) }) == (4.5E+0)));
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_1 {
    factor: i32,
}
impl lambda_1 {
    pub unsafe fn operator_call_i32__int_const(&self, mut x: i32) -> i32 {
        return ((x) * (self.factor));
    }
    pub unsafe fn operator_call_f64__double_const(&self, mut x: f64) -> f64 {
        return ((x) * (self.factor as f64));
    }
}
impl Callable1<i32, i32> for lambda_1 {
    fn call(&self, a1: i32) -> i32 {
        unsafe { lambda_1::operator_call_i32__int_const(self, a1) }
    }
}
impl Callable1<f64, f64> for lambda_1 {
    fn call(&self, a1: f64) -> f64 {
        unsafe { lambda_1::operator_call_f64__double_const(self, a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_3 {}
impl lambda_3 {
    pub unsafe fn operator_call_i32__int_const(mut x: i32) -> i32 {
        return -x;
    }
}
impl Callable1<i32, i32> for lambda_3 {
    fn call(&self, a1: i32) -> i32 {
        unsafe { lambda_3::operator_call_i32__int_const(a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_5 {
    factor: i32,
}
impl lambda_5 {
    pub unsafe fn operator_call_i32__int_const(&self, mut x: i32) -> i32 {
        return ((x) + (self.factor));
    }
    pub unsafe fn operator_call_f64__double_const(&self, mut x: f64) -> f64 {
        return ((x) + (self.factor as f64));
    }
}
impl Callable1<i32, i32> for lambda_5 {
    fn call(&self, a1: i32) -> i32 {
        unsafe { lambda_5::operator_call_i32__int_const(self, a1) }
    }
}
impl Callable1<f64, f64> for lambda_5 {
    fn call(&self, a1: f64) -> f64 {
        unsafe { lambda_5::operator_call_f64__double_const(self, a1) }
    }
}
pub unsafe fn __cpp2rust_init_globals() {}
