extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut twice: lambda_0 = (lambda_0 {});
    assert!(((unsafe { lambda_0::operator_call_i32__int_const(4,) }) == (8)));
    assert!(((unsafe { lambda_0::operator_call_f64__double_const(1.5E+0,) }) == (3.0E+0)));
    let mut base: i32 = 10;
    let mut add_base: lambda_1 = (lambda_1 { base: base });
    assert!(((unsafe { lambda_1::operator_call_i32__int_const(&add_base, 5,) }) == (15)));
    assert!(
        ((unsafe { lambda_1::operator_call_f64__double_const(&add_base, 2.5E+0,) }) == (1.25E+1))
    );
    let mut total: i32 = 0;
    let mut accumulate: lambda_2 = (lambda_2 { total: &mut total });
    (unsafe { lambda_2::operator_call_i32_i32__int_int_const(&accumulate, 2, 3) });
    (unsafe {
        lambda_2::operator_call_u32_u32__unsigned_int_unsigned_int_const(&accumulate, 4_u32, 5_u32)
    });
    assert!(((total) == (26)));
    let mut sub: lambda_3 = (lambda_3 {});
    assert!(((unsafe { lambda_3::operator_call_i32_i32__int_const(9, 4,) }) == (5)));
    assert!(
        ((unsafe { lambda_3::operator_call_f64_f64__double_const(2.5E+0, 1.0E+0,) }) == (1.5E+0))
    );
    let mut mixed: lambda_4 = (lambda_4 { base: base });
    assert!(((unsafe { lambda_4::operator_call_i32_i32__int_int_const(&mixed, 2, 3,) }) == (16)));
    assert!(
        ((unsafe { lambda_4::operator_call_i32_f64__int_double_const(&mixed, 2, 5.0E-1,) })
            == (1.1E+1))
    );
    let mut cast_to: lambda_5 = (lambda_5 {});
    assert!(((unsafe { lambda_5::operator_call_i32__int_const(5,) }) == (2)));
    assert!(((unsafe { lambda_5::operator_call_i32__double_const(5,) }) == (2.5E+0)));
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_0 {}
impl lambda_0 {
    pub unsafe fn operator_call_i32__int_const(mut x: i32) -> i32 {
        return ((x) + (x));
    }
    pub unsafe fn operator_call_f64__double_const(mut x: f64) -> f64 {
        return ((x) + (x));
    }
}
impl Callable1<i32, i32> for lambda_0 {
    fn call(&self, a1: i32) -> i32 {
        unsafe { lambda_0::operator_call_i32__int_const(a1) }
    }
}
impl Callable1<f64, f64> for lambda_0 {
    fn call(&self, a1: f64) -> f64 {
        unsafe { lambda_0::operator_call_f64__double_const(a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_1 {
    base: i32,
}
impl lambda_1 {
    pub unsafe fn operator_call_i32__int_const(&self, mut x: i32) -> i32 {
        return ((x) + (self.base));
    }
    pub unsafe fn operator_call_f64__double_const(&self, mut x: f64) -> f64 {
        return ((x) + (self.base as f64));
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
pub struct lambda_2 {
    total: *mut i32,
}
impl lambda_2 {
    pub unsafe fn operator_call_i32_i32__int_int_const(&self, mut x: i32, mut y: i32) {
        (*self.total) += ((x) * (y));
    }
    pub unsafe fn operator_call_u32_u32__unsigned_int_unsigned_int_const(
        &self,
        mut x: u32,
        mut y: u32,
    ) {
        (*self.total) = (((*self.total) as u32).wrapping_add((x).wrapping_mul(y))) as i32;
    }
}
impl Callable2<i32, i32, ()> for lambda_2 {
    fn call(&self, a1: i32, a2: i32) -> () {
        unsafe { lambda_2::operator_call_i32_i32__int_int_const(self, a1, a2) }
    }
}
impl Callable2<u32, u32, ()> for lambda_2 {
    fn call(&self, a1: u32, a2: u32) -> () {
        unsafe { lambda_2::operator_call_u32_u32__unsigned_int_unsigned_int_const(self, a1, a2) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_3 {}
impl lambda_3 {
    pub unsafe fn operator_call_i32_i32__int_const(mut x: i32, mut y: i32) -> i32 {
        return ((x) - (y));
    }
    pub unsafe fn operator_call_f64_f64__double_const(mut x: f64, mut y: f64) -> f64 {
        return ((x) - (y));
    }
}
impl Callable2<i32, i32, i32> for lambda_3 {
    fn call(&self, a1: i32, a2: i32) -> i32 {
        unsafe { lambda_3::operator_call_i32_i32__int_const(a1, a2) }
    }
}
impl Callable2<f64, f64, f64> for lambda_3 {
    fn call(&self, a1: f64, a2: f64) -> f64 {
        unsafe { lambda_3::operator_call_f64_f64__double_const(a1, a2) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_4 {
    base: i32,
}
impl lambda_4 {
    pub unsafe fn operator_call_i32_i32__int_int_const(&self, mut x: i32, mut y: i32) -> i32 {
        return (((x) * (y)) + (self.base));
    }
    pub unsafe fn operator_call_i32_f64__int_double_const(&self, mut x: i32, mut y: f64) -> f64 {
        return (((x as f64) * (y)) + (self.base as f64));
    }
}
impl Callable2<i32, i32, i32> for lambda_4 {
    fn call(&self, a1: i32, a2: i32) -> i32 {
        unsafe { lambda_4::operator_call_i32_i32__int_int_const(self, a1, a2) }
    }
}
impl Callable2<i32, f64, f64> for lambda_4 {
    fn call(&self, a1: i32, a2: f64) -> f64 {
        unsafe { lambda_4::operator_call_i32_f64__int_double_const(self, a1, a2) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct lambda_5 {}
impl lambda_5 {
    pub unsafe fn operator_call_i32__int_const(mut x: i32) -> i32 {
        return ((x as i32) / (2));
    }
    pub unsafe fn operator_call_i32__double_const(mut x: i32) -> f64 {
        return ((x as f64) / (2_f64));
    }
}
pub unsafe fn __cpp2rust_init_globals() {}
