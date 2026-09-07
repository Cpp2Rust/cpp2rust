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
pub struct Static {}
impl Static {
    pub unsafe fn operator_call(mut a: i32, mut b: i32) -> i32 {
        return ((a) * (b));
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct S {
    pub v: i32,
}
impl S {
    pub unsafe fn operator_call_const(&self) -> i32 {
        return self.v;
    }
    pub unsafe fn operator_call_i32_const(&self, mut a: i32) -> i32 {
        return ((self.v) + (a));
    }
    pub unsafe fn operator_call_i32_i32_const(&self, mut a: i32, mut b: i32) -> i32 {
        return (((self.v) + (a)) + (b));
    }
    pub unsafe fn operator_comma(&self, o: *const S) -> S {
        return S {
            v: (((self.v) * (10)) + ((*o).v)),
        };
    }
    pub unsafe fn operator_int(&self) -> i32 {
        return self.v;
    }
    pub unsafe fn operator__Bool(&self) -> bool {
        return ((self.v) != (0));
    }
}
pub fn main() {
    unsafe {
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = S { v: 3 };
    let mut t: S = S { v: 4 };
    assert!(((unsafe { S::operator_call_const(&s,) }) == (3)));
    assert!(((unsafe { S::operator_call_i32_const(&s, 1,) }) == (4)));
    assert!(((unsafe { S::operator_call_i32_i32_const(&s, 1, 2,) }) == (6)));
    assert!((((unsafe { S::operator_comma(&s, &t as *const S,) }).v) == (34)));
    let mut i: i32 = (unsafe { S::operator_int(&s) });
    assert!(((i) == (3)));
    assert!((((unsafe { S::operator_int(&s,) }) + (1)) == (4)));
    if (unsafe { S::operator__Bool(&s) }) {
        assert!((unsafe { S::operator__Bool(&s,) }));
    } else {
        assert!(false);
    }
    let mut z: S = S { v: 0 };
    assert!(!(unsafe { S::operator__Bool(&z,) }));
    let mut st: Static = <Static>::default();
    assert!(((unsafe { operator_call(6, 7,) }) == (42)));
    return 0;
}
