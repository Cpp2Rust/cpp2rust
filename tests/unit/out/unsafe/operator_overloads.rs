extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct S {
    pub v: i32,
}
impl S {
    pub unsafe fn operator_eq_i32_const(&self, mut o: i32) -> i32 {
        return if ((self.v) == (o)) { 1 } else { 0 };
    }
    pub unsafe fn operator_eq_i64_const(&self, mut o: i64) -> i32 {
        return if ((self.v as i64) == (o)) { 2 } else { 0 };
    }
    pub unsafe fn operator_eq_f64_const(&self, mut o: f64) -> i32 {
        return if ((self.v as f64) == (o)) { 3 } else { 0 };
    }
    pub unsafe fn operator_add(&self, o: *const S) -> i32 {
        return ((self.v) + ((*o).v));
    }
    pub unsafe fn operator_sub(&self, mut o: S) -> i32 {
        return ((self.v) - (o.v));
    }
    pub unsafe fn operator_mul_pconstS_const(&self, o: *const S) -> i32 {
        return ((self.v) * ((*o).v));
    }
    pub unsafe fn operator_mul_i32_const(&self, mut o: i32) -> i32 {
        return (((self.v) * (o)) + (1));
    }
}
pub unsafe fn operator_div_0(a: *const S, b: *const S) -> i32 {
    return (((*a).v) / ((*b).v));
}
pub unsafe fn operator_div_1(mut a: S, mut b: i32) -> i32 {
    return (((a.v) / (b)) + (1));
}
pub unsafe fn operator_rem_2(mut a: S, mut b: S) -> i32 {
    return ((a.v) % (b.v));
}
pub unsafe fn operator_rem_3(a: *const S, mut b: i32) -> i32 {
    return ((((*a).v) % (b)) + (1));
}
pub unsafe fn operator_eq_4(mut a: i32, mut b: S) -> i32 {
    return if ((a) == (b.v)) { 4 } else { 0 };
}
pub unsafe fn operator_eq_5(mut a: i64, b: *const S) -> i32 {
    return if ((a) == ((*b).v as i64)) { 5 } else { 0 };
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = S { v: 6 };
    let mut t: S = S { v: 4 };
    assert!(((unsafe { S::operator_eq_i32_const(&s, 6,) }) == (1)));
    assert!(((unsafe { S::operator_eq_i64_const(&s, 6_i64,) }) == (2)));
    assert!(((unsafe { S::operator_eq_f64_const(&s, 6.0E+0,) }) == (3)));
    assert!(((unsafe { S::operator_eq_i32_const(&s, 7,) }) == (0)));
    assert!(((unsafe { S::operator_add(&s, &t as *const S,) }) == (10)));
    assert!(((unsafe { S::operator_sub(&s, t,) }) == (2)));
    assert!(((unsafe { S::operator_mul_pconstS_const(&s, &t as *const S,) }) == (24)));
    assert!(((unsafe { S::operator_mul_i32_const(&s, 2,) }) == (13)));
    assert!(
        ((unsafe {
            let _a: *const S = &s as *const S;
            operator_div_0(_a, &t as *const S)
        }) == (1))
    );
    assert!(
        ((unsafe {
            let _a: S = s;
            operator_div_1(_a, 4)
        }) == (2))
    );
    assert!(
        ((unsafe {
            let _a: S = s;
            operator_rem_2(_a, t)
        }) == (2))
    );
    assert!(
        ((unsafe {
            let _a: *const S = &s as *const S;
            operator_rem_3(_a, 4)
        }) == (3))
    );
    assert!(((unsafe { operator_eq_4(6, s,) }) == (4)));
    assert!(((unsafe { operator_eq_5(6_i64, &s as *const S,) }) == (5)));
    return 0;
}
